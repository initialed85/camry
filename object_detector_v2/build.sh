#!/bin/bash

set -e

PKG_CONFIG_PATH="$(brew --prefix ffmpeg)/lib/pkgconfig:$PKG_CONFIG_PATH"
PKG_CONFIG_PATH="$(brew --prefix opencv)/lib/pkgconfig:$PKG_CONFIG_PATH"
export PKG_CONFIG_PATH

if [[ "${DEBUG}" == "1" ]]; then
	cargo build
else
	cargo build --release
fi
