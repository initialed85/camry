# camry

# status: not yet working

Intended as the spiritual sucessor to [initialed85/cameranator](https://github.com/initialed85/cameranator), built on [initialed85/djangolang](https://github.com/initialed85/djangolang)

## Tasks

- [TODO] Come up with a more efficient way to use the detections data, probably some sort of aggregation

## Usage

### Testing

```shell
./run-env.sh full
```

### Development

```shell
# shell 1
./run-env.sh

# shell 2
./build.sh

# shell 3
REDIS_URL=redis://localhost:6379 DJANGOLANG_API_ROOT=/api POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR\!11 go run ./cmd/ serve

# shell 4
websocat -B 1048576 ws://localhost:7070/api/__stream | jq

# shell 5
curl -X POST http://localhost:7070/api/cameras -d '[{"name": "Driveway", "stream_url": "rtsp://192.168.137.31:554/Streaming/Channels/101"}, {"name": "Front door", "stream_url": "rtsp://192.168.137.32:554/Streaming/Channels/101"}, {"name": "Side gate", "stream_url": "rtsp://192.168.137.33:554/Streaming/Channels/101"}]' | jq

# shell 6
DJANGOLANG_API_ROOT=/api DESTINATION_PATH=media ENABLE_PASSTHROUGH=1 CAMERA_NAME='Driveway' NET_CAM_URL=rtsp://192.168.137.31:554/Streaming/Channels/101 POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR\!11 go run ./cmd segment_producer

# shell 7
DJANGOLANG_API_ROOT=/api DESTINATION_PATH=media ENABLE_PASSTHROUGH=1 CAMERA_NAME='Front door' NET_CAM_URL=rtsp://192.168.137.32:554/Streaming/Channels/101 POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR\!11 go run ./cmd segment_producer

# shell 8
DJANGOLANG_API_ROOT=/api DESTINATION_PATH=media ENABLE_PASSTHROUGH=1 CAMERA_NAME='Side gate' NET_CAM_URL=rtsp://192.168.137.33:554/Streaming/Channels/101 POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR\!11 go run ./cmd segment_producer
```

### Scratch

```shell
docker run --rm -it --gpus=all --ipc=host pytorch/pytorch:2.4.0-cuda11.8-cudnn9-runtime python3 -c 'import torch; print([torch.cuda.is_available(), torch.cuda.device_count(), torch.cuda.current_device()])'
```
