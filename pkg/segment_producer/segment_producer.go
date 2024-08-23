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

	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/djangolang/pkg/helpers"
)

const (
	executable      = "ffmpeg"
	watchdogTimeout = time.Second * 10
)

var (
	pattern = regexp.MustCompile(`Opening '(.*)' for writing`)
)

func getCommandLine(
	enablePassthrough bool,
	enableNvidia bool,
	netCamURL string,
	durationSeconds int,
	destinationPath string,
	cameraName string,
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
		"-rtsp_transport",
		"tcp",
		"-i",
		netCamURL,
		"-c",
		"copy",
		"-map",
		"0",
		"-f",
		"segment",
		"-segment_time",
		fmt.Sprintf("%v", durationSeconds),
		"-segment_format",
		"mp4",
		"-segment_atclocktime",
		"1",
		"-strftime",
		"1",
		"-x264-params",
		"keyint=100:scenecut=0",
		"-g",
		"100",
		"-muxdelay",
		"0",
		"-muxpreload",
		"0",
		"-reset_timestamps",
		"1",
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

	arguments = append(
		arguments,
		filepath.Join(destinationPath, "Segment_%Y-%m-%dT%H:%M:%S_"+cameraName+".mp4"),
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
					fileSize, _ := GetFileSize(thisFilePath)
					log.Printf("invoking onUpdate(%s, %f, %s)", thisFilePath, fileSize, lastLine)
					err = onUpdate(thisFilePath, fileSize, lastLine)
					if err != nil {
						readErr = fmt.Errorf("failed to invoke onUpdate(%#+v, %#+v): %v", thisFilePath, fileSize, err)
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
				fileSize, _ := GetFileSize(lastFilePath)
				log.Printf("invoking onSave(%s, %f, %s)", lastFilePath, fileSize, lastLine)
				err = onSave(lastFilePath, fileSize, lastLine)
				if err != nil {
					readErr = fmt.Errorf("failed to invoke onSave(%#+v, %#+v): %v", lastFilePath, fileSize, err)
					return
				}
			}

			fileSize, _ := GetFileSize(thisFilePath)
			log.Printf("invoking onOpen(%s, %f, %s)", thisFilePath, fileSize, lastLine)
			err = onOpen(thisFilePath, fileSize, lastLine)
			if err != nil {
				readErr = fmt.Errorf("failed to invoke onOpen(%#+v, %#+v): %v", lastFilePath, fileSize, err)
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
	netCamURL string,
	durationSeconds int,
	destinationPath string,
	cameraName string,
	onOpen func(string, float64, time.Time) error,
	onUpdate func(string, float64, time.Time) error,
	onSave func(string, float64, time.Time) error,
) error {
	arguments := getCommandLine(
		enablePassthrough,
		enableNvidia,
		netCamURL,
		durationSeconds,
		destinationPath,
		cameraName,
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

	netCamURL, err := internal.GetEnvironment[string]("NET_CAM_URL", true, nil)
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

	cameraName, err := internal.GetEnvironment[string]("CAMERA_NAME", true, nil)
	if err != nil {
		return err
	}

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	db, err := helpers.GetDBFromEnvironment(ctx)
	if err != nil {
		log.Fatalf("err: %v", err)
	}
	defer func() {
		db.Close()
	}()

	var camera *api.Camera

	err = func() error {
		tx, err := db.Begin(ctx)
		if err != nil {
			return err
		}

		defer func() {
			_ = tx.Rollback(ctx)
		}()

		camera, err = api.SelectCamera(
			ctx,
			tx,
			fmt.Sprintf(
				"%v = $$?? AND %v = $$??",
				api.CameraTableNameColumn,
				api.CameraTableStreamURLColumn,
			),
			cameraName,
			netCamURL,
		)
		if err != nil {
			return err
		}

		err = tx.Commit(ctx)
		if err != nil {
			return err
		}

		return nil
	}()
	if err != nil {
		return err
	}

	// TODO: I think this is too memory hungry, at least until I work out what's going on
	// err = func() error {
	// 	tx, err := db.Begin(ctx)
	// 	if err != nil {
	// 		return err
	// 	}

	// 	defer func() {
	// 		_ = tx.Rollback(ctx)
	// 	}()

	// 	orphanedVideos, err := api.SelectVideos(
	// 		ctx,
	// 		tx,
	// 		fmt.Sprintf(
	// 			"%v = $$?? AND %v = $$??",
	// 			api.VideoTableCameraIDColumn,
	// 			api.VideoTableStatusColumn,
	// 		),
	// 		nil,
	// 		nil,
	// 		nil,
	// 		camera.ID,
	// 		"recording",
	// 	)
	// 	if err != nil {
	// 		return err
	// 	}

	// 	for _, video := range orphanedVideos {
	// 		log.Printf("marking orphaned video %s as failed", video.ID)

	// 		filePath := filepath.Join(destinationPath, video.FileName)

	// 		_, fileName := filepath.Split(filePath)
	// 		video.FileName = fileName

	// 		ext := filepath.Ext(fileName)
	// 		thumbnailPath := fmt.Sprintf("%v.jpg", filePath[:len(filePath)-len(ext)])
	// 		err = GenerateThumbnail(filePath, thumbnailPath)
	// 		if err == nil {
	// 			_, thumbnailName := filepath.Split(thumbnailPath)
	// 			video.ThumbnailName = &thumbnailName
	// 		}

	// 		video.Status = helpers.Ptr("failed")
	// 		err = video.Update(ctx, tx, false)
	// 		if err != nil {
	// 			return err
	// 		}
	// 	}

	// 	err = tx.Commit(ctx)
	// 	if err != nil {
	// 		return err
	// 	}

	// 	return nil
	// }()
	// if err != nil {
	// 	return fmt.Errorf("failed to handle orphaned videos: %v", err)
	// }

	log.Printf("api confirmed camera %s | %s | %s", camera.ID, camera.StreamURL, camera.Name)

	mu := new(sync.Mutex)
	var video *api.Video

	onOpen := func(filePath string, fileSize float64, timestamp time.Time) error {
		mu.Lock()
		defer mu.Unlock()

		tx, err := db.Begin(ctx)
		if err != nil {
			return err
		}

		defer func() {
			_ = tx.Rollback(ctx)
		}()

		if video != nil {
			log.Printf("warning: there was a Video in-flight that should have been closed out- marking %v as failed", video.ID)

			filePath := filepath.Join(destinationPath, video.FileName)

			_, fileName := filepath.Split(filePath)
			video.FileName = fileName

			ext := filepath.Ext(fileName)
			thumbnailPath := fmt.Sprintf("%v.jpg", filePath[:len(filePath)-len(ext)])
			err = GenerateThumbnail(filePath, thumbnailPath)
			if err == nil {
				_, thumbnailName := filepath.Split(thumbnailPath)
				video.ThumbnailName = &thumbnailName
			}

			video.Status = helpers.Ptr("failed")
			err = video.Update(ctx, tx, false)
			if err != nil {
				return err
			}

			video = nil
		}

		_, fileName := filepath.Split(filePath)

		video = &api.Video{
			FileName:  fileName,
			StartedAt: internal.GetNow(),
			Status:    helpers.Ptr("recording"),
			CameraID:  camera.ID,
		}

		err = video.Insert(ctx, tx, false, false)
		if err != nil {
			return err
		}

		camera.LastSeen = helpers.Ptr(timestamp)
		err = camera.Update(ctx, tx, false)
		if err != nil {
			return err
		}

		err = tx.Commit(ctx)
		if err != nil {
			return err
		}

		return nil
	}

	onUpdate := func(filePath string, fileSize float64, timestamp time.Time) error {
		mu.Lock()
		defer mu.Unlock()

		if video == nil {
			return fmt.Errorf("assertion failed: there should be a Video in-flight that we can update in the database")
		}

		tx, err := db.Begin(ctx)
		if err != nil {
			return err
		}

		defer func() {
			_ = tx.Rollback(ctx)
		}()

		video.FileSize = helpers.Ptr(fileSize)

		duration := timestamp.Sub(video.StartedAt)
		video.Duration = &duration

		err = video.Update(ctx, tx, false)
		if err != nil {
			return err
		}

		camera.LastSeen = helpers.Ptr(timestamp)
		err = camera.Update(ctx, tx, false)
		if err != nil {
			return err
		}

		err = tx.Commit(ctx)
		if err != nil {
			return err
		}

		return nil
	}

	onSave := func(filePath string, fileSize float64, timestamp time.Time) error {
		mu.Lock()
		defer mu.Unlock()

		if video == nil {
			return fmt.Errorf("assertion failed: there should be a Video in-flight that we can update in the database")
		}

		tx, err := db.Begin(ctx)
		if err != nil {
			return err
		}

		defer func() {
			_ = tx.Rollback(ctx)
		}()

		_, fileName := filepath.Split(filePath)
		video.FileName = fileName

		ext := filepath.Ext(fileName)
		thumbnailPath := fmt.Sprintf("%v.jpg", filePath[:len(filePath)-len(ext)])
		err = GenerateThumbnail(filePath, thumbnailPath)
		if err == nil {
			_, thumbnailName := filepath.Split(thumbnailPath)
			video.ThumbnailName = &thumbnailName
		}

		video.FileSize = helpers.Ptr(fileSize)

		video.Duration = helpers.Ptr(timestamp.Sub(video.StartedAt))

		duration, err := GetVideoDuration(filePath)
		if err == nil {
			video.Duration = &duration
		}

		video.EndedAt = helpers.Ptr(timestamp)
		video.Status = helpers.Ptr("needs detection")

		err = video.Update(ctx, tx, false)
		if err != nil {
			return err
		}

		camera.LastSeen = helpers.Ptr(timestamp)
		err = camera.Update(ctx, tx, false)
		if err != nil {
			return err
		}

		err = tx.Commit(ctx)
		if err != nil {
			return err
		}

		video = nil

		return nil
	}

	err = run(
		ctx,
		cancel,
		enablePassthrough,
		enableNvidia,
		netCamURL,
		durationSeconds,
		destinationPath,
		cameraName,
		onOpen,
		onUpdate,
		onSave,
	)
	if err != nil {
		return err
	}

	return nil
}
