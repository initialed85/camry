#!/bin/bash

set -e

# this block ensures we can invoke this script from anywhere and have it automatically change to this folder first
pushd "$(dirname -- "${BASH_SOURCE[0]}")" >/dev/null 2>&1
function teardown() {
    popd >/dev/null 2>&1 || true
}
trap teardown exit

# we need migrate to run the migrations
if ! command -v migrate >/dev/null 2>&1; then
    go install -tags 'postgres' github.com/golang-migrate/migrate/v4/cmd/migrate@latest
fi

POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-NoNVR!11}"
POSTGRES_HOST="${POSTGRES_HOST:-localhost}"
POSTGRES_PORT="${POSTGRES_PORT:-5432}"
POSTGRES_DB="${POSTGRES_DB:-camry}"

migrate \
    --source file://./database/migrations \
    --database "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}?sslmode=disable" \
    up
