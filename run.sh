#!/bin/bash

set -e -m

function cleanup() {
    pkill -SIGINT -f segment_producer || true
    docker compose down --remove-orphans --volumes || true
}
trap cleanup EXIT

docker compose up --build -d

# DURATION_SECONDS=30 CAMERA_NAME=Driveway NET_CAM_URL=rtsp://192.168.137.31:554/Streaming/Channels/101/ go run ./cmd segment_producer &
# DURATION_SECONDS=30 CAMERA_NAME=FrontDoor NET_CAM_URL=rtsp://192.168.137.32:554/Streaming/Channels/101/ go run ./cmd segment_producer &
# DURATION_SECONDS=30 CAMERA_NAME=SideGate NET_CAM_URL=rtsp://192.168.137.33:554/Streaming/Channels/101/ go run ./cmd segment_producer &

read -r -d '' _ </dev/tty
