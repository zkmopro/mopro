#!/bin/bash

# Script for initializing and updating an Android (simplified) project with Rust bindings.

# Prelude
#----------------------------------------------------------------------------
initialize_environment() {
    if [ -z "$MOPRO_ROOT" ]; then
        echo "MOPRO_ROOT is not set. Please set it to your local mopro repository."
        exit 1
    fi
    
    if ! command -v toml &> /dev/null; then
        echo -e "${RED}toml (toml-cli) is not installed. Please install it to continue.${DEFAULT}"
        exit 1
    fi
    
    if [ "$#" -ne 1 ]; then
        echo -e "\n${RED}Usage: $0 path/to/config.toml${DEFAULT}"
        exit 1
    fi
    
    source "${MOPRO_ROOT}/scripts/_prelude.sh"
}

read_configuration() {
    CONFIG_FILE="$1"
    export BUILD_CONFIG_PATH="$PROJECT_DIR/$CONFIG_FILE"
    print_action "Using build configuration file: $BUILD_CONFIG_PATH"
    
    CIRCUIT_DIR=$(read_toml "$CONFIG_FILE" "circuit.dir")
    CIRCUIT_NAME=$(read_toml "$CONFIG_FILE" "circuit.name")
}

# Function to read value from TOML file and remove quotes
read_toml() {
    toml get "$1" "$2" | tr -d '"'
}

copy_circuit_files() {
    print_action "Copying circuit files..."
    cp "${PROJECT_DIR}/${CIRCUIT_DIR}/target/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm" "${WEB_APP_DIR}/public/${CIRCUIT_NAME}.wasm"

    cp "${PROJECT_DIR}/${CIRCUIT_DIR}/target/${CIRCUIT_NAME}_final.zkey" "${WEB_APP_DIR}/public/${CIRCUIT_NAME}_final.zkey"
    if [ $? -ne 0 ]; then
        echo -e "${RED}Failed to copy circuit files.${DEFAULT}"
        exit 1
    fi
}

# Main
#----------------------------------------------------------------------------
main() {
    PROJECT_DIR=$(pwd)
    TARGET_DIR=${PROJECT_DIR}/target
    WEB_APP_DIR=${PROJECT_DIR}/web
    
    initialize_environment "$@"
    read_configuration "$1"
    
    copy_circuit_files
    
    print_action "Done! Please open the web app with \`cd web && npm install && npm run dev\`"
}

main "$@"
