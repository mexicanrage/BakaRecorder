#!/bin/bash

cargo build --release

rm records/empty
sudo cp -r target/release/BakaRecorder /usr/bin/BakaRecorder
