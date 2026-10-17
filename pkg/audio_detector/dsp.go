package audio_detector

import (
	"math"
	"sort"
)

const (
	minDBFS   = -120.0
	fullScale = 32768.0
)

type Config struct {
	SampleRate    int
	WindowMs      int
	GridMs        int
	TriggerDb     float64
	MaxGapWindows int
	FFTSize       int
	MelBands      int
}

type WindowStats struct {
	RmsDbfs  float64
	PeakDbfs float64
}

type Spectral struct {
	CentroidHz    float64
	CentroidStdHz float64
	Flux          float64
	Flatness      float64
	Tonality      float64
	Bands         []float64
}

type Event struct {
	OffsetMs     int64
	DurationMs   int64
	Kind         string
	RmsDbfs      float64
	PeakDbfs     float64
	FloorDbfs    float64
	ProminenceDb float64
	CrestDb      float64
	Score        float64
	Spectral     Spectral
}

type Profile struct {
	Version     int       `json:"version"`
	SampleRate  int       `json:"sample_rate"`
	WindowMs    int       `json:"window_ms"`
	GridMs      int       `json:"grid_ms"`
	Windows     int       `json:"windows"`
	DurationMs  int64     `json:"duration_ms"`
	FloorDbfs   float64   `json:"floor_dbfs"`
	RmsP50Dbfs  float64   `json:"rms_p50_dbfs"`
	RmsP95Dbfs  float64   `json:"rms_p95_dbfs"`
	PeakMaxDbfs float64   `json:"peak_max_dbfs"`
	Events      int       `json:"events"`
	NoAudio     bool      `json:"no_audio"`
	Values      []float64 `json:"values"`
}

func toDBFS(v float64) float64 {
	if v <= 0 {
		return minDBFS
	}

	db := 20 * math.Log10(v)

	if math.IsNaN(db) || math.IsInf(db, 0) || db < minDBFS {
		return minDBFS
	}

	return db
}

func round(v float64, places int) float64 {
	factor := math.Pow(10, float64(places))
	return math.Round(v*factor) / factor
}

func Windowize(pcm []int16, sampleRate, windowMs int) []WindowStats {
	windowSamples := sampleRate * windowMs / 1000
	if windowSamples < 1 {
		windowSamples = 1
	}

	stats := make([]WindowStats, 0, len(pcm)/windowSamples+1)

	for start := 0; start < len(pcm); start += windowSamples {
		end := start + windowSamples
		if end > len(pcm) {
			end = len(pcm)
		}

		var sumOfSquares float64
		var peak float64

		for _, sample := range pcm[start:end] {
			value := float64(sample) / fullScale
			sumOfSquares += value * value

			absolute := value
			if absolute < 0 {
				absolute = -absolute
			}
			if absolute > peak {
				peak = absolute
			}
		}

		stats = append(stats, WindowStats{
			RmsDbfs:  toDBFS(math.Sqrt(sumOfSquares / float64(end-start))),
			PeakDbfs: toDBFS(peak),
		})
	}

	return stats
}

func GridValues(pcm []int16, sampleRate, gridMs int) []float64 {
	stats := Windowize(pcm, sampleRate, gridMs)
	values := make([]float64, 0, len(stats))

	for _, stat := range stats {
		values = append(values, round(stat.RmsDbfs, 1))
	}

	return values
}

func Median(values []float64) float64 {
	return Percentile(values, 0.5)
}

func Percentile(values []float64, p float64) float64 {
	if len(values) == 0 {
		return minDBFS
	}

	sorted := make([]float64, len(values))
	copy(sorted, values)
	sort.Float64s(sorted)

	if p <= 0 {
		return sorted[0]
	}
	if p >= 1 {
		return sorted[len(sorted)-1]
	}

	position := p * float64(len(sorted)-1)
	lower := int(position)
	upper := lower + 1
	if upper >= len(sorted) {
		return sorted[lower]
	}

	weight := position - float64(lower)
	return sorted[lower]*(1-weight) + sorted[upper]*weight
}

type span struct {
	start int
	end   int
}

func FindSpans(stats []WindowStats, floorDbfs float64, cfg Config) []span {
	spans := make([]span, 0)

	i := 0
	for i < len(stats) {
		if stats[i].RmsDbfs-floorDbfs < cfg.TriggerDb {
			i++
			continue
		}

		last := i

		for k := i + 1; k < len(stats); k++ {
			if stats[k].RmsDbfs-floorDbfs >= cfg.TriggerDb {
				last = k
				continue
			}

			if k-last > cfg.MaxGapWindows {
				break
			}
		}

		spans = append(spans, span{start: i, end: last})
		i = last + 1
	}

	return spans
}

func ScoreEvent(prominenceDb, crestDb, flux, triggerDb float64) float64 {
	score := 0.7*(prominenceDb-triggerDb)/30 + 0.3*(crestDb-10)/20 + 0.15*min(flux/0.3, 1.0)

	if score < 0 {
		return 0
	}
	if score > 1 {
		return 1
	}

	return round(score, 3)
}

func Kind(durationMs int64, crestDb, tonality float64) string {
	switch {
	case durationMs >= 4000:
		return "sustained"
	case durationMs <= 400 && crestDb >= 12:
		return "impulsive"
	case tonality >= 0.55:
		return "tonal"
	default:
		return "broadband"
	}
}

func Analyze(pcm []int16, cfg Config) ([]Event, Profile) {
	durationMs := int64(len(pcm)) * 1000 / int64(cfg.SampleRate)

	stats := Windowize(pcm, cfg.SampleRate, cfg.WindowMs)

	rmsValues := make([]float64, 0, len(stats))
	peakMax := minDBFS
	for _, stat := range stats {
		rmsValues = append(rmsValues, stat.RmsDbfs)
		if stat.PeakDbfs > peakMax {
			peakMax = stat.PeakDbfs
		}
	}

	floor := Median(rmsValues)

	profile := Profile{
		Version:     1,
		SampleRate:  cfg.SampleRate,
		WindowMs:    cfg.WindowMs,
		GridMs:      cfg.GridMs,
		Windows:     len(stats),
		DurationMs:  durationMs,
		FloorDbfs:   round(floor, 2),
		RmsP50Dbfs:  round(Percentile(rmsValues, 0.5), 2),
		RmsP95Dbfs:  round(Percentile(rmsValues, 0.95), 2),
		PeakMaxDbfs: round(peakMax, 2),
		Values:      GridValues(pcm, cfg.SampleRate, cfg.GridMs),
	}

	spans := FindSpans(stats, floor, cfg)
	profile.Events = len(spans)

	windowSamples := cfg.SampleRate * cfg.WindowMs / 1000
	bank := NewMelBank(cfg.MelBands, cfg.FFTSize, cfg.SampleRate)

	events := make([]Event, 0, len(spans))

	for _, s := range spans {
		var rmsSum float64
		peak := minDBFS

		for k := s.start; k <= s.end; k++ {
			rmsSum += stats[k].RmsDbfs
			if stats[k].PeakDbfs > peak {
				peak = stats[k].PeakDbfs
			}
		}

		count := float64(s.end - s.start + 1)
		rms := rmsSum / count

		startSample := s.start * windowSamples
		endSample := (s.end + 1) * windowSamples
		if endSample > len(pcm) {
			endSample = len(pcm)
		}

		spectral := AnalyzeSpectral(pcm[startSample:endSample], cfg, bank)

		offsetMs := int64(s.start * windowSamples * 1000 / cfg.SampleRate)
		duration := int64(endSample-startSample) * 1000 / int64(cfg.SampleRate)

		prominence := rms - floor
		crest := peak - rms

		events = append(events, Event{
			OffsetMs:     offsetMs,
			DurationMs:   duration,
			Kind:         Kind(duration, crest, spectral.Tonality),
			RmsDbfs:      round(rms, 2),
			PeakDbfs:     round(peak, 2),
			FloorDbfs:    round(floor, 2),
			ProminenceDb: round(prominence, 2),
			CrestDb:      round(crest, 2),
			Score:        ScoreEvent(prominence, crest, spectral.Flux, cfg.TriggerDb),
			Spectral:     spectral,
		})
	}

	return events, profile
}
