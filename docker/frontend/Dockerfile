FROM node:lts-bookworm-slim AS builder

WORKDIR /srv/

COPY frontend/package.json /srv/
COPY frontend/package-lock.json /srv/
RUN npm ci

COPY frontend/ /srv/
RUN npm run build

FROM nginx:stable-bookworm

COPY --from=builder /srv/build/ /usr/share/nginx/html/

COPY docker/frontend/default.conf /etc/nginx/conf.d/default.conf

RUN mkdir -p /srv/media
