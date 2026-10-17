package audio_detector

import (
	"context"
	"encoding/json"
	"math"
	"math/rand"
	"os/exec"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

var testConfig = Config{
	SampleRate:    48000,
	WindowMs:      50,
	GridMs:        100,
	TriggerDb:     14.0,
	MaxGapWindows: 2,
	FFTSize:       2048,
	MelBands:      16,
}

func sinePCM(frequency, amplitude float64, samples, sampleRate int) []int16 {
	pcm := make([]int16, samples)
	period := float64(sampleRate) / frequency

	for i := range pcm {
		pcm[i] = int16(amplitude * math.Sin(2*math.Pi*float64(i)/period))
	}

	return pcm
}

func noisePCM(amplitude float64, samples int) []int16 {
	source := rand.New(rand.NewSource(42))
	pcm := make([]int16, samples)

	for i := range pcm {
		pcm[i] = int16(amplitude * (2*source.Float64() - 1))
	}

	return pcm
}

func concat(parts ...[]int16) []int16 {
	total := 0
	for _, part := range parts {
		total += len(part)
	}

	result := make([]int16, 0, total)
	for _, part := range parts {
		result = append(result, part...)
	}

	return result
}

func TestWindowizeTone(t *testing.T) {
	// 1 kHz sine at full-scale amplitude 3277 -> peak -20 dBFS, RMS -23.01 dBFS.
	pcm := sinePCM(1000, 3277, 4800, testConfig.SampleRate)
	stats := Windowize(pcm, testConfig.SampleRate, testConfig.WindowMs)

	require.Len(t, stats, 2)
	assert.InDelta(t, -20.0, stats[0].PeakDbfs, 0.1)
	assert.InDelta(t, -23.01, stats[0].RmsDbfs, 0.1)
}

func TestAnalyzeDetectsBurstOverBed(t *testing.T) {
	bed := sinePCM(300, 10, 2*testConfig.SampleRate, testConfig.SampleRate)
	burst := sinePCM(1000, 3277, 300*testConfig.SampleRate/1000, testConfig.SampleRate)
	tail := sinePCM(300, 10, 2*testConfig.SampleRate, testConfig.SampleRate)

	events, profile := Analyze(concat(bed, burst, tail), testConfig)

	assert.InDelta(t, -73.33, profile.FloorDbfs, 1.0)
	require.Len(t, events, 1)

	event := events[0]
	assert.InDelta(t, 2000, event.OffsetMs, 50)
	assert.InDelta(t, 300, event.DurationMs, 100)
	assert.InDelta(t, 50.3, event.ProminenceDb, 2.0)
	assert.InDelta(t, 3.01, event.CrestDb, 0.5)
	assert.Equal(t, "tonal", event.Kind)
	assert.Greater(t, event.Score, 0.7)
	assert.Less(t, event.Score, 1.0)
	assert.InDelta(t, 1000, event.Spectral.CentroidHz, 120)
	assert.Greater(t, event.Spectral.Tonality, 0.5)
	assert.Len(t, event.Spectral.Bands, testConfig.MelBands)
}

func TestAnalyzeSilence(t *testing.T) {
	events, profile := Analyze(make([]int16, testConfig.SampleRate), testConfig)

	assert.Empty(t, events)
	assert.Equal(t, minDBFS, profile.FloorDbfs)
	assert.Equal(t, 0, profile.Events)
	require.Len(t, profile.Values, 10)
	for _, value := range profile.Values {
		assert.Equal(t, minDBFS, value)
	}
}

func TestFindSpansMergesSmallGaps(t *testing.T) {
	// Loud blocks of 2 windows each, split by a 1-window and a 6-window gap.
	quiet := sinePCM(300, 10, testConfig.SampleRate/10, testConfig.SampleRate)
	loud := sinePCM(1000, 3277, testConfig.SampleRate/10, testConfig.SampleRate)

	adjacent := concat(loud, quiet[:testConfig.SampleRate/20], loud)
	spans := FindSpans(Windowize(adjacent, testConfig.SampleRate, testConfig.WindowMs), -73.33, testConfig)
	require.Len(t, spans, 1)
	assert.Equal(t, 0, spans[0].start)
	assert.Equal(t, 4, spans[0].end)

	separated := concat(loud, quiet, quiet, quiet, loud)
	spans = FindSpans(Windowize(separated, testConfig.SampleRate, testConfig.WindowMs), -73.33, testConfig)
	require.Len(t, spans, 2)
	assert.Equal(t, 0, spans[0].start)
	assert.Equal(t, 1, spans[0].end)
	assert.Equal(t, 8, spans[1].start)
	assert.Equal(t, 9, spans[1].end)
}

func TestFFTConcentratesAtToneBin(t *testing.T) {
	magnitudes := Magnitudes(sinePCM(1000, 10000, 2048, testConfig.SampleRate), 2048, Hann(2048))

	maxIndex, maxMagnitude := 0, 0.0
	for i, magnitude := range magnitudes {
		if magnitude > maxMagnitude {
			maxIndex, maxMagnitude = i, magnitude
		}
	}

	assert.InDelta(t, 1000, float64(maxIndex)*float64(testConfig.SampleRate)/2048.0, 25)
	assert.Greater(t, maxMagnitude, 0.0)
}

func TestMelBankShape(t *testing.T) {
	bank := NewMelBank(testConfig.MelBands, testConfig.FFTSize, testConfig.SampleRate)

	require.Len(t, bank.Filters, testConfig.MelBands)
	require.Len(t, bank.Frequencies, testConfig.FFTSize/2)

	for _, filter := range bank.Filters {
		require.Len(t, filter, testConfig.FFTSize/2)

		maxWeight := 0.0
		for _, weight := range filter {
			assert.GreaterOrEqual(t, weight, 0.0)
			assert.GreaterOrEqual(t, 1.0, weight)
			if weight > maxWeight {
				maxWeight = weight
			}
		}
		assert.Greater(t, maxWeight, 0.0)
	}

	tones := Magnitudes(sinePCM(1000, 10000, 2048, testConfig.SampleRate), 2048, Hann(2048))
	bandEnergies := bank.Apply(tones)

	total, loudest, loudestBand := 0.0, 0.0, -1
	for band, energy := range bandEnergies {
		total += energy
		if energy > loudest {
			loudest, loudestBand = energy, band
		}
	}

	assert.Greater(t, loudest/total, 0.5)
	assert.Greater(t, loudestBand, -1)
}

func TestAnalyzeSpectralToneVersusNoise(t *testing.T) {
	bank := NewMelBank(testConfig.MelBands, testConfig.FFTSize, testConfig.SampleRate)

	tone := AnalyzeSpectral(sinePCM(1000, 10000, 4800, testConfig.SampleRate), testConfig, bank)
	noise := AnalyzeSpectral(noisePCM(10000, 4800), testConfig, bank)

	assert.InDelta(t, 1000, tone.CentroidHz, 120)
	assert.Greater(t, tone.Tonality, 0.5)
	assert.Greater(t, tone.Tonality, noise.Tonality)
	assert.Greater(t, noise.Flatness, tone.Flatness)
	assert.Less(t, tone.CentroidStdHz, 400.0)
	assert.Greater(t, noise.CentroidHz, tone.CentroidHz)

	empty := AnalyzeSpectral([]int16{}, testConfig, bank)
	assert.Equal(t, 0.0, empty.CentroidHz)
	assert.Len(t, empty.Bands, testConfig.MelBands)
}

func TestKind(t *testing.T) {
	assert.Equal(t, "sustained", Kind(5000, 4, 0.1))
	assert.Equal(t, "impulsive", Kind(300, 15, 0.1))
	assert.Equal(t, "tonal", Kind(900, 3, 0.9))
	assert.Equal(t, "broadband", Kind(900, 3, 0.1))
}

func TestScoreRisesWithProminence(t *testing.T) {
	low := ScoreEvent(8, 4, 0.1, 6)
	high := ScoreEvent(28, 4, 0.1, 6)

	assert.Greater(t, high, low)
	assert.GreaterOrEqual(t, high, 0.0)
	assert.GreaterOrEqual(t, 1.0, high)
	assert.Equal(t, 0.0, ScoreEvent(1, 0, 0, 6))
	assert.Equal(t, 1.0, ScoreEvent(100, 40, 1, 6))
}

func TestScoreCalibration(t *testing.T) {
	// Measured over 29,106 deployed events (2026-10-09..10): prominence p50 8.1 dB /
	// p99 17.2 dB, crest p50 12.4 dB / p99 27.1 dB. The 2026-10-17 transients that
	// started this feature sit at 46-52 dB prominence, so they own the top of the scale.
	assert.Greater(t, ScoreEvent(46, 8, 0.3, 14), 0.8)
	assert.Greater(t, ScoreEvent(17.2, 27.1, 0.3, 14), 0.4)
	assert.Less(t, ScoreEvent(17.2, 27.1, 0.3, 14), 0.7)
	assert.Less(t, ScoreEvent(8.1, 12.4, 0.3, 14), 0.2)

	// The background median never clears the trigger, so it never becomes an event.
	assert.Empty(t, FindSpans([]WindowStats{{RmsDbfs: -53.9, PeakDbfs: -50.0}}, -62.0, testConfig))
}

func requireFFmpeg(t *testing.T) {
	t.Helper()

	if _, err := exec.LookPath("ffmpeg"); err != nil {
		t.Skip("ffmpeg is not installed")
	}
}

func runFFmpeg(t *testing.T, arguments ...string) {
	t.Helper()

	output, err := exec.Command("ffmpeg", append([]string{"-hide_banner", "-v", "error", "-y"}, arguments...)...).CombinedOutput()
	require.NoError(t, err, "ffmpeg failed: %s", string(output))
}

func TestDecodePCM(t *testing.T) {
	requireFFmpeg(t)

	path := filepath.Join(t.TempDir(), "tone.mp4")
	runFFmpeg(t, "-f", "lavfi", "-i", "sine=frequency=1000:sample_rate=48000:duration=2", "-c:a", "aac", path)

	pcm, err := DecodePCM(context.Background(), path, testConfig.SampleRate, "ffmpeg")
	require.NoError(t, err)

	assert.InDelta(t, 2*testConfig.SampleRate, len(pcm), 2400)

	events, profile := Analyze(pcm, testConfig)
	assert.Empty(t, events)

	// ffmpeg's AAC encoder normalises the full-scale lavfi sine down (astats: peak -17.64, RMS -21.49).
	assert.InDelta(t, -21.49, profile.FloorDbfs, 1.5)
	assert.InDelta(t, profile.FloorDbfs+3.01, profile.PeakMaxDbfs, 1.5)
}

func TestDecodePCMNoAudio(t *testing.T) {
	requireFFmpeg(t)

	path := filepath.Join(t.TempDir(), "video_only.mp4")
	runFFmpeg(t, "-f", "lavfi", "-i", "testsrc=duration=1:size=64x64:rate=5", "-c:v", "mpeg4", path)

	_, err := DecodePCM(context.Background(), path, testConfig.SampleRate, "ffmpeg")
	assert.ErrorIs(t, err, ErrNoAudio)
}

func TestDecodePCMMissingFile(t *testing.T) {
	requireFFmpeg(t)

	_, err := DecodePCM(context.Background(), filepath.Join(t.TempDir(), "absent.mp4"), testConfig.SampleRate, "ffmpeg")
	require.Error(t, err)
	assert.NotErrorIs(t, err, ErrNoAudio)
}

func TestGetSettingsDefaults(t *testing.T) {
	t.Setenv("DESTINATION_PATH", "/srv/media")

	settings, err := GetSettings()
	require.NoError(t, err)

	assert.Equal(t, "/srv/media", settings.DestinationPath)
	assert.Equal(t, "ffmpeg", settings.FFmpegPath)
	assert.Equal(t, 600, settings.ClaimLeaseSeconds)
	assert.Equal(t, 48000, settings.Config.SampleRate)
	assert.Equal(t, 50, settings.Config.WindowMs)
	assert.Equal(t, 100, settings.Config.GridMs)
	assert.Equal(t, 14.0, settings.Config.TriggerDb)
	assert.Equal(t, 2, settings.Config.MaxGapWindows)
	assert.Equal(t, 2048, settings.Config.FFTSize)
	assert.Equal(t, 16, settings.Config.MelBands)
}

func TestNoAudioProfileEncodesEmptyValues(t *testing.T) {
	// A jsonb "null" here breaks jsonb_array_length for every UI consumer of the curve.
	profile := Profile{Version: 1, SampleRate: testConfig.SampleRate, NoAudio: true, Values: []float64{}}

	encoded, err := json.Marshal(profile)
	require.NoError(t, err)
	assert.Contains(t, string(encoded), `"values":[]`)
	assert.NotContains(t, string(encoded), `"values":null`)
}
