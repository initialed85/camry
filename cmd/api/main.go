package main

import (
	"context"
	"log"
	"os"
	"strings"

	"github.com/go-chi/chi/v5"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/djangolang/pkg/config"
	"github.com/initialed85/djangolang/pkg/server"
)

type ClaimRequest struct {
	ClaimDurationSeconds float64 `json:"claim_duration_seconds"`
}

func RunServeWithEnvironment(
	httpMiddlewares []server.HTTPMiddleware,
	objectMiddlewares []server.ObjectMiddleware,
	addCustomHandlers func(chi.Router) error,
) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	port := config.Port()

	db, err := config.GetDBFromEnvironment(ctx)
	if err != nil {
		log.Fatalf("%v", err)
	}
	defer func() {
		db.Close()
	}()

	redisPool, err := config.GetRedisFromEnvironment()
	if err != nil {
		log.Fatalf("%v", err)
	}
	defer func() {
		_ = redisPool.Close()
	}()

	actualAddCustomHandlers := func(r chi.Router) error {
		return nil
	}

	api.RunServeWithArguments(ctx, cancel, port, db, redisPool, nil, nil, actualAddCustomHandlers)
}

func main() {
	if len(os.Args) < 2 {
		log.Fatal("first argument must be command (one of 'serve', 'dump-openapi-json', 'dump-openapi-yaml')")
	}

	command := strings.TrimSpace(strings.ToLower(os.Args[1]))

	switch command {

	case "dump-openapi-json":
		api.RunDumpOpenAPIJSON()

	case "dump-openapi-yaml":
		api.RunDumpOpenAPIYAML()

	case "serve":
		RunServeWithEnvironment(nil, nil, nil)
	}
}
