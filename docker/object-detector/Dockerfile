ARG BASE_IMAGE
FROM ${BASE_IMAGE}

RUN apt-get update && apt-get install ffmpeg libsm6 libxext6  -y

WORKDIR /srv/

COPY ./object_detector/requirements.txt /srv/
RUN python3 -m pip install -r requirements.txt

COPY object_detector /srv/object_detector
COPY yolov8n.pt /srv/

ENV SOURCE_PATH=/srv/media

ENTRYPOINT [ "python3", "-m", "object_detector" ]
CMD []
