package segment_producer

import (
	"bufio"
	"bytes"
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
	watchdogTimeout = time.Second * 5
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

func runCommand(outerCtx context.Context, arguments []string, onOpen func(string) error, onSave func(string) error) error {
	ctx, cancel := context.WithCancel(outerCtx)
	defer cancel()

	cmd := exec.Command(
		executable,
		arguments...,
	)
	cleanup := func() {
		if cmd.Process != nil {
			_ = cmd.Process.Kill()
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

	reader, writer := io.Pipe()
	defer func() {
		_ = reader.Close()
		_ = writer.Close()
	}()

	mu := new(sync.Mutex)
	lastLine := time.Now()

	go func() {
		lastPath := ""

		r := bufio.NewReader(reader)
		for {
			select {
			case <-ctx.Done():
				return
			default:
			}

			b, err := r.ReadBytes(byte('\r'))
			if err != nil {
				log.Printf("warning: failed r.ReadLine(); err: %v", err)
				return
			}
			s := string(b)

			log.Printf("read line (reset watchdog): %#+v", s)

			mu.Lock()
			lastLine = time.Now()
			mu.Unlock()

			match := pattern.FindStringSubmatch(s)
			if len(match) != 2 {
				continue
			}

			path := match[1]
			log.Printf("found path: %#+v", path)

			if lastPath != "" {
				log.Printf("invoking onSave(%#+v)", lastPath)
				err = onSave(lastPath)
				if err != nil {
					log.Printf("warning: failed to invoke onSave(%#+v): %v", lastPath, err)
				}
			}

			log.Printf("invoking onOpen(%#+v)", path)
			err = onOpen(path)
			if err != nil {
				log.Printf("warning: failed to invoke onOpen(%#+v): %v", lastPath, err)
			}

			lastPath = path
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

			now := time.Now()
			watchdogAge := now.Sub(lastLine)
			if watchdogAge > watchdogTimeout {
				log.Printf("watchdog timeout at %v", watchdogAge)
				cancel()
				return
			}

			log.Printf("watchdog okay at %v", watchdogAge)
			time.Sleep(time.Second * 1)
		}
	}()

	err := cmd.Run()
	if err != nil {
		return fmt.Errorf(
			"command %v %v failed; err: %v",
			executable,
			strings.Join(arguments, " "),
			err,
		)
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
	onOpen func(string) error,
	onSave func(string) error,
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
		err := runCommand(ctx, arguments, onOpen, onSave)

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

func getThumbail(videoPath, imagePath string) error {
	arguments := []string{
		"-i",
		videoPath,
		"-ss",
		"00:00:00.000",
		"-vframes",
		"1",
		imagePath,
	}

	cmd := exec.Command(
		"ffmpeg",
		arguments...,
	)

	stdout := new(bytes.Buffer)
	stderr := new(bytes.Buffer)

	cmd.Stdout = stdout
	cmd.Stderr = stderr

	err := cmd.Run()
	if err != nil {
		return fmt.Errorf("%v; stdout=%#+v, stderr=%#+v", err, stdout, stderr)
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
		_ = db.Close()
	}()

	var camera *api.Camera

	err = func() error {
		tx, err := db.BeginTxx(ctx, nil)
		if err != nil {
			return err
		}

		defer func() {
			_ = tx.Rollback()
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

		err = tx.Commit()
		if err != nil {
			return err
		}

		return nil
	}()
	if err != nil {
		return err
	}

	log.Printf("api confirmed %#+v", camera)

	mu := new(sync.Mutex)
	var video *api.Video

	onOpen := func(path string) error {
		mu.Lock()
		defer mu.Unlock()

		tx, err := db.BeginTxx(ctx, nil)
		if err != nil {
			return err
		}

		defer func() {
			_ = tx.Rollback()
		}()

		if video != nil && video.Status != nil && *video.Status == "recording" {
			video.Status = helpers.Ptr("failed")
			err = video.Update(ctx, tx, false)
			if err != nil {
				return err
			}
		}

		_, fileName := filepath.Split(path)

		video = &api.Video{
			FilePath:  fileName,
			StartedAt: time.Now(),
			Status:    helpers.Ptr("recording"),
			CameraID:  camera.ID,
		}

		err = video.Insert(ctx, tx, false, false)
		if err != nil {
			return err
		}

		camera.LastSeen = helpers.Ptr(time.Now())
		err = camera.Update(ctx, tx, false)
		if err != nil {
			return err
		}

		err = tx.Commit()
		if err != nil {
			return err
		}

		return nil
	}

	onSave := func(path string) error {
		mu.Lock()
		defer mu.Unlock()

		tx, err := db.BeginTxx(ctx, nil)
		if err != nil {
			return err
		}

		defer func() {
			_ = tx.Rollback()
		}()

		_, fileName := filepath.Split(path)
		ext := filepath.Ext(fileName)
		thumbnailPath := fmt.Sprintf("%v.jpg", path[:len(path)-len(ext)])

		err = getThumbail(path, thumbnailPath)
		if err != nil {
			return err
		}

		_, thumbnailName := filepath.Split(thumbnailPath)

		if video == nil {
			return fmt.Errorf("assertion failed: there should be a Video in-flight that we can finalize with the database")
		}

		video.EndedAt = helpers.Ptr(time.Now())
		video.Duration = helpers.Ptr(video.EndedAt.Sub(video.StartedAt))
		video.ThumbnailPath = &thumbnailName
		video.Status = helpers.Ptr("needs detection")

		err = video.Update(ctx, tx, false)
		if err != nil {
			return err
		}

		camera.LastSeen = helpers.Ptr(time.Now())
		err = camera.Update(ctx, tx, false)
		if err != nil {
			return err
		}

		err = tx.Commit()
		if err != nil {
			return err
		}

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
		onSave,
	)
	if err != nil {
		return err
	}

	return nil
}
