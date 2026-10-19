package segment_producer

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"io"
	"log"
	"os"
	"os/exec"
	"os/signal"
	"path/filepath"
	"regexp"
	"strings"
	"sync"
	"syscall"
	"time"

	"github.com/google/uuid"
	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/camry/pkg/helpers"

	djangolang_helpers "github.com/initialed85/djangolang/pkg/helpers"
)

const (
	executable      = "ffmpeg"
	watchdogTimeout = time.Second * 30

	// ffmpeg writes each segment under this suffix and the producer renames it away when the
	// segment is closed, so a segment is only ever visible at its final name once it is complete.
	tempFileSuffix = helpers.TempFileSuffix
)

var (
	pattern = regexp.MustCompile(`Opening '(.*)' for writing`)
)

// finalPathFor maps an ffmpeg temp path to the path the finished segment lives at.
func finalPathFor(filePath string) string {
	return strings.TrimSuffix(filePath, tempFileSuffix)
}

// finalizeSegment moves a closed segment to its final name and returns it. ffmpeg's own rename
// never happens, so this is the only point at which a segment becomes visible at its final name.
func finalizeSegment(filePath string) (string, error) {
	finalPath := finalPathFor(filePath)

	if finalPath == filePath {
		return finalPath, nil
	}

	if _, err := os.Stat(finalPath); err == nil {
		return "", fmt.Errorf("refusing to overwrite existing %v", finalPath)
	}

	if err := os.Rename(filePath, finalPath); err != nil {
		return "", err
	}

	return finalPath, nil
}

// recoveredPath returns filePath, moving a dead producer's leftover temp segment into place when
// that is the only copy there is. ffmpeg writes a moov atom only at close, so a leftover temp file
// has no index and its thumbnail fails; the row ends up as a failed one with no thumbnail.
func recoveredPath(filePath string) string {
	if _, err := os.Stat(filePath); err == nil {
		return filePath
	}

	finalPath, err := finalizeSegment(filePath + tempFileSuffix)
	if err != nil {
		return filePath
	}

	log.Printf("recovered abandoned segment %v", finalPath)

	return finalPath
}

func getCommandLine(
	enablePassthrough bool,
	enableNvidia bool,
	streamURL string,
	durationSeconds int,
	destinationPath string,
	cameraID uuid.UUID,
	isLowRes bool,
) []string {
	arguments := make([]string, 0)

	if !enablePassthrough {
		if !enableNvidia {
			arguments = append(
				arguments,
				"-hwaccel",
				"cuda",
				"-c:v",
				"h264_cuvid",
			)
		} else {
			arguments = append(
				arguments,
				"-c:v",
				"h264",
			)
		}
	}

	arguments = append(
		arguments,
		"-rtsp_transport", "tcp",
		"-i", streamURL,
		"-c", "copy",
		"-map", "0",
		"-f", "segment",
		"-segment_time", fmt.Sprintf("%v", durationSeconds),
		"-segment_format", "mp4",
		"-segment_atclocktime", "1",
		"-strftime", "1",
		"-x264-params", "keyint=100:scenecut=0",
		"-g", "100",
		"-muxdelay", "0",
		"-muxpreload", "0",
		"-reset_timestamps", "1",
		// moov before mdat: a browser can start playing a segment as it arrives instead of
		// downloading all ~113MB of it first. It has to be handed to the mp4 muxer per segment -
		// `-movflags +faststart` as an output option is accepted by `-f segment` and silently
		// ignored (measured: moov at the tail), and `-segment_format_options` is what reaches each
		// segment's own muxer (measured: moov at 32, mdat after it).
		"-segment_format_options", "movflags=+faststart",
	)

	if !enablePassthrough {
		if enableNvidia {
			arguments = append(
				arguments,
				"-c:v",
				"h264_nvenc",
			)
		} else {
			arguments = append(
				arguments,
				"-c:v",
				"libx264",
			)
		}
	}

	// -y is safe here: the pattern is this producer's private temp name, fenced by claimFence, so
	// the only writer that can be sitting on it is us.
	arguments = append(arguments, "-y")

	fileSuffix := ""
	if isLowRes {
		fileSuffix = "_low_res"
	}

	arguments = append(
		arguments,
		filepath.Join(destinationPath, "Segment_%Y-%m-%dT%H:%M:%S_"+cameraID.String()+fileSuffix+".mp4"+tempFileSuffix),
	)

	return arguments
}

func runCommand(
	outerCtx context.Context,
	arguments []string,
	onOpen func(string, float64, time.Time) error,
	onUpdate func(string, float64, time.Time) error,
	onSave func(string, float64, time.Time) error,
) error {
	ctx, cancel := context.WithCancel(outerCtx)
	defer cancel()

	reader, writer := io.Pipe()
	defer func() {
		_ = reader.Close()
		_ = writer.Close()
	}()

	cmd := exec.Command(
		executable,
		arguments...,
	)
	cleanup := func() {
		if cmd.Process != nil {
			_ = cmd.Process.Signal(syscall.SIGINT)
			time.Sleep(time.Second * 1)
			go func() {
				time.Sleep(time.Second * 4)
				_ = cmd.Process.Signal(syscall.SIGTERM)
			}()
		}
	}
	defer func() {
		cleanup()
	}()
	go func() {
		<-ctx.Done()
		log.Printf("stopping command...")
		cleanup()
	}()

	mu := new(sync.Mutex)
	lastLine := internal.GetNow()
	lastUpdate := internal.GetNow()

	var readErr error

	go func() {
		thisFilePath := ""
		lastFilePath := ""

		r := bufio.NewReader(reader)
		for {
			select {
			case <-ctx.Done():
				return
			default:
			}

			b, err := r.ReadBytes(byte('\r'))
			if err != nil {
				readErr = err
				return
			}

			s := string(b)

			// TODO: handy for debugging
			// log.Printf("read line (reset watchdog): %#+v", s)

			mu.Lock()
			lastLine = internal.GetNow()
			mu.Unlock()

			if thisFilePath != "" {
				if time.Since(lastUpdate) > time.Second*1 {
					fileSize, _ := helpers.GetFileSize(thisFilePath)
					log.Printf("invoking onUpdate(%s, %f, %s)", thisFilePath, fileSize, lastLine)
					err = onUpdate(thisFilePath, fileSize, lastLine)
					if err != nil {
						readErr = fmt.Errorf("failed to invoke onUpdate(%s, %f, %s): %v", thisFilePath, fileSize, lastLine, err)
						return
					}

					lastUpdate = lastLine
				}
			}

			match := pattern.FindStringSubmatch(s)
			if len(match) != 2 {
				continue
			}

			thisFilePath = strings.TrimSpace(match[1])
			if thisFilePath == "" {
				continue
			}

			// TODO: handy for debugging
			// log.Printf("found path: %#+v", thisFilePath)

			if lastFilePath != "" {
				fileSize, _ := helpers.GetFileSize(lastFilePath)
				log.Printf("invoking onSave(%s, %f, %s)", lastFilePath, fileSize, lastLine)
				err = onSave(lastFilePath, fileSize, lastLine)
				if err != nil {
					readErr = fmt.Errorf("failed to invoke onSave(%s, %f, %s): %v", lastFilePath, fileSize, lastLine, err)
					return
				}
			}

			fileSize, _ := helpers.GetFileSize(thisFilePath)
			log.Printf("invoking onOpen(%s, %f, %s)", thisFilePath, fileSize, lastLine)
			err = onOpen(thisFilePath, fileSize, lastLine)
			if err != nil {
				readErr = fmt.Errorf("failed to invoke onOpen(%s, %f, %s): %v", lastFilePath, fileSize, lastLine, err)
				return
			}

			lastFilePath = thisFilePath
		}
	}()

	cmd.Stdout = writer
	cmd.Stderr = writer

	go func() {
		for {
			select {
			case <-ctx.Done():
				return
			default:
			}

			now := internal.GetNow()
			watchdogAge := now.Sub(lastLine)
			if watchdogAge > watchdogTimeout {
				log.Printf("watchdog timeout at %v", watchdogAge)
				cancel()
				return
			}

			time.Sleep(time.Second * 1)
		}
	}()

	errs := make(chan error, 1)

	go func() {
		errs <- func() error {
			err := cmd.Run()
			if err != nil {
				return fmt.Errorf(
					"command %v %v failed; err: %v",
					executable,
					strings.Join(arguments, " "),
					err,
				)
			}

			if readErr != nil {
				return fmt.Errorf(
					"command %v %v failed during read; err: %v",
					executable,
					strings.Join(arguments, " "),
					readErr,
				)
			}
			return nil
		}()
	}()

	select {
	case <-ctx.Done():
		break
	case err := <-errs:
		if err != nil {
			return err
		}
	}

	return nil
}

func run(
	ctx context.Context,
	cancel context.CancelFunc,
	enablePassthrough bool,
	enableNvidia bool,
	streamURL string,
	durationSeconds int,
	destinationPath string,
	cameraID uuid.UUID,
	isLowRes bool,
	onOpen func(string, float64, time.Time) error,
	onUpdate func(string, float64, time.Time) error,
	onSave func(string, float64, time.Time) error,
) error {
	arguments := getCommandLine(
		enablePassthrough,
		enableNvidia,
		streamURL,
		durationSeconds,
		destinationPath,
		cameraID,
		isLowRes,
	)

	signals := make(chan os.Signal, 16)
	signal.Notify(signals, os.Interrupt, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		<-signals
		cancel()
		log.Printf("shutting down...")
	}()

retry:
	for {
		err := runCommand(ctx, arguments, onOpen, onUpdate, onSave)

		select {
		case <-ctx.Done():
			break retry
		default:
		}

		if err != nil {
			if errors.Is(err, errClaimLost) {
				log.Printf("claim on camera %s is gone; not writing any more segments", cameraID)
				return err
			}

			log.Printf("warning: runCommand failed; err: %v- retrying...", err)
		}

		time.Sleep(time.Second * 1)
	}

	return nil
}

func Run() error {
	enablePassthrough, err := internal.GetEnvironment("ENABLE_PASSTHROUGH", false, internal.Ptr(true), true)
	if err != nil {
		return err
	}

	enableNvidia, err := internal.GetEnvironment("ENABLE_NVIDIA", false, internal.Ptr(false), true)
	if err != nil {
		return err
	}

	durationSeconds, err := internal.GetEnvironment("DURATION_SECONDS", false, internal.Ptr(60))
	if err != nil {
		return err
	}

	destinationPath, err := internal.GetEnvironment("DESTINATION_PATH", false, internal.Ptr("./"))
	if err != nil {
		return err
	}

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	db, err := internal.GetDBFromEnvironment(ctx)
	if err != nil {
		return err
	}
	defer func() {
		db.Close()
	}()

	claimRefreshDuration := (time.Duration(durationSeconds) * time.Second) + (time.Second * 2)

	camera := &api.Camera{}
	var fence *claimFence

	for {
		err = func() error {
			tx, err := db.Begin(ctx)
			if err != nil {
				return err
			}

			defer func() {
				_ = tx.Rollback(ctx)
			}()

			// A fence taken here is only ours once the claim below commits; before that it has to
			// go back, or we would sit on a camera we are not writing.
			claimed := false
			defer func() {
				if !claimed && fence != nil {
					_ = fence.release(context.Background())
					fence = nil
				}
			}()

			log.Printf("waiting to lock for claiming a camera...")

			err = camera.AdvisoryLockWithRetries(ctx, tx, 1, claimRefreshDuration+(time.Second*2), time.Second*1)
			if err != nil {
				return err
			}

			cameras, _, _, _, _, err := api.SelectCameras(
				ctx,
				tx,
				fmt.Sprintf(
					"%v < now()",
					api.CameraTableSegmentProducerClaimedUntilColumn,
				),
				internal.Ptr(fmt.Sprintf(
					"%v DESC",
					api.CameraTableSegmentProducerClaimedUntilColumn,
				)),
				internal.Ptr(10),
				nil,
			)
			if err != nil {
				return err
			}

			if len(cameras) == 0 {
				return fmt.Errorf("found no unclaimed cameras")
			}

			for _, candidate := range cameras {
				candidateFence, err := newClaimFence(ctx, db, candidate.ID)
				if err != nil {
					return err
				}

				locked, err := candidateFence.take(ctx)
				if err != nil {
					_ = candidateFence.release(ctx)
					return err
				}

				if locked {
					camera, fence = candidate, candidateFence
					break
				}

				// The lease says this camera is free, but a producer that is still writing it holds
				// the fence, so take the next camera instead of sharing files with it.
				_ = candidateFence.release(ctx)
			}

			if fence == nil {
				return fmt.Errorf("all %d unclaimed cameras are being written by live producers", len(cameras))
			}

			log.Printf("found most recently unclaimed camera %s | %s | %s", camera.ID, camera.StreamURL, camera.Name)

			now := internal.GetNow()
			camera.LastSeen = now
			camera.SegmentProducerClaimedUntil = now.Add(claimRefreshDuration)

			err = camera.UpdateFields(ctx, tx, map[string]any{
				api.CameraTableLastSeenColumn:                    camera.LastSeen,
				api.CameraTableSegmentProducerClaimedUntilColumn: camera.SegmentProducerClaimedUntil,
				api.CameraTableUpdatedAtColumn:                   time.Now().UTC(),
			})
			if err != nil {
				return err
			}

			log.Printf("acquired claim on camera %s | %s | %s", camera.ID, camera.StreamURL, camera.Name)

			err = tx.Commit(ctx)
			if err != nil {
				return err
			}

			claimed = true

			return nil
		}()
		if err != nil {
			log.Printf("warning: %v", err)
			time.Sleep(time.Second * 1)
			continue
		}

		break
	}

	stale, err := helpers.CleanupStaleTempFiles(destinationPath, camera.ID.String(), time.Duration(3*durationSeconds)*time.Second)
	if err != nil {
		log.Printf("warning: cleaning stale temp files in %v: %v", destinationPath, err)
	}
	if stale > 0 {
		log.Printf("cleaned %v stale temp segments in %v", stale, destinationPath)
	}

	err = func() error {
		tx, err := db.Begin(ctx)
		if err != nil {
			return err
		}

		defer func() {
			_ = tx.Rollback(ctx)
		}()

		orphanedVideos, _, _, _, _, err := api.SelectVideos(
			ctx,
			tx,
			fmt.Sprintf(
				"%v = $$?? AND %v = $$??",
				api.VideoTableCameraIDColumn,
				api.VideoTableStatusColumn,
			),
			nil,
			nil,
			nil,
			camera.ID,
			"recording",
		)
		if err != nil {
			return err
		}

		for _, video := range orphanedVideos {
			log.Printf("marking orphaned video %s as failed", video.ID)

			filePath := recoveredPath(filepath.Join(destinationPath, video.FileName))

			_, fileName := filepath.Split(filePath)
			video.FileName = fileName

			ext := filepath.Ext(fileName)
			thumbnailPath := fmt.Sprintf("%v.jpg", filePath[:len(filePath)-len(ext)])
			err = helpers.GenerateThumbnail(filePath, thumbnailPath)
			if err == nil {
				_, thumbnailName := filepath.Split(thumbnailPath)
				video.ThumbnailName = &thumbnailName
			}

			video.Status = djangolang_helpers.Ptr("failed")
			err = video.Update(ctx, tx, false)
			if err != nil {
				return err
			}
		}

		err = tx.Commit(ctx)
		if err != nil {
			return err
		}

		return nil
	}()
	if err != nil {
		return fmt.Errorf("failed to handle orphaned videos: %v", err)
	}

	cameraMu := new(sync.Mutex)
	recordings := []*recording{
		newRecording(ctx, db, camera, cameraMu, destinationPath, claimRefreshDuration, fence, false),
	}
	if camera.LowResStreamURL != nil && strings.TrimSpace(*camera.LowResStreamURL) != "" {
		recordings = append(recordings, newRecording(ctx, db, camera, cameraMu, destinationPath, claimRefreshDuration, fence, true))
	}

	defer func() {
		cleanupCtx, cleanupCancel := context.WithTimeout(context.Background(), time.Second*10)
		defer cleanupCancel()

		err := func() error {
			tx, err := db.Begin(cleanupCtx)
			if err != nil {
				return err
			}
			defer func() { _ = tx.Rollback(cleanupCtx) }()

			for _, thisRecording := range recordings {
				if err := thisRecording.cleanup(cleanupCtx, tx); err != nil {
					return err
				}
			}

			cameraMu.Lock()
			owned, err := fence.owns(cleanupCtx)
			if err != nil {
				log.Printf("warning: checking ownership of camera %s on shutdown: %v", camera.ID, err)
			}
			if owned {
				camera.SegmentProducerClaimedUntil = internal.GetNow()
				err = camera.UpdateFields(cleanupCtx, tx, map[string]any{
					api.CameraTableSegmentProducerClaimedUntilColumn: camera.SegmentProducerClaimedUntil,
					api.CameraTableUpdatedAtColumn:                   time.Now().UTC(),
				})
				if err != nil {
					cameraMu.Unlock()
					return err
				}
			} else {
				// Some live producer owns this camera and has stamped its own lease; ours is not
				// ours to clear any more.
				log.Printf("not releasing the claim on camera %s; we no longer own it", camera.ID)
			}
			cameraMu.Unlock()

			return tx.Commit(cleanupCtx)
		}()
		if err != nil {
			log.Printf("warning: had %v on shutdown cleanup", err)
		}

		// Last, so no one can take the camera while we are still writing.
		if err := fence.release(context.Background()); err != nil {
			log.Printf("warning: releasing the fence on camera %s: %v", camera.ID, err)
		}
	}()

	errCh := make(chan error, len(recordings))
	var wg sync.WaitGroup
	for _, thisRecording := range recordings {
		thisRecording := thisRecording
		streamURL := camera.StreamURL
		if thisRecording.isLowRes {
			streamURL = *camera.LowResStreamURL
		}

		wg.Add(1)
		go func() {
			defer wg.Done()
			err := run(
				ctx,
				cancel,
				enablePassthrough,
				enableNvidia,
				streamURL,
				durationSeconds,
				destinationPath,
				camera.ID,
				thisRecording.isLowRes,
				thisRecording.onOpen,
				thisRecording.onUpdate,
				thisRecording.onSave,
			)
			if err != nil {
				errCh <- err
				cancel()
			}
		}()
	}

	wg.Wait()
	select {
	case err := <-errCh:
		return err
	default:
		return nil
	}
}
