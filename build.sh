#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

cargo build --release --target aarch64-apple-ios
cargo build --release --target x86_64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
mkdir -p ./target/universal-ios/release
lipo \
    ./target/aarch64-apple-ios-sim/release/libdash.a \
    ./target/x86_64-apple-ios/release/libdash.a -create -output \
    ./target/universal-ios/release/libdash.a

swift-bridge-cli create-package \
  --bridges-dir ./generated \
  --out-dir . \
  --ios target/aarch64-apple-ios/release/libdash.a \
  --simulator target/universal-ios/release/libdash.a \
  --name Dash

wasm-pack build --target bundler --scope adrien --release