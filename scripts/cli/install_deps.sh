#!/bin/bash

# Script for installing required depenedencies for the project.

# Prelude
#----------------------------------------------------------------------------
initialize_environment() {
    if [ -z "$MOPRO_ROOT" ]; then
        echo "MOPRO_ROOT is not set. Please set it to your local mopro repository."
        exit 1
    fi

    source "${MOPRO_ROOT}/scripts/_prelude.sh"
}

# Define target architectures
TARGETS=(
    x86_64-apple-ios
    aarch64-apple-ios
    aarch64-apple-ios-sim
    aarch64-linux-android
    armv7-linux-androideabi
    i686-linux-android
    x86_64-linux-android
)

# Check for target support and add if necessary
#----------------------------------------------------------------------------
add_target_support() {
    local target
    for target in "${TARGETS[@]}"; do
        if ! rustup target list | grep installed | grep -q "$target"; then
            rustup target add "$target"
        else
            echo "Target $target already installed, skipping."
        fi
    done
}

# Download circuit artifacts
#----------------------------------------------------------------------------

# XXX: This is a bit of a hack, just to get mopro-core to compile
# This happens when we run mopro deps. We can probably improve on this.
download_files() {
    local url="https://mopro.vivianjeng.xyz"
    local dir=$1
    local circuit=$2
    local circuit_dir="${MOPRO_ROOT}/mopro-core/examples/circom"
    local target_dir="${circuit_dir}/${dir}/target"
    local js_target_dir="${target_dir}/${circuit}_js"

    # Create directories if they don't exist
    mkdir -p "$target_dir" "$js_target_dir"

    # Check if file exists
    # Download files to the specified directories
    if ! [ -f "${target_dir}/${circuit}_final.arkzkey" ]; then
        wget -P "$target_dir" "${url}/${circuit}_final.arkzkey"
    else
        echo "File ${circuit}_final.arkzkey already exists, skipping download."
    fi

    if ! [ -f "${target_dir}/${circuit}_final.zkey" ]; then
        wget -P "$target_dir" "${url}/${circuit}_final.zkey"
    else
        echo "File ${circuit}_final.zkey already exists, skipping download."
    fi

    if ! [ -f "${js_target_dir}/${circuit}.wasm" ]; then
        wget -P "$js_target_dir" "${url}/${circuit}.wasm"
    else
        echo "File ${circuit}.wasm already exists, skipping download."
    fi
}

# Check uniffi-bindgen version
#----------------------------------------------------------------------------
check_uniffi_bindgen_version() {
    local UNIFFI_VERSION
    UNIFFI_VERSION=$(uniffi-bindgen --version | grep -oE '0\.25\.[0-9]+' || echo "not found")
    local EXPECTED_VERSION_PREFIX="0.25"
    if [[ $UNIFFI_VERSION != $EXPECTED_VERSION_PREFIX* ]]; then
        echo -e "${RED}Error: uniffi-bindgen version is not 0.25.x. Current version: $(uniffi-bindgen --version)${DEFAULT}"
        echo -e "${RED}Please uninstall uniffi-bindgen and run this script again.${DEFAULT}"
        exit 1
    else
        echo "uniffi-bindgen version is $UNIFFI_VERSION, which is acceptable."
    fi
}


# Install required binaries
#----------------------------------------------------------------------------
install_binary() {
    local bin_path=$1
    local bin_name=$2
    local cargo_path=$3

    cd "$bin_path"
    if ! command -v "$bin_name" &> /dev/null; then
        cargo install --bin "$bin_name" --path "$cargo_path"
    else
        echo "$bin_name already installed, skipping."
    fi
}

install_remote_binary() {
    local bin_path=$1
    local bin_name=$2

    if ! command -v "$bin_name" &> /dev/null; then
        cargo install "$bin_name"
    else
        echo "$bin_name already installed, skipping."
    fi
}


# Main
#----------------------------------------------------------------------------
main() {
    initialize_environment "$@"

    add_target_support
    install_binary "${MOPRO_ROOT}/ark-zkey" "arkzkey-util" "."
    download_files "multiplier2" "multiplier2"
    download_files "keccak256" "keccak256_256_test"
    install_binary "${MOPRO_ROOT}/mopro-ffi" "uniffi-bindgen" "."
    install_remote_binary "toml" "toml-cli"
    check_uniffi_bindgen_version

    print_warning "There are more platform-specific dependencies to be installed."
    print_warning "See mopro README.md for details."
    print_action "Done! You may now initialize or build your project."
}

main "$@"