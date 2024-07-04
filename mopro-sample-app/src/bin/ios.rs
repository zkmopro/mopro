use std::io;
use std::process::Command;

use camino::Utf8Path;
use uniffi_bindgen::bindings::SwiftBindingGenerator;
use uniffi_bindgen::library_mode::generate_bindings;

fn main() {
    let library_name = "mopro_app"; // TODO - get this from the Cargo.toml

    // Build the crate as a release library for the bindgen
    if let Err(e) = build_release() {
        eprintln!("Failed to execute cargo build: {}", e);
    }

    let cwd = std::env::current_dir().unwrap();
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(cwd.to_str().unwrap().to_string());

    let bindings_dir = format!("{}/target/out", manifest_dir);
    let library_path = format!("{}/target/release/lib{}.dylib", manifest_dir, library_name);

    // Generate the bindings for IOS
    generate_bindings(
        Utf8Path::new(&library_path),
        None,
        &SwiftBindingGenerator,
        None,
        Utf8Path::new(&bindings_dir),
        true,
    )
    .expect("Failed to generate bindings for IOS");

    // Combine the bindings into for the iOS app
    mopro_ffi::app_config::ios::build(library_name, &manifest_dir, &bindings_dir);
}

fn build_release() -> io::Result<()> {
    // Set up the command to run `cargo build --release`
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .spawn()
        .expect("Failed to spawn cargo build")
        .wait()
        .expect("cargo build errored");

    if !output.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to build the release library".to_string(),
        ));
    }

    Ok(())
}
