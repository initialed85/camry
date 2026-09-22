package segment_producer

import (
	"bufio"
	"context"
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
	"github.com/initialed85/djangolang/pkg/config"
	djangolang_helpers "github.com/initialed85/djangolang/pkg/helpers"
)

const (
	executable      = "ffmpeg"
	watchdogTimeout = time.Second * 30
)

var (
	pattern = regexp.MustCompile(`Opening '(.*)' for writing`)
)

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

	fileSuffix := ""
	if isLowRes {
		fileSuffix = "_low_res"
	}

	arguments = append(
		arguments,
		filepath.Join(destinationPath, "Segment_%Y-%m-%dT%H:%M:%S_"+cameraID.String()+fileSuffix+".mp4"),
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

	db, err := config.GetDBFromEnvironment(ctx)
	if err != nil {
		return err
	}
	defer func() {
		db.Close()
	}()

	claimRefreshDuration := (time.Duration(durationSeconds) * time.Second) + (time.Second * 2)

	camera := &api.Camera{}

	for {
		err = func() error {
			tx, err := db.Begin(ctx)
			if err != nil {
				return err
			}

			defer func() {
				_ = tx.Rollback(ctx)
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
				internal.Ptr(1),
				nil,
			)
			if err != nil {
				return err
			}

			if len(cameras) != 1 {
				return fmt.Errorf("wanted exactly 1 unclaimed camera, got %d", len(cameras))
			}

			camera = cameras[0]

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

			return nil
		}()
		if err != nil {
			log.Printf("warning: %v", err)
			time.Sleep(time.Second * 1)
			continue
		}

		break
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

			filePath := filepath.Join(destinationPath, video.FileName)

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
		newRecording(ctx, db, camera, cameraMu, destinationPath, claimRefreshDuration, false),
	}
	if camera.LowResStreamURL != nil && strings.TrimSpace(*camera.LowResStreamURL) != "" {
		recordings = append(recordings, newRecording(ctx, db, camera, cameraMu, destinationPath, claimRefreshDuration, true))
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
			camera.SegmentProducerClaimedUntil = internal.GetNow()
			err = camera.UpdateFields(cleanupCtx, tx, map[string]any{
				api.CameraTableSegmentProducerClaimedUntilColumn: camera.SegmentProducerClaimedUntil,
				api.CameraTableUpdatedAtColumn:                   time.Now().UTC(),
			})
			cameraMu.Unlock()
			if err != nil {
				return err
			}

			return tx.Commit(cleanupCtx)
		}()
		if err != nil {
			log.Printf("warning: had %v on shutdown cleanup", err)
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
