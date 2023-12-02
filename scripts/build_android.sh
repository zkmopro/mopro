#!/bin/bash

PROJECT_DIR=$(pwd)

# Color definitions
DEFAULT='\033[0m'
RED='\033[0;31m'

cd ${PROJECT_DIR}/mopro-ffi

cargo build --lib \
--target x86_64-linux-android \
--target i686-linux-android \
--target armv7-linux-androideabi \
--target aarch64-linux-android

for binary in target/*/*/libmopro_ffi.so; do file $binary; done

mkdir -p jniLibs/arm64-v8a/ && \
cp target/aarch64-linux-android/debug/libmopro_ffi.so jniLibs/arm64-v8a/libuniffi_mopro.so && \
mkdir -p jniLibs/armeabi-v7a/ && \
cp target/armv7-linux-androideabi/debug/libmopro_ffi.so jniLibs/armeabi-v7a/libuniffi_mopro.so && \
mkdir -p jniLibs/x86/ && \
cp target/i686-linux-android/debug/libmopro_ffi.so jniLibs/x86/libuniffi_mopro.so && \
mkdir -p jniLibs/x86_64/ && \
cp target/x86_64-linux-android/debug/libmopro_ffi.so jniLibs/x86_64/libuniffi_mopro.so

cargo run --features=uniffi/cli \
    --bin uniffi-bindgen \
    generate src/mopro.udl \
    --language kotlin

cp -r ${PROJECT_DIR}/mopro-ffi/jniLibs/ ${PROJECT_DIR}/mopro-android/Example/app/src/main/jniLibs/
cp -r ${PROJECT_DIR}/mopro-ffi/src/uniffi/ ${PROJECT_DIR}/mopro-android/Example/app/src/main/java/uniffi/