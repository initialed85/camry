package segment_producer

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"time"

	"github.com/alfg/mp4"
)

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
