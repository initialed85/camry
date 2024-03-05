#!/bin/bash

if ! command -v atlas; then
    curl -sSf https://atlasgo.sh | sh
fi

atlas migrate apply \
    --url "postgres://postgres:camry@localhost:5432/camry?sslmode=disable" \
    --dir file://./pkg/migrations/
