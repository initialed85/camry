#!/bin/bash

# TODO
# export CPATH=/usr/local/cuda-13.1/targets/x86_64-linux/include:${CPATH}
# cd /home/edward/Projects/Home/camry/object_detector_v2

PKG_CONFIG_PATH="$(brew --prefix ffmpeg)/lib/pkgconfig:$PKG_CONFIG_PATH"
PKG_CONFIG_PATH="$(brew --prefix opencv)/lib/pkgconfig:$PKG_CONFIG_PATH"
export PKG_CONFIG_PATH

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
	cargo build --release
fi
