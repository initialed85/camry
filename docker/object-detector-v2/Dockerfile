ARG BASE_IMAGE
FROM ${BASE_IMAGE}

RUN gpg --keyserver keyserver.ubuntu.com --recv-keys A4B469963BF863CC && \
    gpg --export --armor A4B469963BF863CC | apt-key add - || true

RUN apt-get update && apt-get install -y ffmpeg libsm6 libxext6 curl \
    clang pkg-config

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install.sh && \
    chmod +x install.sh && \
    ./install.sh -y && rm -frv install.sh
ENV PATH=${PATH}:/root/.cargo/bin/
RUN rustup update

WORKDIR /srv/

COPY ./object_detector_v2/Cargo.toml /srv/
COPY ./object_detector_v2/Cargo.lock /srv/
COPY ./object_detector_v2/.cargo /srv/

RUN mkdir -p /srv/src
RUN touch /srv/src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo fetch

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

RUN apt-get install -y clang libclang-dev
RUN apt-get install -y libavfilter-dev libavdevice-dev

RUN ln -s /usr/local/cuda-11.8 /usr/local/cuda

ENV CUDA_PATH="/usr/local/cuda"
ENV CUDA_HOME="/usr/local/cuda"
ENV CUDACXX="/usr/local/cuda/bin/nvcc"
ENV PATH="/usr/local/cuda/bin:$PATH"
ENV LD_LIBRARY_PATH="/usr/local/cuda/lib64:$LD_LIBRARY_PATH"
ENV CPATH="/usr/local/cuda/include:$CPATH"
ENV CPLUS_INCLUDE_PATH="/usr/local/cuda/include:$CPLUS_INCLUDE_PATH"
ENV C_INCLUDE_PATH="/usr/local/cuda/include:$C_INCLUDE_PATH"

# RUN apt install -y lsb-release wget software-properties-common gnupg
# RUN add-apt-repository ppa:ubuntu-toolchain-r/test
# RUN apt update && apt upgrade -y
# RUN wget -qO- https://apt.llvm.org/llvm.sh | bash -s -- 14

RUN apt-get install -y wget gnupg software-properties-common
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN add-apt-repository "deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-11 main"
RUN apt-get update && apt-get install -y clang-11 libclang-11-dev

# Point clang-sys at the new version
ENV LIBCLANG_PATH=/usr/lib/llvm-11/lib

RUN bash -c 'if ! python3 --version | grep "3.11"; then apt-get install -y python3.7; else true; fi'
RUN bash -c 'if ! python3 --version | grep "3.11"; then update-alternatives --install $(which python3) python3 $(which python3.7) 1; else true; fi'

COPY ./object_detector_v2/ /srv/
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

ENV SOURCE_PATH=/srv/media

ENTRYPOINT [ "target/release/object-detector" ]
CMD []
