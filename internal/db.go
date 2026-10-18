package internal

import (
	"context"
	"time"

	"github.com/initialed85/djangolang/pkg/config"
	"github.com/jackc/pgx/v5/pgxpool"
)

// GetDBLimits reads the pool size knobs. djangolang hardcodes 5 idle and 500
// open connections per process, and every camry worker calls
// config.GetDBFromEnvironment, so a dozen audio workers hold sixty idle
// connections against a Postgres with max_connections = 100.
func GetDBLimits() (int, int, error) {
	maxIdleConns, err := GetEnvironment("POSTGRES_MAX_IDLE_CONNS", false, Ptr(5))
	if err != nil {
		return 0, 0, err
	}

	maxOpenConns, err := GetEnvironment("POSTGRES_MAX_OPEN_CONNS", false, Ptr(500))
	if err != nil {
		return 0, 0, err
	}

	return maxIdleConns, maxOpenConns, nil
}

// GetDBFromEnvironment is djangolang's pool with a tunable size.
func GetDBFromEnvironment(ctx context.Context) (*pgxpool.Pool, error) {
	maxIdleConns, maxOpenConns, err := GetDBLimits()
	if err != nil {
		return nil, err
	}

	return config.GetDB(ctx, config.GetDSN(), maxIdleConns, maxOpenConns, time.Second*30, time.Second*600)
}
