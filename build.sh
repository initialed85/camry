#!/bin/bash

set -e

# this block ensures we can invoke this script from anywhere and have it automatically change to this folder first
pushd "$(dirname -- "${BASH_SOURCE[0]}")" >/dev/null 2>&1
function teardown() {
    popd >/dev/null 2>&1 || true
}
trap teardown exit

# ensure we've got a djangolang executable available (required for templating)
if [[ "${FORCE_UPDATE}" == "1" ]] || ! command -v djangolang >/dev/null 2>&1; then
    GOPRIVATE="${GOPRIVATE:-}" go install github.com/initialed85/djangolang@latest
    GOPRIVATE="${GOPRIVATE:-}" go get -u github.com/initialed85/djangolang@latest
fi

# we need oapi-codegen to generate the client for use by Go code
if ! command -v oapi-codegen; then
    go install github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen@main
fi

# we need npm to generate the client for use by the frontend
if ! command -v npm >/dev/null 2>&1; then
    echo "error: can't find npm command- you likely need to install node / npm"
    exit 1
fi

# ensure the docker compose environment is already running
if ! docker compose ps | grep camry | grep postgres | grep healthy >/dev/null 2>&1; then
    echo "error: can't find healthy docker compose environment; ensure to invoke ./run-env.sh in another shell"
    exit 1
fi

# introspect the database and generate the Djangolang API
# note: the environment variables are coupled to the environment described in docker-compose.yaml
echo -e "\generating the api..."
DJANGOLANG_PACKAGE_NAME=api POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR!11 djangolang template

# dump out the OpenAPI v3 schema for the Djangolang API
mkdir -p ./schema
./pkg/api/bin/api dump-openapi-json >./schema/openapi.json

# generate the client for use by the frontend
echo -e "\ngenerating typescript client..."
cd frontend
if [[ "${FORCE_UPDATE}" == "1" ]]; then
    npm ci
fi
npm run openapi-typescript
npm run prettier
cd ..

# generate the client for use by Go code
echo -e "\ngenerating go client..."
mkdir -p ./pkg/api_client
oapi-codegen --generate 'types,client,spec' -package api_client -o ./pkg/api_client/client.go ./schema/openapi.json
go mod tidy
goimports -w .
go get ./...
go fmt ./...
