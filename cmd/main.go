package main

import (
	"log"
	"os"
	"strings"

	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/camry/pkg/segment_producer"
	"github.com/initialed85/camry/pkg/stream_producer"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("first arg must be command (one of 'serve', 'dump-openapi-json', 'dump-openapi-yaml', 'segment_producer', 'stream_producer')")
	}

	command := strings.TrimSpace(strings.ToLower(os.Args[1]))

	var err error

	switch command {

	case "dump-openapi-json":
		api.RunDumpOpenAPIJSON()

	case "dump-openapi-yaml":
		api.RunDumpOpenAPIYAML()

	case "serve":
		api.RunServeWithEnvironment()

	case "segment_producer":
		err = segment_producer.Run()

	case "stream_producer":
		err = stream_producer.Run()

	default:
		log.Fatal("first arg must be command (one of 'serve', 'dump-openapi-json', 'dump-openapi-yaml', 'segment_producer', 'stream_producer')")
	}

	if err != nil {
		log.Fatalf("command %#+v failed; err: %v", command, err)
	}
}
