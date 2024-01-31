#!/bin/bash

# Source the script prelude
source "scripts/_prelude.sh"

# Check for the device type argument
if [[ "$1" == "x86_64" ]]; then
    ARCHITECTURE="x86_64-linux-android"
    FOLDER="x86_64"
    elif [[ "$1" == "x86" ]]; then
    ARCHITECTURE="i686-linux-android"
    FOLDER="x86"
    elif [[ "$1" == "arm" ]]; then
    ARCHITECTURE="armv7-linux-androideabi"
    FOLDER="armeabi-v7a"
    elif [[ "$1" == "arm64" ]]; then
    ARCHITECTURE="aarch64-linux-android"
    FOLDER="arm64-v8a"
else
    echo -e "${RED}Error: Please specify either 'x86_64', 'x86', 'arm' or 'arm64' as the first argument.${DEFAULT}"
    exit 1
fi

# Check for the build mode argument
if [[ "$2" == "debug" ]]; then
    BUILD_MODE="debug"
    LIB_DIR="debug"
    COMMAND=""
    elif [[ "$2" == "release" ]]; then
    BUILD_MODE="release"
    LIB_DIR="release"
    COMMAND="--release"
else
    echo -e "${RED}Error: Please specify either 'debug' or 'release' as the second argument.${DEFAULT}"
    exit 1
fi

PROJECT_DIR=$(pwd)
cd ${PROJECT_DIR}/mopro-ffi

# Print appropriate message based on device type
print_action "Using $ARCHITECTURE libmopro_ffi.a ($LIB_DIR) static library..."
print_warning "This only works on $FOLDER devices!"

print_action "[android] Build target in $BUILD_MODE mode"
cargo build --lib ${COMMAND} --target ${ARCHITECTURE}

print_action "[android] Copy files in mopro-android/Example/jniLibs/"
for binary in target/*/*/libmopro_ffi.so; do file $binary; done

mkdir -p jniLibs/${FOLDER}/ && \
cp target/${ARCHITECTURE}/${LIB_DIR}/libmopro_ffi.so jniLibs/${FOLDER}/libuniffi_mopro.so

print_action "[android] Generating Kotlin bindings in $BUILD_MODE mode..."
cargo run --features=uniffi/cli ${COMMAND} \
--bin uniffi-bindgen \
generate src/mopro.udl \
--language kotlin

print_action "[android] Copy Kotlin bindings to mopro-android/Example"
cp -r ${PROJECT_DIR}/mopro-ffi/jniLibs/ ${PROJECT_DIR}/mopro-android/Example/app/src/main/jniLibs/
cp -r ${PROJECT_DIR}/mopro-ffi/src/uniffi/ ${PROJECT_DIR}/mopro-android/Example/app/src/main/java/uniffi/
