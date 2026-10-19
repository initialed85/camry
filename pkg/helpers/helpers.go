package helpers

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"time"

	"github.com/alfg/mp4"
)

// TempFileSuffix is the suffix a segment carries while it is being written. Nothing outside the
// segment producer ever sees a file at this name.
const TempFileSuffix = ".tmp"

func GenerateThumbnail(videoPath, imagePath string) error {
	arguments := []string{
		"-i",
		videoPath,
		"-ss",
		"00:00:00.000",
		"-frames:v",
		"1",
		imagePath,
	}

	// TODO(initialed85): software is hard, overwrite files if they already exist /shrug
	arguments = append(arguments, "-y")

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
		return fmt.Errorf("failed to generate thumbnail: %v; stdout=%#+v, stderr=%#+v", err, stdout.String(), stderr.String())
	}

	return nil
}

func GetVideoDuration(path string) (time.Duration, error) {
	video, err := mp4.Open(path)
	if err != nil {
		return time.Duration(0), fmt.Errorf("failed to get video duration: %v", err)
	}

	if video.Moov == nil || video.Moov.Mvhd == nil {
		return time.Duration(0), fmt.Errorf("can't get duration- video.Moov or video.Moov.Mvhd is nil")
	}

	return time.Millisecond * time.Duration(video.Moov.Mvhd.Duration), nil
}

func GetFileSize(path string) (float64, error) {
	fileInfo, err := os.Lstat(path)
	if err != nil {
		return 0, fmt.Errorf("failed to get file size: %v", err)
	}

	if fileInfo.IsDir() {
		return 0, fmt.Errorf("failed to get file size: %#+v is a folder, not a file", path)
	}

	return float64(fileInfo.Size()) / 1000000, nil
}

// CleanupStaleTempFiles deletes this camera's half-written segments that are older than olderThan,
// which is every one of them left by a producer that died mid-segment. A producer writing right now
// touches its temp file every second, so its own work is never in range.
func CleanupStaleTempFiles(dir, cameraID string, olderThan time.Duration) (int, error) {
	matches, err := filepath.Glob(filepath.Join(dir, "Segment_*_"+cameraID+"*.mp4"+TempFileSuffix))
	if err != nil {
		return 0, fmt.Errorf("failed to look for stale temp segments in %v: %v", dir, err)
	}

	cleaned := 0
	var firstErr error

	for _, match := range matches {
		fileInfo, err := os.Stat(match)
		if err != nil {
			if firstErr == nil {
				firstErr = err
			}
			continue
		}

		if time.Since(fileInfo.ModTime()) <= olderThan {
			continue
		}

		if err := os.Remove(match); err != nil {
			if firstErr == nil {
				firstErr = err
			}
			continue
		}

		cleaned++
	}

	return cleaned, firstErr
}
