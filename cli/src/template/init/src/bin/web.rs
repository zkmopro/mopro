use std::{path::Path, process::Command};

fn main() {
    // Define the paths
    let mopro_wasm_path = "./mopro-wasm";   // Path to mopro-wasm
    let output_dir = "../MoproWasmBindings"; // Output directory
    
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
