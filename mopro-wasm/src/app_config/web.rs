use std::path::Path;
use std::process::Command;

pub fn build() {
    // The `mopro-wasm` package requires compilation using the nightly version of Rust.
    // The build method that uses the `wasm-pack` crate cannot be called directly in `test-e2e` as it relies on stable Rust.
    // Rust does not support mixing different toolchains or targets in a single build, hence this separation.
    // Note: There is a known another issue with `wasm-pack` that may affect the build process. For more information, see: https://github.com/rustwasm/wasm-pack/issues/1400.
    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let bindings_dest = Path::new(&cwd).join("MoproWasmBindings");

    // Check if `wasm-pack` command is available
    let check_status = Command::new("wasm-pack").arg("--version").status();
    if check_status.is_err() || !check_status.unwrap().success() {
        eprintln!("Error: `wasm-pack` command not found. Please install `wasm-pack` to proceed.");
        std::process::exit(1);
    }

    let mopro_wasm_lib_dir = cwd.join("mopro-wasm-lib");
    if !mopro_wasm_lib_dir.exists() {
        eprintln!("Error: `mopro-wasm-lib` directory not found. Please ensure it exists.");
        std::process::exit(1);
    }

    let output = Command::new("rustup")
        .current_dir(mopro_wasm_lib_dir)
        .args([
            "run",
            "nightly-2024-07-18",
            "wasm-pack",
            "build",
            "--target",
            "web",
            "--out-dir",
            bindings_dest.to_str().unwrap(),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if output.success() {
        println!("mopro-wasm package build completed successfully.");
    } else {
        eprintln!("mopro-wasm package build failed.");
        std::process::exit(1);
    }
}
