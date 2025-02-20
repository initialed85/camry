# FROM golang:1.23-bookworm AS builder

# WORKDIR /srv/

# COPY go.mod /srv/
# COPY go.sum /srv/
# RUN go get ./...

# COPY . /srv/
# RUN mkdir -p ./bin
# RUN CGO_ENABLED=0 go build -o ./bin/segment-producer ./cmd/segment_producer

FROM ubuntu:22.04

ENV TZ Australia/Perth
ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && apt-get install -y \
    tzdata imagemagick x264 libmicrohttpd-dev libjpeg8-dev libavutil-dev libavformat-dev libavcodec-dev libswscale-dev libavdevice-dev ffmpeg

RUN dpkg-reconfigure -f noninteractive tzdata

# COPY --from=builder /srv/bin/segment-producer /srv/bin/segment-producer

COPY ./tmp/segment-producer /srv/bin/segment-producer

RUN mkdir /srv/media

ENV DJANGOLANG_API_ROOT=/api
ENV DESTINATION_PATH=/srv/media
ENV ENABLE_PASSTHROUGH=1

ENTRYPOINT ["/srv/bin/segment-producer"]
CMD []
