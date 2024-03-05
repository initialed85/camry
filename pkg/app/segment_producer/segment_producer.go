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

func runCommand(outerCtx context.Context, arguments []string, onOpen func(string), onSave func(string)) error {
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
				onSave(lastPath)
			}

			log.Printf("invoking onOpen(%#+v)", path)
			onOpen(path)

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
	enablePassthrough bool,
	enableNvidia bool,
	netCamURL string,
	durationSeconds int,
	destinationPath string,
	cameraName string,
	onOpen func(string),
	onSave func(string),
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

	ctx, cancel := context.WithCancel(context.Background())

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

func Run() error {
	enablePassthrough, err := internal.GetEnvironment("ENABLE_PASSTHROUGH", false, internal.Ptr(true))
	if err != nil {
		return err
	}

	enableNvidia, err := internal.GetEnvironment("ENABLE_NVIDIA", false, internal.Ptr(false))
	if err != nil {
		return err
	}

	netCamURL, err := internal.GetEnvironment[string]("NET_CAM_URL", true, nil)
	if err != nil {
		return err
	}

	durationSeconds, err := internal.GetEnvironment("DURATION_SECONDS", false, internal.Ptr(300))
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

	onOpen := func(path string) {}

	onSave := func(path string) {}

	err = run(
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
