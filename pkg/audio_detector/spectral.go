package audio_detector

import (
	"bytes"
	"context"
	"encoding/binary"
	"errors"
	"fmt"
	"math"
	"os/exec"
	"strings"
)

var ErrNoAudio = errors.New("file has no audio stream")

func DecodePCM(ctx context.Context, path string, sampleRate int, ffmpegPath string) ([]int16, error) {
	stdout := &bytes.Buffer{}

	command := exec.CommandContext(
		ctx,
		ffmpegPath,
		"-hide_banner",
		"-v", "error",
		"-i", path,
		"-map", "0:a:0",
		"-f", "s16le",
		"-acodec", "pcm_s16le",
		"-ac", "1",
		"-ar", fmt.Sprintf("%d", sampleRate),
		"-",
	)
	command.Stdout = stdout
	command.Stderr = &bytes.Buffer{}

	err := command.Run()
	if err != nil {
		stderr := command.Stderr.(*bytes.Buffer).String()

		if strings.Contains(stderr, "matches no streams") {
			return nil, fmt.Errorf("%w: %s", ErrNoAudio, stderr)
		}

		return nil, fmt.Errorf("failed to decode %s: %w: %s", path, err, stderr)
	}

	sampleCount := stdout.Len() / 2
	pcm := make([]int16, sampleCount)

	for i := 0; i < sampleCount; i++ {
		pcm[i] = int16(binary.LittleEndian.Uint16(stdout.Bytes()[i*2:]))
	}

	return pcm, nil
}

func AnalyzeSpectral(samples []int16, cfg Config, bank *MelBank) Spectral {
	fftSize := cfg.FFTSize
	if fftSize < 2 {
		fftSize = 2
	}

	hop := fftSize / 2
	window := Hann(fftSize)

	frameCount := 0
	if len(samples) > 0 {
		frameCount = (len(samples) + hop) / hop
	}

	if frameCount < 1 {
		frameCount = 1
	}

	centroids := make([]float64, 0, frameCount)
	bandSums := make([]float64, bank.Bands)
	bandTotal := 0.0
	fluxSum := 0.0
	flatnessSum := 0.0
	tonalityMax := 0.0
	previous := make([]float64, fftSize/2)

	for frame := 0; ; frame++ {
		start := frame * hop
		if start >= len(samples) {
			break
		}

		end := start + fftSize
		if end > len(samples) {
			end = len(samples)
		}

		magnitudes := Magnitudes(samples[start:end], fftSize, window)

		centroids = append(centroids, Centroid(magnitudes, bank.Frequencies))
		flatnessSum += Flatness(magnitudes)
		fluxSum += Flux(magnitudes, previous)
		copy(previous, magnitudes)

		bandEnergies := bank.Apply(magnitudes)
		var bandSum float64
		for _, energy := range bandEnergies {
			bandSum += energy
		}
		if bandSum > 0 {
			bandTotal += bandSum
			for band, energy := range bandEnergies {
				bandSums[band] += energy
				if ratio := energy / bandSum; ratio > tonalityMax {
					tonalityMax = ratio
				}
			}
		}
	}

	frameCount = len(centroids)
	if frameCount == 0 {
		return Spectral{Bands: make([]float64, bank.Bands)}
	}

	var centroidSum, centroidSquaredSum float64
	for _, centroid := range centroids {
		centroidSum += centroid
		centroidSquaredSum += centroid * centroid
	}

	meanCentroid := centroidSum / float64(frameCount)
	variance := centroidSquaredSum/float64(frameCount) - meanCentroid*meanCentroid
	if variance < 0 {
		variance = 0
	}

	bands := make([]float64, bank.Bands)
	if bandTotal > 0 {
		for band := range bands {
			bands[band] = round(bandSums[band]/bandTotal, 4)
		}
	}

	return Spectral{
		CentroidHz:    round(meanCentroid, 1),
		CentroidStdHz: round(math.Sqrt(variance), 1),
		Flux:          round(fluxSum/float64(frameCount), 4),
		Flatness:      round(flatnessSum/float64(frameCount), 4),
		Tonality:      round(tonalityMax, 4),
		Bands:         bands,
	}
}
