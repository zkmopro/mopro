use std::path::PathBuf;
use std::{env, fs};

fn export_ios_bindings(destination: &PathBuf) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let target_dir = current_dir.join("target");
    let ios_output_dir = &destination.join("ios");

    // Create ios output directory if it does not exist
    fs::create_dir_all(ios_output_dir).expect("Failed to create ios output directory");

    // Define the paths to the source files and directories
    let swift_bindings_dir = target_dir.join("SwiftBindings");
    for library in [
        "x86_64-apple-ios",
        "aarch64-apple-ios-sim",
        "aarch64-apple-ios",
    ] {
        for build_mode in ["release", "debug"] {
            let library_dir = target_dir.join(library);
            let library_file = library_dir.join(build_mode).join("libmopro_ffi.a");
            if library_file.exists() {
                fs_extra::dir::copy(
                    library_dir,
                    ios_output_dir,
                    &fs_extra::dir::CopyOptions::new(),
                )
                .expect("Failed to copy SwiftBindings directory");
            }
        }
    }

    // Copy SwiftBindings directory
    fs_extra::dir::copy(
        swift_bindings_dir,
        ios_output_dir,
        &fs_extra::dir::CopyOptions::new(),
    )
    .expect("Failed to copy SwiftBindings directory");

    fs::rename(
        ios_output_dir.join("SwiftBindings"),
        ios_output_dir.join("Bindings"),
    )
    .expect("Failed to rename SwiftBindings directory");

    println!("Exported Swift bindings to {:?}", destination);
}

fn export_android_bindings(destination: &PathBuf) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let target_dir = current_dir.join("target");
    let android_output_dir = &destination.join("android");

    // Create android output directory if it does not exist
    fs::create_dir_all(android_output_dir).expect("Failed to create android output directory");

    // Define the paths to the source files and directories
    let jni_bindings_dir = target_dir.join("jniLibs");
    let kotlin_bindings_file = target_dir
        .join("KotlinBindings/uniffi/mopro")
        .join("mopro.kt");

    // Define the destination paths
    let _destination_jni_bindings_dir = android_output_dir.join("jniLibs");
    let destination_kotlin_bindings_file = android_output_dir
        .join("uniffi")
        .join("mopro")
        .join("mopro.kt");

    // Create KotlinBindings directory if it does not exist
    fs::create_dir_all(android_output_dir.join("uniffi").join("mopro"))
        .expect("Failed to create KotlinBindings directory");

    // Copy jniLibs directory
    fs_extra::dir::copy(
        jni_bindings_dir,
        android_output_dir,
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
