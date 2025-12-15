#!/bin/bash

(return 0 2>/dev/null) && sourced=1 || sourced=0

if [[ "${sourced}" == "0" ]]; then
	set -e
fi

PKG_CONFIG_PATH="$(brew --prefix ffmpeg)/lib/pkgconfig:$PKG_CONFIG_PATH"
PKG_CONFIG_PATH="$(brew --prefix opencv)/lib/pkgconfig:$PKG_CONFIG_PATH"
export PKG_CONFIG_PATH

if [[ "${sourced}" == "0" ]]; then
	if [[ "${DEBUG}" == "1" ]]; then
		cargo build
	else
		cargo build --release
	fi
fi
