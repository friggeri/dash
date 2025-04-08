#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

# Build ios library
cargo build --features ios

cargo run  --features ios --bin uniffi-bindgen generate --library ./target/debug/libdash.dylib --language swift --out-dir ./bindings

cargo build --lib --release --target aarch64-apple-ios --features ios
cargo build --lib --release --target aarch64-apple-ios-sim --features ios

mv ./bindings/dashFFI.modulemap ./bindings/module.modulemap
mv ./bindings/dash.swift ./ios/Dash.swift
rm -rf ./ios/Dash.xcframework
xcodebuild -create-xcframework \
        -library ./target/aarch64-apple-ios-sim/release/libdash.a -headers ./bindings \
        -library ./target/aarch64-apple-ios/release/libdash.a -headers ./bindings \
        -output "ios/Dash.xcframework"
rm -rf ./bindings

# build wasm library
wasm-pack build --target bundler --scope adrien --release --out-dir wasm