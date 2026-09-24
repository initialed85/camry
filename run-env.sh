#!/bin/bash

set -e

usage() {
    cat <<'EOF'
Usage:
  ./run-env.sh                       Start, follow logs, and tear down on exit (legacy)
  ./run-env.sh full                  Same, with the full application profile
  ./run-env.sh up -d [full]           Start in background and leave running
  ./run-env.sh logs [full]            Follow logs without changing lifecycle
  ./run-env.sh down [--volumes]       Stop; preserve volumes unless explicitly requested
EOF
}

mode="foreground"
detach=0
full=0
remove_volumes=0

if [[ $# -gt 0 ]]; then
    case "$1" in
        up|down|logs)
            mode="$1"
            shift
            ;;
        full)
            full=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "error: unknown command '$1'" >&2
            usage >&2
            exit 2
            ;;
    esac
fi

while [[ $# -gt 0 ]]; do
    case "$1" in
        full|--full)
            full=1
            ;;
        -d|--detach)
            detach=1
            ;;
        --volumes)
            remove_volumes=1
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "error: unknown option '$1'" >&2
            usage >&2
            exit 2
            ;;
    esac
    shift
done

compose_profile=()
if [[ "$full" == "1" ]]; then
    compose_profile+=(--profile full)
fi

compose() {
    docker compose "${compose_profile[@]}" "$@"
}

teardown_legacy() {
    docker compose "${compose_profile[@]}" down --remove-orphans --volumes
}

cleanup_on_exit() {
    local exit_status=$?
    if [[ "$mode" == "foreground" ]]; then
        teardown_legacy || true
    elif [[ "$mode" == "up" && "$up_complete" != "1" ]]; then
        # Clean up partial startup failures, but leave a successfully started
        # detached environment running when this script exits.
        docker compose --profile full down --remove-orphans || true
    fi
    return "$exit_status"
}

if [[ "$mode" == "down" ]]; then
    if [[ "$remove_volumes" == "1" ]]; then
        docker compose --profile full down --remove-orphans --volumes
    else
        docker compose --profile full down --remove-orphans
    fi
    exit 0
fi

if [[ "$mode" == "logs" ]]; then
    docker compose --profile full logs -f -t
    exit 0
fi

if [[ "$remove_volumes" == "1" && "$mode" != "down" ]]; then
    echo "error: --volumes is only valid with 'down'" >&2
    exit 2
fi
if [[ "$detach" == "1" && "$mode" != "up" ]]; then
    echo "error: -d/--detach is only valid with 'up'" >&2
    exit 2
fi

up_complete=0
if [[ "$mode" == "foreground" || "$mode" == "up" ]]; then
    trap cleanup_on_exit EXIT

    docker compose pull
    compose build

    if ! docker compose up -d; then
        compose logs -t migrate
        echo "error: docker compose up failed; scroll up for logs" >&2
        exit 1
    fi

    # PostgreSQL must use logical WAL for the generated API's change stream.
    docker compose exec -T postgres psql -U postgres -c 'ALTER SYSTEM SET wal_level = logical;'
    docker compose restart postgres

    DJANGLANG_PROFILE="${DJANGLANG_PROFILE:-0}" compose up -d
    up_complete=1
fi

if [[ "$mode" == "up" && "$detach" == "1" ]]; then
    echo "Compose environment is up. Use './run-env.sh logs' to follow logs and './run-env.sh down' to stop it (volumes are preserved)."
    # Prevent the EXIT trap from cleaning up a successful detached environment.
    trap - EXIT
    exit 0
fi

compose logs -f -t
