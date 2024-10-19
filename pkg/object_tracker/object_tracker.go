package object_tracker

import (
	"context"
	"encoding/json"
	"fmt"
	"image"
	"image/color"
	"io"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/pkg/api"
	"github.com/jackc/pgx/v5/pgtype"
	ffmpeg "github.com/u2takey/ffmpeg-go"
	"gocv.io/x/gocv"
)

const (
	objectDetectionStrideFrames = 4
	boundingBoxHoldDuration     = time.Millisecond * 333
	centroidHoldDuration        = time.Millisecond * 5_000
)

var filePathPrefix string

func init() {
	var err error

	filePathPrefix, err = internal.GetEnvironment("FILE_PATH_PREFIX", false, internal.Ptr(""), false)
	if err != nil {
		panic(err)
	}
}

type FFProbeOutput struct {
	Streams []struct {
		Width       int    `json:"width"`
		Height      int    `json:"height"`
		RawFPS      string `json:"r_frame_rate"`
		RawFrames   string `json:"nb_frames"`
		RawDuration string `json:"duration"`
	} `json:"streams"`
}

type Detection struct {
	ID             uuid.UUID     `json:"id"`
	CreatedAt      time.Time     `json:"created_at"`
	UpdatedAt      time.Time     `json:"updated_at"`
	DeletedAt      *time.Time    `json:"deleted_at"`
	SeenAt         time.Time     `json:"seen_at"`
	ClassID        int64         `json:"class_id"`
	ClassName      string        `json:"class_name"`
	Score          float64       `json:"score"`
	Centroid       pgtype.Vec2   `json:"centroid"`
	BoundingBox    []pgtype.Vec2 `json:"bounding_box"`
	VideoID        uuid.UUID     `json:"video_id"`
	VideoIDObject  *api.Video    `json:"video_id_object"`
	CameraID       uuid.UUID     `json:"camera_id"`
	CameraIDObject *api.Camera   `json:"camera_id_object"`
	IDWithinFrame  int64         `json:"id_within_frame"`
	IDWithinVideo  int64         `json:"id_within_video"`
	IsIntersecting bool          `json:"is_overlapping"`
}

func Track(ctx context.Context, video *api.Video, matsVariadic ...chan gocv.Mat) error {
	var mats chan gocv.Mat
	if len(matsVariadic) > 0 && matsVariadic[0] != nil {
		mats = matsVariadic[0]
	}

	if mats != nil {
		log.Printf("will be showing debug window")
	} else {
		log.Printf("will not be showing debug window")
	}

	adjustedFilePath := filepath.Join(filePathPrefix, video.FileName)
	log.Printf("adjustedFilePath: %s", adjustedFilePath)

	stat, err := os.Stat(adjustedFilePath)
	if err != nil {
		return err
	}

	if stat.IsDir() {
		return fmt.Errorf("expected %s to be a file but it was a folder", adjustedFilePath)
	}

	rawData, err := ffmpeg.Probe(adjustedFilePath)
	if err != nil {
		return fmt.Errorf("failed to probe %#+v for video.ID: %v: %v", adjustedFilePath, video.ID, err)
	}

	data := []byte(rawData)
	var ffProbeOutput FFProbeOutput
	err = json.Unmarshal(data, &ffProbeOutput)
	if err != nil {
		return fmt.Errorf("failed to parse ffprobe output %v for video.ID: %v: %v", string(data), video.ID, err)
	}

	width := int(ffProbeOutput.Streams[0].Width)
	height := int(ffProbeOutput.Streams[0].Height)

	fps, err := strconv.ParseFloat(strings.Split(ffProbeOutput.Streams[0].RawFPS, "/")[0], 64)
	if err != nil {
		return fmt.Errorf("failed to parse left portion of %#+v as float", ffProbeOutput.Streams[0].RawFPS)
	}

	frames, err := strconv.ParseInt(strings.Split(ffProbeOutput.Streams[0].RawFrames, "/")[0], 10, 64)
	if err != nil {
		return fmt.Errorf("failed to parse left portion of %#+v as int", ffProbeOutput.Streams[0].RawFrames)
	}

	log.Printf("video.ID: %v; video is %v frames of %v * %v @ %v FPS for %v",
		video.ID,
		frames,
		width,
		height,
		fps,
		video.Duration,
	)

	detectionsByFrame := make(map[int64][]Detection)
	for _, detection := range video.ReferencedByDetectionVideoIDObjects {
		detection := Detection{
			ID:             detection.ID,
			CreatedAt:      detection.CreatedAt,
			UpdatedAt:      detection.UpdatedAt,
			DeletedAt:      detection.DeletedAt,
			SeenAt:         detection.SeenAt,
			ClassID:        detection.ClassID,
			ClassName:      detection.ClassName,
			Score:          detection.Score,
			Centroid:       detection.Centroid,
			BoundingBox:    detection.BoundingBox,
			VideoID:        detection.VideoID,
			VideoIDObject:  detection.VideoIDObject,
			CameraID:       detection.CameraID,
			CameraIDObject: detection.CameraIDObject,
			IDWithinFrame:  -1,
			IDWithinVideo:  -1,
		}

		frame := int64(detection.SeenAt.Sub(video.StartedAt).Seconds() * fps)

		detectionsForFrame, ok := detectionsByFrame[frame]
		if !ok {
			detectionsForFrame = make([]Detection, 0)
		}

		detectionsForFrame = append(detectionsForFrame, detection)

		detectionsByFrame[frame] = detectionsForFrame
	}

	for frame, detectionsForFrame := range detectionsByFrame {
		for i, detection := range detectionsForFrame {
			detection.IDWithinFrame = int64(i)
			detectionsByFrame[frame][i] = detection
		}
	}

	boundingBoxesByFrame := make(map[int64][]BoundingBox)

	var lastDetectionsForFrame []Detection
	for frame, detectionsForFrame := range detectionsByFrame {
		for _, a := range detectionsForFrame {
			aTopLeft := a.BoundingBox[0]
			aBottomRight := a.BoundingBox[2]
			bbA := BoundingBox{TL: aTopLeft, BR: aBottomRight, Color: color.RGBA{255, 0, 0, 0}, IDWithinFrame: a.IDWithinFrame}

			boundingBoxesForFrame := boundingBoxesByFrame[frame]
			if boundingBoxesForFrame == nil {
				boundingBoxesForFrame = make([]BoundingBox, 0)
			}

			boundingBoxesForFrame = append(boundingBoxesForFrame, bbA)

			for _, b := range lastDetectionsForFrame {
				bTopLeft := b.BoundingBox[0]
				bBottomRight := b.BoundingBox[2]
				bbB := BoundingBox{TL: bTopLeft, BR: bBottomRight}

				bbI := GetIntersection2D(bbA, bbB)
				if bbI == nil {
					continue
				}

				bbI.Color = color.RGBA{255, 255, 255, 0}
				bbI.IDWithinFrame = -1

				log.Printf("bbA: %v, %v", bbA.TL, bbA.BR)
				log.Printf("bbB: %v, %v", bbB.TL, bbB.BR)
				log.Printf("bbI: %v, %v\n", bbI.TL, bbI.BR)

				boundingBoxesForFrame = append(boundingBoxesForFrame, *bbI)
			}

			boundingBoxesByFrame[frame] = boundingBoxesForFrame
		}

		detectionsByFrame[frame] = detectionsForFrame

		lastDetectionsForFrame = detectionsForFrame
	}

	reader, writer := io.Pipe()
	defer func() {
		_ = reader.Close()
		_ = writer.Close()
	}()

	go func() {
		<-ctx.Done()
		_ = reader.Close()
		_ = writer.Close()
	}()

	errs := make(chan error, 16)

	ffmpegCtx, ffmpegCancel := context.WithCancel(ctx)

	go func() {
		defer ffmpegCancel()

		err := ffmpeg.Input(adjustedFilePath).
			Output("pipe:",
				ffmpeg.KwArgs{
					"format":  "rawvideo",
					"pix_fmt": "rgb24",
				}).
			WithOutput(writer).
			// ErrorToStdOut().
			Run()
		if err != nil {
			errs <- err
		}
	}()

	select {
	case <-time.After(time.Second * 1):
	case err := <-errs:
		return fmt.Errorf("failed to open ffmpeg input stream for video.ID: %v: %v", video.ID, err)
	}

	boundingBoxHoldFrames := int64(boundingBoxHoldDuration.Seconds() * fps)
	boundingBoxHoldFramesColorStep := uint8(255 / boundingBoxHoldFrames)
	centroidHoldFrames := int64(centroidHoldDuration.Seconds() * fps)
	centroidHoldFramesColorStep := uint8(255 / centroidHoldFrames)
	charSize := gocv.GetTextSize("a", gocv.FontHersheyPlain, 1.0, 1)

	_ = boundingBoxHoldFrames
	_ = boundingBoxHoldFramesColorStep
	_ = centroidHoldFrames
	_ = centroidHoldFramesColorStep
	_ = charSize

	frameSize := width * height * 3
	buf := make([]byte, frameSize)
	frame := int64(0)

	for {
		select {
		case <-ffmpegCtx.Done():
			log.Printf("finished; ffmpegCtx cancelled")
			return nil
		default:
		}

		n, err := io.ReadFull(reader, buf)
		if n != frameSize || (err != nil && err != io.EOF) {
			return fmt.Errorf("failed to read %#+v after %v bytes for video.ID: %v: %v", adjustedFilePath, n, video.ID, err)
		}

		if n == 0 || err == io.EOF {
			log.Printf("finished; n: %v, err: %v", n, err)
			break
		}

		originalImage, err := gocv.NewMatFromBytes(height, width, gocv.MatTypeCV8UC3, buf)
		if err != nil {
			return fmt.Errorf("failed to decode %#+v bytes: %v", n, err)
		}

		shouldBreak, shouldContinue, err := func() (bool, bool, error) {
			defer func() {
				_ = originalImage.Close()
			}()

			if originalImage.Empty() {
				log.Printf("finished; originalImage was empty")
				return true, false, nil
			}

			frame++

			overlay := gocv.NewMat()
			gocv.CvtColor(originalImage, &overlay, gocv.ColorBGRToRGBA)

			frameDurationSeconds := float64(frame) / fps
			frameDuration := time.Nanosecond * time.Duration(frameDurationSeconds*1000000000)
			frameTimestamp := video.StartedAt.Add(frameDuration)
			_ = frameTimestamp

			for i := int64(0); i < boundingBoxHoldFrames; i++ {
				currentFrame := frame - i

				for _, bb := range boundingBoxesByFrame[currentFrame] {
					boundingBoxRect := image.Rectangle{
						Min: image.Point{
							X: int(bb.TL.X),
							Y: int(bb.TL.Y),
						},
						Max: image.Point{
							X: int(bb.BR.X),
							Y: int(bb.BR.Y),
						},
					}

					boundingBoxColor := bb.Color

					if bb.IDWithinFrame != -1 {
						if bb.IDWithinFrame == 0 {
							boundingBoxColor = color.RGBA{
								255,
								0,
								0,
								0,
							}
						} else {
							boundingBoxColor = color.RGBA{
								0,
								0,
								255,
								0,
							}
						}
					}

					gocv.Rectangle(&overlay, boundingBoxRect, boundingBoxColor, 1)
				}

				// for _, detection := range detectionsByFrame[currentFrame] {
				// 	bottomLeft := detection.BoundingBox[3]

				// 	textColor := color.RGBA{
				// 		255,
				// 		255,
				// 		255,
				// 		0,
				// 	}

				// 	text1 := fmt.Sprintf("%.2f%% | %d | %s", detection.Score*100.0, detection.IDWithinFrame, detection.ClassName)
				// 	// text2 := fmt.Sprintf("%.2fw x %.2fh", detection.Width, detection.Height)
				// 	// text3 := fmt.Sprintf("%.2f | %.2f", detection.Area, detection.AspectRatio)

				// 	textPoint1 := image.Pt(int(bottomLeft.X), int(bottomLeft.Y)+(charSize.Y*0))
				// 	// textPoint2 := image.Pt(int(bottomLeft.X), int(bottomLeft.Y)+(charSize.Y*2))
				// 	// textPoint3 := image.Pt(int(bottomLeft.X), int(bottomLeft.Y)+(charSize.Y*4))

				// 	gocv.PutText(&overlay, text1, textPoint1, gocv.FontHersheyPlain, 1.0, textColor, 1)
				// 	// gocv.PutText(&overlay, text2, textPoint2, gocv.FontHersheyPlain, 1.0, textColor, 1)
				// 	// gocv.PutText(&overlay, text3, textPoint3, gocv.FontHersheyPlain, 1.0, textColor, 1)
				// }
			}

			// for i := int64(0); i < centroidHoldFrames; i++ {
			// 	currentFrame := frame - i

			// 	for _, detection := range detectionsByFrame[currentFrame] {
			// 		centroidPoint := image.Point{int(detection.Centroid.X), int(detection.Centroid.Y)}

			// 		centroidColor := color.RGBA{
			// 			255 - (centroidHoldFramesColorStep * uint8(i)),
			// 			255 - (centroidHoldFramesColorStep * uint8(i)),
			// 			255 - (centroidHoldFramesColorStep * uint8(i)),
			// 			0,
			// 		}

			// 		gocv.Circle(&overlay, centroidPoint, 4, centroidColor, 2)
			// 	}
			// }

			if mats != nil {
				mats <- overlay
			}

			return false, false, nil
		}()
		if err != nil {
			return err
		}

		if shouldBreak {
			break
		}

		if shouldContinue {
			continue
		}

		_ = shouldBreak
		_ = shouldContinue
	}

	return nil
}
