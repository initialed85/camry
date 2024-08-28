package main

import (
	"log"

	"github.com/initialed85/camry/pkg/stream_producer"
)

func main() {
	err := stream_producer.Run()
	if err != nil {
		log.Fatalf("err: %v", err)
	}
}
