#!/bin/bash

PROJECT_DIR=$(pwd)

# Color definitions
DEFAULT='\033[0m'
RED='\033[0;31m'

# Check for the device type argument
if [[ "$1" == "x86_64" ]]; then
    DEVICE_TYPE="x86_64"
    ARCHITECTURE="x86_64-apple-ios"
elif [[ "$1" == "simulator" ]]; then
    DEVICE_TYPE="simulator"
    ARCHITECTURE="aarch64-apple-ios-sim"
elif [[ "$1" == "device" ]]; then
    DEVICE_TYPE="device"
    ARCHITECTURE="aarch64-apple-ios"
else
    echo -e "${RED}Error: Please specify either 'x86_64', 'simulator' or 'device' as the first argument.${DEFAULT}"
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
cd ${PROJECT_DIR}/mopro-core
if [[ "$BUILD_MODE" == "debug" ]]; then
    cargo build
elif [[ "$BUILD_MODE" == "release" ]]; then
    cargo build --release
fi

# build MoproKit pods
cd ${PROJECT_DIR}/mopro-ios/MoproKit/Example
pod install

# update bindings
cd ${PROJECT_DIR}
./scripts/update_bindings.sh ${DEVICE_TYPE} ${BUILD_MODE}

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
