#!/bin/bash

SCRIPT_DIR=$(cd $(dirname $0); pwd)

# Build the project
cargo zigbuild --target aarch64-unknown-linux-ohos
cargo zigbuild --target armv7-unknown-linux-ohos
cargo zigbuild --target x86_64-unknown-linux-ohos

# Copy the build output to the dist directory
cp target/aarch64-unknown-linux-ohos/release/libada_url.so $SCRIPT_DIR/dist/arm64-v8a
cp target/armv7-unknown-linux-ohos/release/libada_url.so $SCRIPT_DIR/dist/armeabi-v7a
cp target/x86_64-unknown-linux-ohos/release/libada_url.so $SCRIPT_DIR/dist/x86_64
