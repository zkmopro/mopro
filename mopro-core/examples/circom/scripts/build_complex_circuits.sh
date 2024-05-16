#!/bin/bash

# Script for building complex circuits, used for benchmarking

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
# DEBUG_COLOR="${DEFAULT}"
# trap 'echo -e ${DEBUG_COLOR}${BASH_COMMAND}${DEFAULT}' DEBUG

# Function to handle exit
handle_exit() {
    # $? is a special variable that holds the exit code of the last command executed
    if [ $? -ne 0 ]; then
        echo -e "\n${RED}Script did not finish successfully!${DEFAULT}"
    fi
}

# Set the trap
trap handle_exit EXIT

print_action() {
    printf "\n${GREEN}$1${DEFAULT}\n"
}

print_warning() {
    printf "\n${YELLOW}$1${DEFAULT}\n"
}

# Assert we're in the project root
if [[ ! -d "mopro-ffi" || ! -d "mopro-core" || ! -d "mopro-ios" ]]; then
    echo -e "${RED}Error: This script must be run from the project root directory that contains mopro-ffi, mopro-core, and mopro-ios folders.${DEFAULT}"
    exit 1
fi

PROJECT_DIR=$(pwd)
CIRCOM_DIR="${PROJECT_DIR}/mopro-core/examples/circom"
ARKZKEY_DIR="${PROJECT_DIR}/ark-zkey"

compile_circuit() {
    local circuit_dir=$1
    local circuit_file=$2
    local target_file="$circuit_dir/target/$(basename $circuit_file .circom).r1cs"

    print_action "[core/circom] Compiling $circuit_file example circuit..."
    if [ ! -f "$target_file" ]; then
        ./scripts/compile.sh $circuit_dir $circuit_file
    else
        echo "File $target_file already exists, skipping compilation."
    fi
}

# Build Circom circuits in mopro-core and run trusted setup
print_action "[core/circom] Compiling complex-circuit..."
cd "${CIRCOM_DIR}"

# Compile complex-circuits
compile_circuit complex-circuit complex-circuit-100k-100k.circom
compile_circuit complex-circuit complex-circuit-200k-200k.circom
compile_circuit complex-circuit complex-circuit-400k-400k.circom
compile_circuit complex-circuit complex-circuit-800k-800k.circom
compile_circuit complex-circuit complex-circuit-1600k-1600k.circom

# Run trusted setup for complex-circuits
print_action "[core/circom] Running trusted setup for complex-circuit..."
./scripts/trusted_setup.sh complex-circuit 17 complex-circuit-100k-100k
./scripts/trusted_setup.sh complex-circuit 18 complex-circuit-200k-200k
./scripts/trusted_setup.sh complex-circuit 19 complex-circuit-400k-400k
./scripts/trusted_setup.sh complex-circuit 20 complex-circuit-800k-800k
./scripts/trusted_setup.sh complex-circuit 21 complex-circuit-1600k-1600k

# Generate arkzkey for complex-circuits
print_action "[core/circom] Generating arkzkey for complex-circuit..."
./scripts/generate_arkzkey.sh complex-circuit complex-circuit-100k-100k
./scripts/generate_arkzkey.sh complex-circuit complex-circuit-200k-200k
./scripts/generate_arkzkey.sh complex-circuit complex-circuit-400k-400k
./scripts/generate_arkzkey.sh complex-circuit complex-circuit-800k-800k
./scripts/generate_arkzkey.sh complex-circuit complex-circuit-1600k-1600k

print_action "Done! All complex-circuit circuits built and prepared."