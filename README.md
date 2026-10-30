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

## Sign-in

Camry has no user database of its own. `AUTH_MODE` picks the credential model:

| `AUTH_MODE`        | Who authenticates                                          | Notes                                                                                     |
| ------------------ | ---------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `sso`              | Traefik `auth-oauth2@kubernetescrd` -> oauth2-proxy -> Google | The cluster gate (home-ops `clusters/{dev,prod}/applications/auth`). `/api/custom/login` answers 410. |
| unset, `PASSWORD` + `TOKEN` set | the legacy fixed password -> fixed bearer token exchange | Kept so the Compose environment and any ungated deployment keep working.                  |
| unset, neither set | nothing                                                    | Auth is off, so local development stays open.                                             |

Under `sso` Traefik authenticates every request to the host - including the `/media/` and HLS requests
a browser cannot attach an `Authorization` header to, which is the hole the old `camry_token` cookie
could not cover. The API then trusts the identity headers Traefik injects (`X-Auth-Request-Email`,
`X-Auth-Request-User`), which a client cannot forge because Traefik deletes client-supplied copies
before injecting its own. Requests with no `X-Forwarded-Host` are Camry's own workloads calling
`http://api:7070` in-cluster: they are trusted by network position, as they always were, and everything
that came through Traefik must show a verified identity, so a misconfigured Ingress fails closed with a
401 instead of quietly becoming public. `ALLOWED_EMAILS` (comma separated) narrows access to a
per-app list; the family allowlist itself belongs to oauth2-proxy, not here.

The Sign in card sends the browser to `https://auth.<env>.initialed85.cc/oauth2/start?rd=<current URL>`
rather than reloading the page, and Camry deliberately ships no working service worker. The Create React
App template's Workbox worker precached every build asset and answered all navigation requests with the
 cached `index.html` - and a navigation answered from cache never reaches Traefik, so the gate never gets
a request to redirect, while the SPA's `/api` and `/media` requests do hit the network and do get 302'd
to Google. That is what a browser stuck on the precached shell looks like: a sign-in card a reload
cannot escape, because the reload is answered from cache again. `src/service-worker.ts` is now a
retirement stub (drop the caches, unregister, register no fetch handler), `index.tsx` sweeps up old
registrations and caches, and nginx serves the SPA shell and `/service-worker.js` as `no-store` with the
content-hashed `/static/` assets `immutable`. Chrome update-checks `/service-worker.js` on navigation,
which is what frees a browser that is still holding the old worker.

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
