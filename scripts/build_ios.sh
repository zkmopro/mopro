#!/bin/bash

PROJECT_DIR=$(pwd)

# Check for the device type argument
if [[ "$1" == "simulator" ]]; then
    DEVICE_TYPE="simulator"
    ARCHITECTURE="aarch64-apple-ios-sim"
elif [[ "$1" == "device" ]]; then
    DEVICE_TYPE="device"
    ARCHITECTURE="aarch64-apple-ios"
else
    echo -e "${RED}Error: Please specify either 'simulator' or 'device' as the first argument.${DEFAULT}"
    exit 1
fi

# Check for the build mode argument
if [[ "$2" == "debug" ]]; then
    BUILD_MODE="debug"
    LIB_DIR="debug"
elif [[ "$2" == "release" ]]; then
    BUILD_MODE="release"
    LIB_DIR="release"
else
    echo -e "${RED}Error: Please specify either 'debug' or 'release' as the second argument.${DEFAULT}"
    exit 1
fi

# build circom circuits in mopro-core

cd ${PROJECT_DIR}/mopro-core/examples/circom/keccak256
npm install
./compile.sh
cd ${PROJECT_DIR}/mopro-core
cargo build --release

# build ffi in mopro-ffi
cd ${PROJECT_DIR}/mopro-ffi
rustup target add x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim
cargo install --bin uniffi-bindgen --path .
make
cargo build --release

# build MoproKit pods
cd ${PROJECT_DIR}/mopro-ios/MoproKit/Example
pod install

# update bindings
cd ${PROJECT_DIR}
./scripts/update_bindings.sh $1 $2

# update xcconfig

MODES="debug release"
XCCONFIG_PATH=mopro-ios/MoproKit/Example/Pods/Target\ Support\ Files/MoproKit
CONFIGS="
    LIBRARY_SEARCH_PATHS=\${SRCROOT}/../../Libs
    OTHER_LDFLAGS=-lmopro_ffi
    USER_HEADER_SEARCH_PATHS=\${SRCROOT}/../../include
"
for mode in ${MODES}
do
    FILE_NAME=${PROJECT_DIR}/${XCCONFIG_PATH}/MoproKit.${mode}.xcconfig
    for config in ${CONFIGS}; do
        EXIST=$(grep -c "${config}" "${FILE_NAME}")
        if [[ $EXIST -eq 0 ]]; then
            echo "${config}" >> "${FILE_NAME}"
        fi
    done
done