# camry

# status: in production at my house for ages now

Intended as the spiritual sucessor to [initialed85/cameranator](https://github.com/initialed85/cameranator), built on
[initialed85/djangolang](https://github.com/initialed85/djangolang)

## Features

- Lots of nice caching etc thanks to [initialed85/djangolang](https://github.com/initialed85/djangolang)
- Frontend is a PWA for a good mobile / offline experience
- Uses YOLO for object detection
- See
  [initialed85/home-ops/tree/master/applications/camry](https://github.com/initialed85/home-ops/tree/master/applications/camry)
  for some inspiration on how to deploy

## Tasks

- [WIP] Rust object detector
  - [TODO] Transcoding on the GPU
- [TODO] Work out how to make cache more effective (the regular camera + video claim stuff means very few cache hits); there's a
  TODO about smarter cache invalidation in Djangolang for this

## Usage

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
devserver --address 0.0.0.0:6060

# shell 6
curl -X POST http://localhost:7070/api/cameras -d '[{"name": "Driveway", "stream_url": "rtsp://192.168.137.31:554/Streaming/Channels/101"}, {"name": "Front door", "stream_url": "rtsp://192.168.137.32:554/Streaming/Channels/101"}, {"name": "Side gate", "stream_url": "rtsp://192.168.137.33:554/Streaming/Channels/101"}]' | jq

# shell 7
DJANGOLANG_API_ROOT=/api DESTINATION_PATH=media ENABLE_PASSTHROUGH=1 POSTGRES_DB=camry POSTGRES_PASSWORD=NoNVR\!11 go run ./cmd segment_producer

# shell 8
DEBUG=1 API_URL=http://localhost:7070 SOURCE_PATH=media ~/.venv/camry/bin/python3 -m object_detector
```

### Scratch

```shell
docker run --rm -it --gpus=all --ipc=host pytorch/pytorch:2.4.0-cuda11.8-cudnn9-runtime python3 -c 'import torch; print([torch.cuda.is_available(), torch.cuda.device_count(), torch.cuda.current_device()])'
```
