#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
cargo build --target aarch64-apple-ios-sim
mkdir -p ./target/universal-ios/debug
lipo \
    ./target/aarch64-apple-ios-sim/debug/libdash.a \
    ./target/x86_64-apple-ios/debug/libdash.a -create -output \
    ./target/universal-ios/debug/libdash.a

swift-bridge-cli create-package \
  --bridges-dir ./generated \
  --out-dir . \
  --ios target/aarch64-apple-ios/debug/libdash.a \
  --simulator target/universal-ios/debug/libdash.a \
  --name Dash

wasm-pack build --target bundler