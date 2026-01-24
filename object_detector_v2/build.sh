#!/bin/bash

export CPATH=/usr/local/cuda-13.1/targets/x86_64-linux/include:${CPATH}

cd /home/edward/Projects/Home/camry/object_detector_v2

cargo build --release
