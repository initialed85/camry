#!/bin/bash

if ! command -v atlas; then
    curl -sSf https://atlasgo.sh | sh
fi

mkdir -p ./pkg/schema
touch ./pkg/schema/schema.sql
mkdir -p ./pkg/migrations

atlas migrate diff camry \
    --dir "file://./pkg/migrations" \
    --to "file://./pkg/schema/schema.sql" \
    --dev-url "docker://postgres/16"

sqlfluff fix --dialect postgres -f ./pkg/schema/ ./pkg/migrations/

atlas migrate hash --dir file://./pkg/migrations
