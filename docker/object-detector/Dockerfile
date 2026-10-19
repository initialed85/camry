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

COPY yolo26n.pt /srv/

RUN python3 -m pip install onnxruntime==1.31.0 onnx onnxslim --break-system-packages

# The ONNX graph is exported here rather than copied from the build context:
# CPU inference is ~16 ms/frame against ~24 ms/frame through PyTorch.
RUN python3 -c "from ultralytics import YOLO; YOLO('/srv/yolo26n.pt').export(format='onnx', dynamic=True, simplify=True, imgsz=640)"

# Last, so a code-only change does not invalidate the pip install and the export
# above it; only the weights invalidate them.
COPY object_detector /srv/object_detector

WORKDIR /srv/

ENV SOURCE_PATH=/srv/media
ENV MODEL_PATH=/srv/yolo26n.onnx

ENTRYPOINT [ "python3", "-u", "-m", "object_detector" ]
CMD []
