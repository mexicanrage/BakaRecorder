#!/bin/bash

cargo build --release

sudo cp -r target/release/BakaRecorder /usr/bin/BakaRecorder
