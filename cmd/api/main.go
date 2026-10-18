package main

import (
	"os"
	"strings"

	"github.com/go-chi/chi/v5"
	"github.com/gomodule/redigo/redis"
	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/internal/auth"
	"github.com/initialed85/camry/internal/timeline"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/djangolang/pkg/config"
	"github.com/initialed85/djangolang/pkg/server"
	"github.com/jackc/pgx/v5/pgxpool"
)

func addCustomHandlers(r chi.Router, db *pgxpool.Pool, redisPool *redis.Pool) error {

	a, err := auth.New()
	if err != nil {
		return err
	}

	a.Routes(r)

	// GET /api/custom/audio-timeline: hourly per-camera audio buckets for the date slider strip
	timeline.New(db).Routes(r)

	return nil
}

var log = api.ThisLogger()

func main() {
	if len(os.Args) < 2 {
		log.Fatal("first argument must be command (one of 'dump-config', 'dump-openapi-json', 'dump-openapi-yaml' or 'serve')")
	}

	command := strings.TrimSpace(strings.ToLower(os.Args[1]))

	switch command {

	case "dump-config":
		config.DumpConfig()

	case "dump-openapi-json":
		api.RunDumpOpenAPIJSON()

	case "dump-openapi-yaml":
		api.RunDumpOpenAPIYAML()

	case "serve":
		// passing middlewares replaces the djangolang defaults, so start from them
		api.RunServeWithEnvironment(server.GetDefaultHTTPMiddlewares(internal.JSONCompressMiddleware()), nil, addCustomHandlers)
	}
}
