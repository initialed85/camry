package object_tracker

import (
	"context"
	"runtime"

	"gocv.io/x/gocv"
)

const (
	playDelayMs        = int(1.0 / 20.0 * 1_000)
	fastForwardDelayMs = 1
	slowMotionDelayMs  = int(1.0 / 5.0 * 1_000)
)

func init() {
	runtime.LockOSThread()
}

func HandleDebugWindow(ctx context.Context, cancel context.CancelFunc, mats chan gocv.Mat) error {
	defer func() {
		cancel()
	}()

	if mats == nil {
		<-ctx.Done()
		return nil
	}

	window := gocv.NewWindow("camry")
	defer func() {
		_ = window.Close()
	}()

	window.SetWindowProperty(gocv.WindowPropertyAspectRatio, gocv.WindowKeepRatio)
	window.SetWindowProperty(gocv.WindowPropertyAutosize, gocv.WindowAutosize)

	go func() {
		<-ctx.Done()
		_ = window.Close()
	}()

	delay := playDelayMs
	pause := false

	for {
		select {
		case <-ctx.Done():
			return nil
		case mat := <-mats:
			window.IMShow(mat)
			_ = mat.Close()

			for {
				select {
				case <-ctx.Done():
					return nil
				default:
				}

				key := window.WaitKey(delay)

				switch key {
				case 27: // escape
					return nil

				case 32: // space
					pause = !pause

				case 0: // up
					delay = fastForwardDelayMs

				case 1: // down
					delay = slowMotionDelayMs

				case 3: // right
					delay = playDelayMs

				default:
				}

				if pause {
					continue
				}

				break
			}
		}
	}
}
