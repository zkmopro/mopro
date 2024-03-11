use std::path::PathBuf;
use std::{env, fs};

fn export_ios_bindings(destination: &PathBuf) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let target_dir = current_dir.join("target");

    // Define the paths to the source files and directories
    let swift_bindings_dir = target_dir.join("SwiftBindings");
    let libmopro_ffi_file = target_dir.join("libmopro_ffi.a");

    // Define the destination paths
    let _destination_swift_bindings_dir = destination.join("SwiftBindings");
    let destination_libmopro_ffi_file = destination.join("libmopro_ffi.a");

    // Copy SwiftBindings directory
    fs_extra::dir::copy(
        swift_bindings_dir,
        destination,
        &fs_extra::dir::CopyOptions::new(),
    )
    .expect("Failed to copy SwiftBindings directory");

    // Copy libmopro_ffi.a file
    fs::copy(libmopro_ffi_file, destination_libmopro_ffi_file)
        .expect("Failed to copy libmopro_ffi.a file");

    println!("Exported Swift bindings to {:?}", destination);
}

fn export_android_bindings(destination: &PathBuf) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let target_dir = current_dir.join("target");

    // Define the paths to the source files and directories
    let jni_bindings_dir = target_dir.join("jniLibs");
    let kotlin_bindings_file = target_dir
        .join("KotlinBindings/uniffi/mopro")
        .join("mopro.kt");

    // Define the destination paths
    let _destination_jni_bindings_dir = destination.join("jniLibs");
    let destination_kotlin_bindings_file = destination.join("KotlinBindings").join("mopro.kt");

    // Create KotlinBindings directory if it does not exist
    fs::create_dir_all(destination.join("KotlinBindings"))
        .expect("Failed to create KotlinBindings directory");

    // Copy jniLibs directory
    fs_extra::dir::copy(
        jni_bindings_dir,
        destination,
        &fs_extra::dir::CopyOptions::new(),
    )
    .expect("Failed to copy jniLibs directory");

    // Copy Kotlin bindings file
    fs::copy(kotlin_bindings_file, destination_kotlin_bindings_file)
        .expect("Failed to copy Kotlin bindings file");

    println!("Exported Kotlin bindings to {:?}", destination);
}

pub fn export_bindings(platforms: &Vec<String>, destination: &PathBuf) {
    fs::create_dir_all(destination).expect("Failed to create destination directory");
    if platforms.contains(&"ios".to_string()) {
        export_ios_bindings(destination);
    }
    if platforms.contains(&"android".to_string()) {
        export_android_bindings(destination);
    }
}
