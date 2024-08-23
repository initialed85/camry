#!/bin/bash

set -e

profile=""
if [[ "${1}" == "full" ]]; then
    profile="--profile full"
fi

function teardown() {
    # shellcheck disable=SC2086
    docker compose ${profile} down --remove-orphans --volumes
}

trap teardown exit

# shellcheck disable=SC2086
docker compose pull

# shellcheck disable=SC2086
docker compose ${profile} build

if ! docker compose up -d; then
    # shellcheck disable=SC2086
    docker compose ${profile} logs -t
    echo "error: docker compose up failed; scroll up for logs"
    exit 1
fi

docker compose exec -it postgres psql -U postgres -c 'ALTER SYSTEM SET wal_level = logical;'

# docker compose exec -it postgres psql -U postgres -c "ALTER DATABASE camry SET log_statement = 'all';"

docker compose restart postgres

# shellcheck disable=SC2086
DJANGOLANG_PROFILE="${DJANGOLANG_PROFILE:-0}" docker compose ${profile} up -d

# shellcheck disable=SC2086
docker compose ${profile} logs -f -t
