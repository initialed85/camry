# ARG BASE_IMAGE
# FROM ${BASE_IMAGE} AS base

# RUN gpg --keyserver keyserver.ubuntu.com --recv-keys A4B469963BF863CC && \
#     gpg --export --armor A4B469963BF863CC | apt-key add - || true

# RUN apt-get update && \
#     apt-get -y install ffmpeg libsm6 libxext6 && \
#     apt-get autoremove -y && \
#     apt-get purge -y && \
#     rm -rf /var/lib/apt/lists/*

# WORKDIR /srv/

# COPY ./object_detector/requirements.txt /srv/
# RUN python3 -m pip install -r requirements.txt --break-system-packages

# COPY object_detector /srv/object_detector
# COPY yolo26n.pt /srv/

# FROM scratch

# COPY --link --from=base / /

FROM initialed85/camry-object-detector:frozen

COPY object_detector /srv/object_detector
COPY yolo26n.pt /srv/

WORKDIR /srv/

ENV SOURCE_PATH=/srv/media

ENTRYPOINT [ "python3", "-u", "-m", "object_detector" ]
CMD []
