# FROM golang:1.26-bookworm AS builder

# WORKDIR /srv/

# COPY go.mod /srv/
# COPY go.sum /srv/
# RUN go get ./...

# COPY . /srv/
# RUN mkdir -p ./bin
# RUN CGO_ENABLED=0 go build -o ./bin/audio-detector ./cmd/audio_detector

FROM ubuntu:22.04

ENV TZ Australia/Perth
ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && apt-get install -y \
    tzdata ffmpeg

RUN dpkg-reconfigure -f noninteractive tzdata

# COPY --from=builder /srv/bin/audio-detector /srv/bin/audio-detector

COPY ./tmp/audio-detector /srv/bin/audio-detector

RUN mkdir /srv/media

ENV DJANGOLANG_API_ROOT=/api
ENV DESTINATION_PATH=/srv/media

ENTRYPOINT ["/srv/bin/audio-detector"]
CMD []
