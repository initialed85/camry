package audio_detector

import (
	"math"
	"math/cmplx"
)

func FFT(re, im []float64) {
	n := len(re)
	if n&(n-1) != 0 {
		return
	}

	for i, j := 1, 0; i < n; i++ {
		bit := n >> 1
		for j&bit != 0 {
			j &= ^bit
			bit >>= 1
		}
		j |= bit
		if i < j {
			re[i], re[j] = re[j], re[i]
			im[i], im[j] = im[j], im[i]
		}
	}

	for length := 2; length <= n; length *= 2 {
		angle := -2 * math.Pi / float64(length)
		wReal, wImag := math.Cos(angle), math.Sin(angle)

		for start := 0; start < n; start += length {
			currentReal, currentImag := 1.0, 0.0

			for offset := 0; offset < length/2; offset++ {
				even := start + offset
				odd := even + length/2

				tReal := re[odd]*currentReal - im[odd]*currentImag
				tImag := re[odd]*currentImag + im[odd]*currentReal

				re[odd] = re[even] - tReal
				im[odd] = im[even] - tImag
				re[even] += tReal
				im[even] += tImag

				nextReal := currentReal*wReal - currentImag*wImag
				currentImag = currentReal*wImag + currentImag*wReal
				currentReal = nextReal
			}
		}
	}
}

func Hann(size int) []float64 {
	window := make([]float64, size)

	for i := range window {
		window[i] = 0.5 - 0.5*math.Cos(2*math.Pi*float64(i)/float64(size))
	}

	return window
}

func Magnitudes(samples []int16, fftSize int, window []float64) []float64 {
	re := make([]float64, fftSize)
	im := make([]float64, fftSize)

	for i := range re {
		if i < len(samples) {
			re[i] = (float64(samples[i]) / fullScale) * window[i]
		}
	}

	FFT(re, im)

	magnitudes := make([]float64, fftSize/2)
	for i := range magnitudes {
		magnitudes[i] = cmplx.Abs(complex(re[i], im[i]))
	}

	return magnitudes
}
