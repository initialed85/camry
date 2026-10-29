package main

import (
	"os"
	"strings"

	"github.com/go-chi/chi/v5"
	"github.com/gomodule/redigo/redis"
	"github.com/initialed85/camry/internal"
	"github.com/initialed85/camry/internal/auth"
	"github.com/initialed85/camry/internal/cache"
	"github.com/initialed85/camry/internal/timeline"
	"github.com/initialed85/camry/pkg/api"
	"github.com/initialed85/djangolang/pkg/config"
	"github.com/initialed85/djangolang/pkg/server"
	"github.com/jackc/pgx/v5/pgxpool"
)

func addCustomHandlers(a *auth.Auth) func(chi.Router, *pgxpool.Pool, *redis.Pool) error {
	return func(r chi.Router, db *pgxpool.Pool, redisPool *redis.Pool) error {
		a.Routes(r)

		// GET /api/custom/audio-timeline: hourly per-camera audio buckets for the date slider strip
		timeline.New(db).Routes(r)

		return nil
	}
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
		a, err := auth.New()
		if err != nil {
			log.Fatal(err)
		}

		// The response cache goes outside the compressor: it stores and replays the bytes that go on
		// the wire, so a hit replays a stored gzip body without recompressing it. The identity check is
		// mounted ahead of the cache - djangolang appends these extras after RequestID/RealIP/Logger and
		// chi runs them outermost-first - so a cached body is never replayed to a caller that has not
		// passed it.
		cacheMiddleware, err := cache.New()
		if err != nil {
			log.Fatal(err)
		}

		api.RunServeWithEnvironment(server.GetDefaultHTTPMiddlewares(a.Middleware, cacheMiddleware, internal.JSONCompressMiddleware()), nil, addCustomHandlers(a))
	}
}
