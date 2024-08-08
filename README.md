# camry

# status: not yet working

Intended as the spiritual sucessor to [initialed85/cameranator](https://github.com/initialed85/cameranator), build on [initialed85/djangolang](https://github.com/initialed85/djangolang)

## Usage

```shell
# shell 1
./run-env.sh

# shell 2
./build.sh

# shell 3
POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR\!11 go run ./cmd/ serve

# shell 4
websocat ws://localhost:7070/__stream | jq

# shell 5
curl -X POST http://localhost:7070/cameras -d '[{"name": "Driveway", "stream_url": "rtsp://192.168.137.31:554/Streaming/Channels/101"}, {"name": "Front door", "stream_url": "rtsp://192.168.137.32:554/Streaming/Channels/101"}, {"name": "Side gate", "stream_url": "rtsp://192.168.137.33:554/Streaming/Channels/101"}]' | jq

# shell 6
PAGER=cat PGPASSWORD=NoNVR\!11 psql -h localhost -p 5432 -U postgres camry -c 'TRUNCATE TABLE video CASCADE;'

# shell 7
DESTINATION_PATH=media ENABLE_PASSTHROUGH=1 CAMERA_NAME='Driveway' NET_CAM_URL=rtsp://192.168.137.31:554/Streaming/Channels/101 POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR\!11 go run ./cmd segment_producer

# shell 8
DESTINATION_PATH=media ENABLE_PASSTHROUGH=1 CAMERA_NAME='Front door' NET_CAM_URL=rtsp://192.168.137.32:554/Streaming/Channels/101 POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR\!11 go run ./cmd segment_producer

# shell 9
DESTINATION_PATH=media ENABLE_PASSTHROUGH=1 CAMERA_NAME='Side gate' NET_CAM_URL=rtsp://192.168.137.33:554/Streaming/Channels/101 POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR\!11 go run ./cmd segment_producer
```

## Tasks

- [TODO] Add file size column + calculation
