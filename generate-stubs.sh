#!/bin/bash

if ! command -v xo; then
    go install github.com/xo/xo@latest
fi

rm -frv ./pkg/models >/dev/null 2>&1 || true
mkdir -p ./pkg/models

xo schema postgres://postgres:camry@localhost:5432/camry?sslmode=disable -o ./pkg/models
