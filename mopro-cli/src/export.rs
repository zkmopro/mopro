use std::path::PathBuf;
use std::{env, fs};

pub fn export_bindings(destination: &PathBuf) {
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
