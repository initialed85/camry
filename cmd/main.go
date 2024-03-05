package main

import (
	"log"
	"os"

	"github.com/initialed85/camry/pkg/app/segment_producer"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("first arg must be command")
	}
	command := os.Args[1]
	log.Printf("command: %#+v", command)

	var err error

	switch command {
	case "segment_producer":
		err = segment_producer.Run()
	}

	if err != nil {
		log.Fatalf("command %#+v failed; err: %v", command, err)
	}
}
