FROM golang:1.21-bookworm AS builder

WORKDIR /srv/

COPY go.mod /srv/
COPY go.sum /srv/
RUN go get ./...

COPY . /srv/
RUN mkdir -p ./bin
RUN CGO_ENABLED=0 go build -o ./bin/api ./cmd/api

FROM ubuntu:22.04

ENV TZ Australia/Perth
ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && apt-get install -y \
    tzdata curl

RUN dpkg-reconfigure -f noninteractive tzdata

WORKDIR /srv/

COPY --from=builder /srv/bin/api /srv/bin/api

ENV DJANGOLANG_API_ROOT=/api

HEALTHCHECK --interval=5s --timeout=5s --start-period=5s --retries=3 CMD [ "curl", "http://localhost:7070/api/healthz" ]
ENTRYPOINT ["/srv/bin/api"]
CMD []
