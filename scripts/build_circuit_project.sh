#!/bin/bash

# Script to build and prepare circuits for mopro.
#
# Does the following:
# - Install npm dependencies and compiles the circuit
# - Run trusted setup
# - Generate arkzkey

# Initialize environment and check prerequisites
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

# Read configuration from TOML file
read_configuration() {
    CONFIG_FILE="$1"
    export BUILD_CONFIG_PATH="$PWD/$CONFIG_FILE"
    print_action "Using build configuration file: $BUILD_CONFIG_PATH"

    DEVICE_TYPE=$(read_toml "$CONFIG_FILE" "build.device_type")
    BUILD_MODE=$(read_toml "$CONFIG_FILE" "build.build_mode")
    USE_DYLIB=$(read_toml "$CONFIG_FILE" "dylib.use_dylib")
    DYLIB_NAME=$(read_toml "$CONFIG_FILE" "dylib.name")
    CIRCUIT_DIR=$(read_toml "$CONFIG_FILE" "circuit.dir")
    CIRCUIT_NAME=$(read_toml "$CONFIG_FILE" "circuit.name")
}

# Function to read value from TOML file and remove quotes
read_toml() {
    toml get "$1" "$2" | tr -d '"'
}

# Compile the circuit
compile_circuit() {
    print_action "Compiling circuit $CIRCUIT_NAME..."

    local circuit_file_path="${CIRCUIT_DIR}/${CIRCUIT_NAME}.circom"
    local output_dir="${CIRCUIT_DIR}/target"

    mkdir -p "$output_dir"

    # Compile the circuit using circom
    circom "$circuit_file_path" --r1cs --wasm --sym --output "$output_dir"
}

# Main function to orchestrate the script
main() {
    initialize_environment "$@"
    read_configuration "$1"
    compile_circuit
    print_action "Circuit compilation completed successfully."
}

main "$@"