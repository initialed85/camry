package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"sync"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/gomodule/redigo/redis"
	"github.com/initialed85/djangolang/pkg/config"
	"github.com/initialed85/djangolang/pkg/introspect"
	"github.com/initialed85/djangolang/pkg/openapi"
	"github.com/initialed85/djangolang/pkg/server"
	"github.com/initialed85/djangolang/pkg/types"
	"github.com/jackc/pgx/v5/pgxpool"
	"gopkg.in/yaml.v2"

	"net/http/pprof"
)

type patternAndMutateRouterFn struct {
	pattern        string
	mutateRouterFn server.MutateRouterFn
}

var mu = new(sync.Mutex)
var newFromItemFnByTableName = make(map[string]func(map[string]any) (any, error))
var patternsAndMutateRouterFns = make([]patternAndMutateRouterFn, 0)
var allObjects = make([]any, 0)
var openApi *types.OpenAPI
var profile = config.Profile()

var httpHandlerSummaries []server.HTTPHandlerSummary = make([]server.HTTPHandlerSummary, 0)

func isRequired(columns map[string]*introspect.Column, columnName string) bool {
	column := columns[columnName]
	if column == nil {
		return false
	}

	return column.NotNull && !column.HasDefault
}

func register(
	tableName string,
	object any,
	newFromItem func(map[string]any) (any, error),
	pattern string,
	getRouterFn server.MutateRouterFn,
) {
	allObjects = append(allObjects, object)
	newFromItemFnByTableName[tableName] = newFromItem
	patternsAndMutateRouterFns = append(patternsAndMutateRouterFns, patternAndMutateRouterFn{
		pattern:        pattern,
		mutateRouterFn: getRouterFn,
	})
}

func GetOpenAPI() (*types.OpenAPI, error) {
	mu.Lock()
	defer mu.Unlock()

	if openApi != nil {
		return openApi, nil
	}

	var err error
	openApi, err = openapi.NewFromIntrospectedSchema(httpHandlerSummaries)
	if err != nil {
		return nil, err
	}

	return openApi, nil
}

func NewFromItem(tableName string, item map[string]any) (any, error) {
	if item == nil {
		return nil, nil
	}

	mu.Lock()
	newFromItemFn, ok := newFromItemFnByTableName[tableName]
	mu.Unlock()

	if !ok {
		return nil, fmt.Errorf("table name %v not known", tableName)
	}

	return newFromItemFn(item)
}

func MutateRouter(r chi.Router, db *pgxpool.Pool, redisPool *redis.Pool, objectMiddlewares []server.ObjectMiddleware, waitForChange server.WaitForChange) {
	mu.Lock()
	patternsAndGetRouterFns := patternsAndMutateRouterFns
	mu.Unlock()

	for _, thisPatternAndGetRouterFn := range patternsAndGetRouterFns {
		thisPatternAndGetRouterFn.mutateRouterFn(r, db, redisPool, objectMiddlewares, waitForChange)
	}

	healthzMu := new(sync.Mutex)
	healthzExpiresAt := time.Now().Add(-time.Second * 5)
	var lastHealthz error

	healthz := func(ctx context.Context) error {
		healthzMu.Lock()
		defer healthzMu.Unlock()

		if time.Now().Before(healthzExpiresAt) {
			return lastHealthz
		}

		lastHealthz = func() error {
			err := db.Ping(ctx)
			if err != nil {
				return fmt.Errorf("db ping failed; %v", err)
			}

			redisConn, err := redisPool.GetContext(ctx)
			if err != nil {
				return fmt.Errorf("redis pool get failed; %v", err)
			}

			defer func() {
				_ = redisConn.Close()
			}()

			_, err = redisConn.Do("PING")
			if err != nil {
				return fmt.Errorf("redis ping failed; %v", err)
			}

			return nil
		}()

		healthzExpiresAt = time.Now().Add(time.Second * 5)

		return lastHealthz
	}

	if profile {
		r.HandleFunc("/debug/pprof/cmdline", pprof.Cmdline)
		r.HandleFunc("/debug/pprof/profile", pprof.Profile)
		r.HandleFunc("/debug/pprof/symbol", pprof.Symbol)
		r.HandleFunc("/debug/pprof/trace", pprof.Trace)

		r.Handle("/debug/pprof/goroutine", pprof.Handler("goroutine"))
		r.Handle("/debug/pprof/heap", pprof.Handler("heap"))
		r.Handle("/debug/pprof/allocs", pprof.Handler("allocs"))
		r.Handle("/debug/pprof/threadcreate", pprof.Handler("threadcreate"))
		r.Handle("/debug/pprof/block", pprof.Handler("block"))
		r.Handle("/debug/pprof/mutex", pprof.Handler("mutex"))

		r.HandleFunc("/debug/pprof/", pprof.Index)
	}

	r.Get("/healthz", func(w http.ResponseWriter, r *http.Request) {
		ctx, cancel := context.WithTimeout(r.Context(), time.Second*5)
		defer cancel()

		err := healthz(ctx)
		if err != nil {
			server.HandleErrorResponse(w, http.StatusInternalServerError, err)
			return
		}

		server.HandleObjectsResponse(w, http.StatusOK, nil)
	})

	r.Get("/openapi.json", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Add("Content-type", "application/json")

		openApi, err := GetOpenAPI()
		if err != nil {
			server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to get OpenAPI schema; %v", err))
			return
		}

		b, err := json.MarshalIndent(openApi, "", "  ")
		if err != nil {
			server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to get OpenAPI schema; %v", err))
			return
		}

		server.WriteResponse(w, http.StatusOK, b)
	})

	r.Get("/openapi.yaml", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Add("Content-type", "application/yaml")

		openApi, err := GetOpenAPI()
		if err != nil {
			server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to get OpenAPI schema; %v", err))
			return
		}

		b, err := yaml.Marshal(openApi)
		if err != nil {
			server.HandleErrorResponse(w, http.StatusInternalServerError, fmt.Errorf("failed to get OpenAPI schema; %v", err))
			return
		}

		server.WriteResponse(w, http.StatusOK, b)
	})
}

func getHTTPHandler[T any, S any, Q any, R any](method string, path string, status int, handle func(context.Context, T, S, Q, any) (R, error), modelObject any, table *introspect.Table) (*server.HTTPHandler[T, S, Q, R], error) {
	customHTTPHandler, err := server.GetHTTPHandler(method, path, status, handle)
	if err != nil {
		return nil, err
	}

	customHTTPHandler.Builtin = true
	customHTTPHandler.BuiltinModelObject = modelObject
	customHTTPHandler.BuiltinTable = table

	mu.Lock()
	httpHandlerSummaries = append(httpHandlerSummaries, customHTTPHandler.Summarize())
	mu.Unlock()

	return customHTTPHandler, nil
}

func GetHTTPHandler[T any, S any, Q any, R any](method string, path string, status int, handle func(context.Context, T, S, Q, any) (R, error)) (*server.HTTPHandler[T, S, Q, R], error) {
	customHTTPHandler, err := server.GetHTTPHandler(method, path, status, handle)
	if err != nil {
		return nil, err
	}

	mu.Lock()
	httpHandlerSummaries = append(httpHandlerSummaries, customHTTPHandler.Summarize())
	mu.Unlock()

	return customHTTPHandler, nil
}

func GetHTTPHandlerSummaries() ([]server.HTTPHandlerSummary, error) {
	mu.Lock()
	defer mu.Unlock()

	if len(httpHandlerSummaries) == 0 {
		return nil, fmt.Errorf("httpHandlerSummaries unexpectedly empty; you'll need to call GetRouter() so that this is populated")
	}

	return httpHandlerSummaries, nil
}

var tableByNameAsJSON = []byte(`{
  "camera": {
    "tablename": "camera",
    "oid": "20314",
    "schema": "public",
    "reltuples": -1,
    "relkind": "r",
    "relam": "2",
    "relacl": null,
    "reltype": "20316",
    "relowner": "10",
    "relhasindex": true,
    "columns": [
      {
        "column": "id",
        "datatype": "uuid",
        "table": "camera",
        "pos": 1,
        "typeid": "2950",
        "typelen": 16,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": true,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20314",
        "zero_type": "00000000-0000-0000-0000-000000000000",
        "query_type_template": "uuid.UUID",
        "stream_type_template": "[16]uint8",
        "type_template": "uuid.UUID"
      },
      {
        "column": "created_at",
        "datatype": "timestamp with time zone",
        "table": "camera",
        "pos": 2,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20314",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "updated_at",
        "datatype": "timestamp with time zone",
        "table": "camera",
        "pos": 3,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20314",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "deleted_at",
        "datatype": "timestamp with time zone",
        "table": "camera",
        "pos": 4,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": false,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20314",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "name",
        "datatype": "text",
        "table": "camera",
        "pos": 5,
        "typeid": "25",
        "typelen": -1,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20314",
        "zero_type": "",
        "query_type_template": "string",
        "stream_type_template": "string",
        "type_template": "string"
      },
      {
        "column": "stream_url",
        "datatype": "text",
        "table": "camera",
        "pos": 6,
        "typeid": "25",
        "typelen": -1,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20314",
        "zero_type": "",
        "query_type_template": "string",
        "stream_type_template": "string",
        "type_template": "string"
      },
      {
        "column": "last_seen",
        "datatype": "timestamp with time zone",
        "table": "camera",
        "pos": 7,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20314",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "segment_producer_claimed_until",
        "datatype": "timestamp with time zone",
        "table": "camera",
        "pos": 8,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20314",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "stream_producer_claimed_until",
        "datatype": "timestamp with time zone",
        "table": "camera",
        "pos": 9,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20314",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      }
    ]
  },
  "detection": {
    "tablename": "detection",
    "oid": "20361",
    "schema": "public",
    "reltuples": -1,
    "relkind": "r",
    "relam": "2",
    "relacl": null,
    "reltype": "20363",
    "relowner": "10",
    "relhasindex": true,
    "columns": [
      {
        "column": "id",
        "datatype": "uuid",
        "table": "detection",
        "pos": 1,
        "typeid": "2950",
        "typelen": 16,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": true,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": "00000000-0000-0000-0000-000000000000",
        "query_type_template": "uuid.UUID",
        "stream_type_template": "[16]uint8",
        "type_template": "uuid.UUID"
      },
      {
        "column": "created_at",
        "datatype": "timestamp with time zone",
        "table": "detection",
        "pos": 2,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "updated_at",
        "datatype": "timestamp with time zone",
        "table": "detection",
        "pos": 3,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "deleted_at",
        "datatype": "timestamp with time zone",
        "table": "detection",
        "pos": 4,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": false,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "seen_at",
        "datatype": "timestamp with time zone",
        "table": "detection",
        "pos": 5,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "class_id",
        "datatype": "bigint",
        "table": "detection",
        "pos": 6,
        "typeid": "20",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": 0,
        "query_type_template": "int64",
        "stream_type_template": "int64",
        "type_template": "int64"
      },
      {
        "column": "class_name",
        "datatype": "text",
        "table": "detection",
        "pos": 7,
        "typeid": "25",
        "typelen": -1,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": "",
        "query_type_template": "string",
        "stream_type_template": "string",
        "type_template": "string"
      },
      {
        "column": "score",
        "datatype": "double precision",
        "table": "detection",
        "pos": 8,
        "typeid": "701",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": 0,
        "query_type_template": "float64",
        "stream_type_template": "float64",
        "type_template": "float64"
      },
      {
        "column": "centroid",
        "datatype": "point",
        "table": "detection",
        "pos": 9,
        "typeid": "600",
        "typelen": 16,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": {
          "X": 0,
          "Y": 0
        },
        "query_type_template": "pgtype.Point",
        "stream_type_template": "pgtype.Point",
        "type_template": "pgtype.Vec2"
      },
      {
        "column": "bounding_box",
        "datatype": "polygon",
        "table": "detection",
        "pos": 10,
        "typeid": "604",
        "typelen": -1,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20361",
        "zero_type": [],
        "query_type_template": "pgtype.Polygon",
        "stream_type_template": "pgtype.Polygon",
        "type_template": "[]pgtype.Vec2"
      },
      {
        "column": "video_id",
        "datatype": "uuid",
        "table": "detection",
        "pos": 11,
        "typeid": "2950",
        "typelen": 16,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": "video",
        "fcolumn": "id",
        "parent_id": "20361",
        "zero_type": "00000000-0000-0000-0000-000000000000",
        "query_type_template": "uuid.UUID",
        "stream_type_template": "[16]uint8",
        "type_template": "uuid.UUID"
      },
      {
        "column": "camera_id",
        "datatype": "uuid",
        "table": "detection",
        "pos": 12,
        "typeid": "2950",
        "typelen": 16,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": "camera",
        "fcolumn": "id",
        "parent_id": "20361",
        "zero_type": "00000000-0000-0000-0000-000000000000",
        "query_type_template": "uuid.UUID",
        "stream_type_template": "[16]uint8",
        "type_template": "uuid.UUID"
      }
    ]
  },
  "schema_migrations": {
    "tablename": "schema_migrations",
    "oid": "19622",
    "schema": "public",
    "reltuples": -1,
    "relkind": "r",
    "relam": "2",
    "relacl": null,
    "reltype": "19624",
    "relowner": "10",
    "relhasindex": true,
    "columns": [
      {
        "column": "version",
        "datatype": "bigint",
        "table": "schema_migrations",
        "pos": 1,
        "typeid": "20",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": true,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "19622",
        "zero_type": 0,
        "query_type_template": "int64",
        "stream_type_template": "int64",
        "type_template": "int64"
      },
      {
        "column": "dirty",
        "datatype": "boolean",
        "table": "schema_migrations",
        "pos": 2,
        "typeid": "16",
        "typelen": 1,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "19622",
        "zero_type": false,
        "query_type_template": "bool",
        "stream_type_template": "bool",
        "type_template": "bool"
      }
    ]
  },
  "video": {
    "tablename": "video",
    "oid": "20331",
    "schema": "public",
    "reltuples": -1,
    "relkind": "r",
    "relam": "2",
    "relacl": null,
    "reltype": "20333",
    "relowner": "10",
    "relhasindex": true,
    "columns": [
      {
        "column": "id",
        "datatype": "uuid",
        "table": "video",
        "pos": 1,
        "typeid": "2950",
        "typelen": 16,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": true,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "00000000-0000-0000-0000-000000000000",
        "query_type_template": "uuid.UUID",
        "stream_type_template": "[16]uint8",
        "type_template": "uuid.UUID"
      },
      {
        "column": "created_at",
        "datatype": "timestamp with time zone",
        "table": "video",
        "pos": 2,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "updated_at",
        "datatype": "timestamp with time zone",
        "table": "video",
        "pos": 3,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "deleted_at",
        "datatype": "timestamp with time zone",
        "table": "video",
        "pos": 4,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": false,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "file_name",
        "datatype": "text",
        "table": "video",
        "pos": 5,
        "typeid": "25",
        "typelen": -1,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "",
        "query_type_template": "string",
        "stream_type_template": "string",
        "type_template": "string"
      },
      {
        "column": "started_at",
        "datatype": "timestamp with time zone",
        "table": "video",
        "pos": 6,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "ended_at",
        "datatype": "timestamp with time zone",
        "table": "video",
        "pos": 7,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": false,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "duration",
        "datatype": "interval",
        "table": "video",
        "pos": 8,
        "typeid": "1186",
        "typelen": 16,
        "typemod": -1,
        "notnull": false,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": 0,
        "query_type_template": "time.Duration",
        "stream_type_template": "time.Duration",
        "type_template": "time.Duration"
      },
      {
        "column": "file_size",
        "datatype": "double precision",
        "table": "video",
        "pos": 9,
        "typeid": "701",
        "typelen": 8,
        "typemod": -1,
        "notnull": false,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": 0,
        "query_type_template": "float64",
        "stream_type_template": "float64",
        "type_template": "float64"
      },
      {
        "column": "thumbnail_name",
        "datatype": "text",
        "table": "video",
        "pos": 10,
        "typeid": "25",
        "typelen": -1,
        "typemod": -1,
        "notnull": false,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "",
        "query_type_template": "string",
        "stream_type_template": "string",
        "type_template": "string"
      },
      {
        "column": "status",
        "datatype": "text",
        "table": "video",
        "pos": 11,
        "typeid": "25",
        "typelen": -1,
        "typemod": -1,
        "notnull": false,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "",
        "query_type_template": "string",
        "stream_type_template": "string",
        "type_template": "string"
      },
      {
        "column": "object_detector_claimed_until",
        "datatype": "timestamp with time zone",
        "table": "video",
        "pos": 12,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "object_tracker_claimed_until",
        "datatype": "timestamp with time zone",
        "table": "video",
        "pos": 13,
        "typeid": "1184",
        "typelen": 8,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": false,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": "0001-01-01T00:00:00Z",
        "query_type_template": "time.Time",
        "stream_type_template": "time.Time",
        "type_template": "time.Time"
      },
      {
        "column": "camera_id",
        "datatype": "uuid",
        "table": "video",
        "pos": 14,
        "typeid": "2950",
        "typelen": 16,
        "typemod": -1,
        "notnull": true,
        "hasdefault": false,
        "hasmissing": false,
        "ispkey": false,
        "ftable": "camera",
        "fcolumn": "id",
        "parent_id": "20331",
        "zero_type": "00000000-0000-0000-0000-000000000000",
        "query_type_template": "uuid.UUID",
        "stream_type_template": "[16]uint8",
        "type_template": "uuid.UUID"
      },
      {
        "column": "detection_summary",
        "datatype": "jsonb",
        "table": "video",
        "pos": 15,
        "typeid": "3802",
        "typelen": -1,
        "typemod": -1,
        "notnull": true,
        "hasdefault": true,
        "hasmissing": true,
        "ispkey": false,
        "ftable": null,
        "fcolumn": null,
        "parent_id": "20331",
        "zero_type": null,
        "query_type_template": "any",
        "stream_type_template": "any",
        "type_template": "any"
      }
    ]
  }
}`)

var tableByName introspect.TableByName

func init() {
	mu.Lock()
	defer mu.Unlock()

	err := json.Unmarshal(tableByNameAsJSON, &tableByName)
	if err != nil {
		panic(fmt.Errorf("failed to unmarshal tableByNameAsJSON into introspect.TableByName; %v", err))
	}
}

func RunServer(
	ctx context.Context,
	changes chan server.Change,
	addr string,
	db *pgxpool.Pool,
	redisPool *redis.Pool,
	httpMiddlewares []server.HTTPMiddleware,
	objectMiddlewares []server.ObjectMiddleware,
	addCustomHandlers func(chi.Router) error,
) error {
	mu.Lock()
	thisTableByName := tableByName
	mu.Unlock()

	return server.RunServer(ctx, changes, addr, NewFromItem, MutateRouter, db, redisPool, httpMiddlewares, objectMiddlewares, addCustomHandlers, thisTableByName)
}
