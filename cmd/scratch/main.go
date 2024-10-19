package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"runtime"

	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/camry/pkg/object_tracker"
	"github.com/initialed85/djangolang/pkg/server"
	"gocv.io/x/gocv"

	_ "embed"
)

//go:embed test_response.json
var testResponseJSON []byte

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	var resp *server.Response[api.Video]
	err := json.Unmarshal(testResponseJSON, &resp)
	if err != nil {
		log.Fatal(err)
	}

	if len(resp.Objects) < 1 {
		log.Fatal(fmt.Errorf("assertion fails: need at least one video"))
	}

	video := resp.Objects[0]

	mats := make(chan gocv.Mat) // note: unbuffered

	go func() {
		err = object_tracker.Track(ctx, video, mats)
		if err != nil {
			log.Fatal(err)
		}
	}()
	runtime.Gosched()

	err = object_tracker.HandleDebugWindow(ctx, cancel, mats)
	if err != nil {
		log.Fatal(err)
	}
}
