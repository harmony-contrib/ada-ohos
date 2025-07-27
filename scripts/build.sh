#!/bin/bash

SCRIPT_DIR=$(cd $(dirname $0); pwd)

# Clean the dist directory
rm -rf $SCRIPT_DIR/../dist

release_mode="$1"

if [ "$release_mode" == "release" ]; then
    release_mode="--release"
else
    release_mode=""
fi

echo "Building in $release_mode mode"

# Build the project
cargo zigbuild --target aarch64-unknown-linux-ohos $release_mode
cargo zigbuild --target armv7-unknown-linux-ohos $release_mode
cargo zigbuild --target x86_64-unknown-linux-ohos $release_mode

# Create the dist directory
mkdir -p $SCRIPT_DIR/../dist/arm64-v8a
mkdir -p $SCRIPT_DIR/../dist/armeabi-v7a
mkdir -p $SCRIPT_DIR/../dist/x86_64

target_dir=""

if [ "$release_mode" == "--release" ]; then
    target_dir="release"
else
    target_dir="debug"
fi

# Copy the build output to the dist directory
cp target/aarch64-unknown-linux-ohos/$target_dir/libada_ohos.so $SCRIPT_DIR/../dist/arm64-v8a
cp target/armv7-unknown-linux-ohos/$target_dir/libada_ohos.so $SCRIPT_DIR/../dist/armeabi-v7a
cp target/x86_64-unknown-linux-ohos/$target_dir/libada_ohos.so $SCRIPT_DIR/../dist/x86_64
