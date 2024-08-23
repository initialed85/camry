package main

import (
	"log"

	"github.com/initialed85/camry/pkg/segment_producer"
)

func main() {
	err := segment_producer.Run()
	if err != nil {
		log.Fatalf("err: %v", err)
	}
}
