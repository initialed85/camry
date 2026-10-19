package cache

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/gomodule/redigo/redis"
	"github.com/initialed85/camry/internal"
	"github.com/initialed85/djangolang/pkg/config"
	"github.com/jackc/pgx/v5/pgxpool"
)

// redisStore is the production Store.
type redisStore struct {
	pool *redis.Pool
}

func (s *redisStore) Get(key string) ([]byte, error) {

	conn := s.pool.Get()
	defer conn.Close()

	return redis.Bytes(conn.Do("GET", key))
}

func (s *redisStore) Set(key string, value []byte, ttl time.Duration) error {

	conn := s.pool.Get()
	defer conn.Close()

	_, err := conn.Do("SET", key, value, "PX", ttl.Milliseconds())

	return err
}

// statusOf reads a video row's status, which is what decides the TTL.
func statusOf(pool *pgxpool.Pool) StatusFunc {

	return func(ctx context.Context, videoID string) (string, error) {

		var status string

		err := pool.QueryRow(ctx, "select status from video where id = $1", videoID).Scan(&status)
		if err != nil {
			return "", err
		}

		return status, nil
	}
}

// New wires the middleware with its own Redis and database pools: it runs as HTTP middleware, above
// the request handlers, so it has no access to the connections djangolang hands them. The database
// pool only ever serves the one indexed status read on a miss.
func New() (func(http.Handler) http.Handler, error) {

	redisPool, err := config.GetRedisFromEnvironment()
	if err != nil {
		return nil, fmt.Errorf("failed to connect to redis for the response cache; %v", err)
	}

	dbPool, err := internal.GetDBFromEnvironment(context.Background())
	if err != nil {
		return nil, fmt.Errorf("failed to connect to postgres for the response cache; %v", err)
	}

	opts := DefaultOptions()

	Log(opts)

	return Middleware(&redisStore{pool: redisPool}, statusOf(dbPool), opts), nil
}
