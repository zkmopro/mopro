#!/bin/bash

# Deal with errors
set -euo pipefail

# Color definitions
DEFAULT='\033[0m'
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
GREY='\033[0;90m'

# Coloring the -x output (commands)
DEBUG_COLOR="${DEFAULT}"
trap 'echo -e ${DEBUG_COLOR}${BASH_COMMAND}${DEFAULT}' DEBUG

color_printf() {
    printf "\n${GREEN}$1${DEFAULT}\n"
}

# Assert we're in the project root
if [[ ! -d "mopro-ffi" || ! -d "mopro-core" || ! -d "mopro-ios" ]]; then
    echo -e "${RED}Error: This script must be run from the project root directory that contains mopro-ffi, mopro-core, and mopro-ios folders.${DEFAULT}"
    exit 1
fi

color_printf "[Updating mopro-ffi bindings and library...]"

PROJECT_DIR=$(pwd)
TARGET_DIR=${PROJECT_DIR}/target
MOPROKIT_DIR=${PROJECT_DIR}/mopro-ios/MoproKit

color_printf "[Generating Swift bindings...]"
uniffi-bindgen generate ${PROJECT_DIR}/mopro-ffi/src/mopro_uniffi.udl --language swift --out-dir ${TARGET_DIR}/SwiftBindings

color_printf "[Building mopro-ffi static library...]"
(cd ${PROJECT_DIR}/mopro-ffi && cargo build)

# TODO: Update this to deal with different architectures and environments
color_printf "[Using aarch64-apple-ios-sim static libmopro_ffi.a only...]"
cp ${PROJECT_DIR}/mopro-ffi/target/aarch64-apple-ios-sim/debug/libmopro_ffi.a ${TARGET_DIR}/

color_printf "[Copying Swift bindings and static library to MoproKit...]"
cp ${TARGET_DIR}/SwiftBindings/moproFFI.h ${MOPROKIT_DIR}/Include/
cp ${TARGET_DIR}/SwiftBindings/mopro.swift ${MOPROKIT_DIR}/Bindings/
cp ${TARGET_DIR}/SwiftBindings/moproFFI.modulemap ${MOPROKIT_DIR}/Resources/
cp ${TARGET_DIR}/libmopro_ffi.a ${MOPROKIT_DIR}/Libs/

color_printf "[Done!]"
