#!/bin/bash

cargo build --release

cd BakaRecorder

rm records/empty
sudo cp -r target/release/BakaRecorder /usr/bin/BakaRecorder
