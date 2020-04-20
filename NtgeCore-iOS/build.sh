#!/bin/bash

set -ex

BASEPATH="${PWD}"
echo "Building rust library..."

cd ../ntge-core
cargo build --target x86_64-apple-ios --release --lib
RUSTFLAGS="-Z embed-bitcode" cargo +ios-arm64 build --target aarch64-apple-ios --release --lib

echo "lipo bitcode lib"
cd ..
echo $(PWD)
lipo -create target/aarch64-apple-ios/release/libntge_core.a target/x86_64-apple-ios/release/libntge_core.a -output target/libntge_core.a

cd "${BASEPATH}"
mkdir -p lib
cp ../target/*.a lib/
rm -rf NtgeCore/Classes/include
mkdir -p NtgeCore/Classes/include
cp -r ../ntge-core/include NtgeCore/Classes/