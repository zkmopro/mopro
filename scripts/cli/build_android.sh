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
    
    DEVICE_TYPE=$(read_toml "$CONFIG_FILE" "build.android_device_type")
    BUILD_MODE=$(read_toml "$CONFIG_FILE" "build.build_mode")
    USE_DYLIB=$(read_toml "$CONFIG_FILE" "dylib.use_dylib")
    DYLIB_NAME=$(read_toml "$CONFIG_FILE" "dylib.name")
}

# Function to read value from TOML file and remove quotes
read_toml() {
    toml get "$1" "$2" | tr -d '"'
}

# Determine the architecture based on device type
determine_architecture() {
    case $DEVICE_TYPE in
        "x86_64")
            ARCHITECTURE="x86_64-linux-android"
            FOLDER="x86_64"
        ;;
        "x86")
            ARCHITECTURE="i686-linux-android"
            FOLDER="x86"
        ;;
        "arm")
            ARCHITECTURE="armv7-linux-androideabi"
            FOLDER="armeabi-v7a"
        ;;
        "arm64")
            ARCHITECTURE="aarch64-linux-android"
            FOLDER="arm64-v8a"
        ;;
        *)
            echo -e "\n${RED}Error: Invalid device type specified in config: $DEVICE_TYPE${DEFAULT}"
            exit 1
        ;;
    esac
}

# Determine the library directory based on build mode
determine_build_directory() {
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
}

# Build process
#----------------------------------------------------------------------------

# Build process for mopro_core
build_mopro_core() {
    cd "${MOPRO_ROOT}/mopro-core" || exit
    print_action "Building mopro-core ($BUILD_MODE)..."
    if [[ "$BUILD_MODE" == "release" ]]; then
        env BUILD_CONFIG_PATH="$BUILD_CONFIG_PATH" cargo build --target "$ARCHITECTURE" --release
    else
        env BUILD_CONFIG_PATH="$BUILD_CONFIG_PATH" cargo build --target "$ARCHITECTURE"
    fi
}

build_mopro_ffi_static() {
    cd "${MOPRO_ROOT}/mopro-ffi" || exit
    print_action "Building mopro-ffi as a static library ($BUILD_MODE)..."
    if [[ "$BUILD_MODE" == "release" ]]; then
        cargo ndk --target "$ARCHITECTURE" build --lib --release 
    else
        cargo ndk --target "$ARCHITECTURE" build --lib
    fi
    
    # Ensure the target directory exists
    mkdir -p "${TARGET_DIR}/jniLibs/${FOLDER}"
    
    # Copy the static library to the target directory
    print_action "Copying static library to target directory..."
    cp "${MOPRO_ROOT}/target/${ARCHITECTURE}/${LIB_DIR}/libmopro_ffi.so" \
    "${TARGET_DIR}/jniLibs/${FOLDER}/libuniffi_mopro.so"
    if [ $? -ne 0 ]; then
        echo -e "${RED}Failed to copy static library.${DEFAULT}"
        exit 1
    fi
}

generate_kotlin_bindings() {
    print_action "Generating Kotlin bindings..."
    uniffi-bindgen generate "${MOPRO_ROOT}/mopro-ffi/src/mopro.udl" --language kotlin --out-dir "${TARGET_DIR}/KotlinBindings"
    if [ $? -ne 0 ]; then
        echo -e "${RED}Failed to generate Kotlin bindings.${DEFAULT}"
        exit 1
    fi

    # Copy the mopro.swift file to the Bindings directory
    cp -r "${TARGET_DIR}/jniLibs/" "${ANDROID_APP_DIR}/app/src/main/jniLibs/"
    cp -r "${TARGET_DIR}/KotlinBindings/uniffi/" "${ANDROID_APP_DIR}/app/src/main/java/uniffi/"
    if [ $? -ne 0 ]; then
        echo -e "${RED}Failed to copy mopro.swift to Bindings directory.${DEFAULT}"
        exit 1
    fi
}

# Main
#----------------------------------------------------------------------------
main() {
    PROJECT_DIR=$(pwd)
    TARGET_DIR=${PROJECT_DIR}/target
    ANDROID_APP_DIR=${PROJECT_DIR}/android
    
    initialize_environment "$@"
    read_configuration "$1"
    determine_architecture
    determine_build_directory
    
    if [[ "$USE_DYLIB" == true ]]; then
        build_mopro_ffi_dylib
    else
        build_mopro_ffi_static
    fi
    
    generate_kotlin_bindings
    
    print_action "Done! Please re-build your project in Android Studio."
    print_action "Run \`open android -a Android\ Studio\` to do so."
}

main "$@"
