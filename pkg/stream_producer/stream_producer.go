package stream_producer

import (
	"bufio"
	"context"
	"fmt"
	"io"
	"log"
	"os"
	"os/exec"
	"os/signal"
	"strings"
	"sync"
	"syscall"
	"time"

	"github.com/google/uuid"
	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/djangolang/pkg/config"
)

const (
	executable           = "ffmpeg"
	watchdogTimeout      = time.Second * 30
	claimRefreshDuration = time.Second * 90 // TODO: should probably be configurable
)

func getCommandLine(
	enablePassthrough bool,
	enableNvidia bool,
	streamURL string,
	destinationURL string,
	cameraID uuid.UUID,
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
		"-re",
		"-i", streamURL,
		"-c:v", "copy",
		"-c:a", "libopus",
		"-b:a", "64k",
		"-async", "50",
		"-f", "rtsp",
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
		fmt.Sprintf("%s/%s", destinationURL, cameraID.String()),
	)

	return arguments
}

func runCommand(
	outerCtx context.Context,
	arguments []string,
	onUpdate func(time.Time) error,
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

			if s != "" {
				if time.Since(lastUpdate) > time.Second*1 {
					log.Printf("invoking onUpdate(%s)", lastLine)
					err = onUpdate(lastLine)
					if err != nil {
						readErr = fmt.Errorf("failed to invoke onUpdate(%s): %v", lastLine, err)
						return
					}

					lastUpdate = lastLine
				}
			}

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
				out, _ := io.ReadAll(reader)
				return fmt.Errorf(
					"command %v %v failed; err: %v, out: %s",
					executable,
					strings.Join(arguments, " "),
					err,
					string(out),
				)
			}

			if readErr != nil {
				out, _ := io.ReadAll(reader)
				return fmt.Errorf(
					"command %v %v failed during read; err: %v, out: %s",
					executable,
					strings.Join(arguments, " "),
					readErr,
					string(out),
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
	destinationURL string,
	cameraID uuid.UUID,
	onUpdate func(time.Time) error,
) error {
	arguments := getCommandLine(
		enablePassthrough,
		enableNvidia,
		streamURL,
		destinationURL,
		cameraID,
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
		err := runCommand(ctx, arguments, onUpdate)

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

	destinationURL, err := internal.GetEnvironment("DESTINATION_URL", true, internal.Ptr(""), false)
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

			err = camera.AdvisoryLockWithRetries(ctx, tx, 2, claimRefreshDuration+(time.Second*2), time.Second*1)
			if err != nil {
				return err
			}

			cameras, _, _, _, _, err := api.SelectCameras(
				ctx,
				tx,
				fmt.Sprintf(
					"%v < now()",
					api.CameraTableStreamProducerClaimedUntilColumn,
				),
				internal.Ptr(fmt.Sprintf(
					"%v DESC",
					api.CameraTableStreamProducerClaimedUntilColumn,
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
			camera.SegmentProducerClaimedUntil = time.Time{}
			camera.StreamProducerClaimedUntil = now.Add(claimRefreshDuration)

			err = camera.Update(ctx, tx, false)
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

	onUpdate := func(timestamp time.Time) error {
		tx, err := db.Begin(ctx)
		if err != nil {
			return err
		}

		defer func() {
			_ = tx.Rollback(ctx)
		}()

		camera.LastSeen = timestamp
		camera.SegmentProducerClaimedUntil = time.Time{}
		camera.StreamProducerClaimedUntil = timestamp.Add(claimRefreshDuration)

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

	defer func() {
		ctx, cancel := context.WithTimeout(context.Background(), time.Second*10)
		defer cancel()

		err := func() error {
			tx, err := db.Begin(ctx)
			if err != nil {
				return err
			}

			defer func() {
				_ = tx.Rollback(ctx)
			}()

			if camera != nil {
				camera.SegmentProducerClaimedUntil = time.Time{} // zero to ensure we don't wipe out an existing value
				camera.StreamProducerClaimedUntil = internal.GetNow()

				err = camera.Update(ctx, tx, false)
				if err != nil {
					return err
				}

				log.Printf("released claim on camera %s | %s | %s", camera.ID, camera.StreamURL, camera.Name)
			}

			err = tx.Commit(ctx)
			if err != nil {
				return err
			}

			return nil
		}()
		if err != nil {
			log.Printf("warning: had %v on shutdown cleanup", err)
		}
	}()

	err = run(
		ctx,
		cancel,
		enablePassthrough,
		enableNvidia,
		camera.StreamURL,
		destinationURL,
		camera.ID,
		onUpdate,
	)
	if err != nil {
		return err
	}

	return nil
}
