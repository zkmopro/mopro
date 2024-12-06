use std::{path::Path, process::Command};

fn main() {
    // The `mopro-wasm` package requires compilation using the nightly version of Rust.
    // The build method that uses the `wasm-pack` crate cannot be called directly in `test-e2e` as it relies on stable Rust.
    // Rust does not support mixing different toolchains or targets in a single build, hence this separation.
    // Note: There is a known another issue with `wasm-pack` that may affect the build process. For more information, see: https://github.com/rustwasm/wasm-pack/issues/1400.

    // Define the paths
    let mopro_wasm_path = "../mopro-wasm"; // Path to mopro-wasm
    let output_dir = "../test-e2e/web/mopro-pkg"; // Output directory

    // Debugging: Verify the mopro-wasm path
    println!(
        "Expected mopro-wasm path: {}",
        Path::new(mopro_wasm_path)
            .canonicalize()
            .unwrap_or_else(|_| Path::new(mopro_wasm_path).to_path_buf())
            .display()
    );

    // Verify that the mopro-wasm directory exists
    if !Path::new(mopro_wasm_path).exists() {
        eprintln!("Error: The directory '{}' does not exist.", mopro_wasm_path);
        std::process::exit(1);
    }

    // Check if `wasm-pack` command is available
    let check_status = Command::new("wasm-pack").arg("--version").status();

    if check_status.is_err() || !check_status.unwrap().success() {
        eprintln!("Error: `wasm-pack` command not found. Please install `wasm-pack` to proceed.");
        std::process::exit(1);
    }

    // Recommended: Use the `yarn build` script in `test-e2e/web` for faster builds, as it avoids compiling Rust each time.
    let output = Command::new("rustup")
        .current_dir(mopro_wasm_path)
        .args(&[
            "run",
            "nightly-2024-07-18",
            "wasm-pack",
            "build",
            "--target",
            "web",
            "--out-dir",
            output_dir,
        ])
        .args(&["--", "--all-features"]) // feature flags in mopro-wasm
        .output()
        .expect(&format!(
            "Failed to execute wasm-pack on {}",
            mopro_wasm_path
        ));

    if output.status.success() {
        println!("mopro-wasm package build completed successfully.");
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error details:\n{}", stderr_str);
        eprintln!("mopro-wasm package build failed.");
        std::process::exit(1);
    }
}
