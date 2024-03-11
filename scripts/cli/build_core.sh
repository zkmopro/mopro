#!/bin/bash

# Check if MOPRO_ROOT is set
if [ -z "$MOPRO_ROOT" ]; then
    echo "MOPRO_ROOT is not set. Please set it to your local mopro repository."
    exit 1
fi

# Source the script prelude
source "${MOPRO_ROOT}/scripts/_prelude.sh"

# Check if toml-cli is installed
if ! command -v toml &> /dev/null; then
    echo -e "${RED}toml (toml-cli) is not installed. Please install it to continue.${DEFAULT}"
    exit 1
fi

# Function to read value from TOML file and remove quotes
read_toml() {
    toml get "$1" "$2" | tr -d '"'
}

# Check if a configuration file was passed as an argument
if [ "$#" -ne 1 ]; then
    echo -e "\n${RED}Usage: $0 path/to/config.toml${DEFAULT}"
    exit 1
fi

# Read the path to the TOML configuration file from the first argument
CONFIG_FILE="$1"

# XXX: This isn't necessarily propagated to `cargo build` build process,
# so we pass it explicitly. Consider using `source` instead of `export`.
# Export the configuration file path as an environment variable
export BUILD_CONFIG_PATH="$(pwd)/$CONFIG_FILE"

# Print which configuration file is being used
echo "Using build configuration file: $BUILD_CONFIG_PATH"

# Read configurations from TOML file within [build] block
BUILD_MODE=$(read_toml "$CONFIG_FILE" "build.build_mode")

# XXX: This is currently not used, need to pass it to `cargo build` explicitly.
PROJECT_DIR=$(pwd)

# Build circom circuits in mopro-core
cd "${MOPRO_ROOT}/mopro-core"
if [[ "$BUILD_MODE" == "debug" ]]; then
    env BUILD_CONFIG_PATH="$BUILD_CONFIG_PATH" cargo build
    elif [[ "$BUILD_MODE" == "release" ]]; then
    env BUILD_CONFIG_PATH="$BUILD_CONFIG_PATH" cargo build --release
fi