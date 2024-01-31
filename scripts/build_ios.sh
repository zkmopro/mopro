#!/bin/bash

# Source the script prelude
source "scripts/_prelude.sh"

# Check if a configuration file was passed as an argument
if [ "$#" -ne 1 ]; then
    echo -e "\n${RED}Usage: $0 path/to/config.toml${DEFAULT}"
    exit 1
fi

# Read the path to the TOML configuration file from the first argument
CONFIG_FILE="$1"

# Export the configuration file path as an environment variable
export BUILD_CONFIG_PATH="$(pwd)/$CONFIG_FILE"

# Print which configuration file is being used
echo "Using configuration file: $CONFIG_FILE"

# Read configurations from TOML file within [build] block
DEVICE_TYPE=$(read_toml "$CONFIG_FILE" "build.device_type")
BUILD_MODE=$(read_toml "$CONFIG_FILE" "build.build_mode")

# Determine the architecture based on device type
case $DEVICE_TYPE in
    "x86_64")
        ARCHITECTURE="x86_64-apple-ios"
        ;;
    "simulator")
        ARCHITECTURE="aarch64-apple-ios-sim"
        ;;
    "device")
        ARCHITECTURE="aarch64-apple-ios"
        ;;
    *)
        echo -e "\n${RED}Error: Invalid device type specified in config: $DEVICE_TYPE${DEFAULT}"
        exit 1
        ;;
esac

# Determine the library directory based on build mode
case $BUILD_MODE in
    "debug")
        LIB_DIR="debug"
        ;;
    "release")
        LIB_DIR="release"
        ;;
    *)
        echo -e "\n${RED}Error: Invalid build mode specified in config: $BUILD_MODE${DEFAULT}"
        exit 1
        ;;
esac

PROJECT_DIR=$(pwd)

# Build circom circuits in mopro-core
cd "${PROJECT_DIR}/mopro-core"
if [[ "$BUILD_MODE" == "debug" ]]; then
    cargo build
    elif [[ "$BUILD_MODE" == "release" ]]; then
    cargo build --release
fi

# Build MoproKit pods
cd "${PROJECT_DIR}/mopro-ios/MoproKit/Example"
pod install

# Update bindings
cd "${PROJECT_DIR}"
./scripts/update_bindings.sh $CONFIG_FILE

# Update xcconfig
MODES="debug release"
XCCONFIG_PATH=mopro-ios/MoproKit/Example/Pods/Target\ Support\ Files/MoproKit
CONFIGS="
LIBRARY_SEARCH_PATHS=\${SRCROOT}/../../Libs
OTHER_LDFLAGS=-lmopro_ffi
USER_HEADER_SEARCH_PATHS=\${SRCROOT}/../../include
"
for mode in $MODES; do
    FILE_NAME="${PROJECT_DIR}/${XCCONFIG_PATH}/MoproKit.${mode}.xcconfig"
    for config in $CONFIGS; do
        if ! grep -q "${config}" "${FILE_NAME}"; then
            echo "${config}" >> "${FILE_NAME}"
        fi
    done
done