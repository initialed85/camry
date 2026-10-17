package main

import (
	"log"

	"github.com/initialed85/camry/pkg/audio_detector"
)

func main() {
	err := audio_detector.Run()
	if err != nil {
		log.Fatalf("err: %v", err)
	}
}
