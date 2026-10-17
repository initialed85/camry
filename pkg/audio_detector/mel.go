package audio_detector

import "math"

type MelBank struct {
	Bands       int
	FFTSize     int
	SampleRate  int
	Frequencies []float64
	Filters     [][]float64
}

func hzToMel(hz float64) float64 {
	return 2595 * math.Log10(1+hz/700)
}

func melToHz(mel float64) float64 {
	return 700 * (math.Pow(10, mel/2595) - 1)
}

func NewMelBank(bands, fftSize, sampleRate int) *MelBank {
	if bands < 1 {
		bands = 1
	}

	nyquist := float64(sampleRate) / 2
	if nyquist > 22050 {
		nyquist = 22050
	}

	low, high := 20.0, nyquist

	edges := make([]float64, bands+2)
	melLow, melHigh := hzToMel(low), hzToMel(high)
	for i := range edges {
		edges[i] = melToHz(melLow + (melHigh-melLow)*float64(i)/float64(bands+1))
	}

	binCount := fftSize / 2
	binWidth := float64(sampleRate) / float64(fftSize)

	filters := make([][]float64, bands)
	for band := 0; band < bands; band++ {
		filter := make([]float64, binCount)
		left, center, right := edges[band], edges[band+1], edges[band+2]

		for bin := 0; bin < binCount; bin++ {
			frequency := float64(bin) * binWidth

			weight := 0.0
			if frequency > left && frequency <= center && center > left {
				weight = (frequency - left) / (center - left)
			} else if frequency > center && frequency < right && right > center {
				weight = (right - frequency) / (right - center)
			}

			filter[bin] = weight
		}

		filters[band] = filter
	}

	frequencies := make([]float64, binCount)
	for bin := range frequencies {
		frequencies[bin] = float64(bin) * binWidth
	}

	return &MelBank{
		Bands:       bands,
		FFTSize:     fftSize,
		SampleRate:  sampleRate,
		Frequencies: frequencies,
		Filters:     filters,
	}
}

func (m *MelBank) Apply(magnitudes []float64) []float64 {
	bandEnergies := make([]float64, m.Bands)

	for band, filter := range m.Filters {
		var sum float64
		for bin, weight := range filter {
			if weight == 0 || bin >= len(magnitudes) {
				continue
			}
			sum += magnitudes[bin] * weight
		}
		bandEnergies[band] = sum
	}

	return bandEnergies
}

func Centroid(magnitudes []float64, frequencies []float64) float64 {
	var weighted, total float64

	for bin, magnitude := range magnitudes {
		if bin >= len(frequencies) {
			continue
		}
		weighted += magnitude * frequencies[bin]
		total += magnitude
	}

	if total == 0 {
		return 0
	}

	return weighted / total
}

func Flatness(magnitudes []float64) float64 {
	if len(magnitudes) == 0 {
		return 0
	}

	var logSum float64
	var arithmetic float64
	binCount := 0

	for _, magnitude := range magnitudes {
		value := magnitude
		if value < 1e-10 {
			value = 1e-10
		}
		logSum += math.Log(value)
		arithmetic += magnitude
		binCount++
	}

	if binCount == 0 || arithmetic == 0 {
		return 0
	}

	geometric := math.Exp(logSum / float64(binCount))
	return geometric / (arithmetic / float64(binCount))
}

func Flux(current, previous []float64) float64 {
	if len(current) == 0 {
		return 0
	}

	var positive, total float64

	for bin, magnitude := range current {
		delta := magnitude
		if bin < len(previous) {
			delta = magnitude - previous[bin]
		}
		if delta > 0 {
			positive += delta
		}
		total += magnitude
	}

	if total == 0 {
		return 0
	}

	return positive / total
}
