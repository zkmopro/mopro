use std::fs;
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

    // Search the `mopro-wasm` directory
    let mopro_wasm_dir = if cwd
        .parent()
        .map_or(false, |p| p.join("mopro-wasm").exists())
    {
        // When running the script from `test-e2e`
        cwd.parent()
            .expect("Failed to get parent directory")
            .join("mopro-wasm")
    } else {
        // When running the script from the CLI template or else, download 'mopro-wasm' from repo
        fetch_mopro_wasm();
        cwd.join("mopro-wasm")
    };

    let output = Command::new("rustup")
        .current_dir(mopro_wasm_dir)
        .args(&[
            "run",
            "nightly-2024-07-18",
            "wasm-pack",
            "build",
            "--target",
            "web",
            "--out-dir",
            bindings_dest.to_str().unwrap(),
        ])
        .args(&["--", "--all-features"]) // feature flags in mopro-wasm
        .output()
        .expect(&format!("Failed to execute wasm-pack"));

    if output.status.success() {
        println!("mopro-wasm package build completed successfully.");
    } else {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error details:\n{}", stderr_str);
        eprintln!("mopro-wasm package build failed.");
        std::process::exit(1);
    }

    // fetches and extract the `mopro-wasm` crate from mopro.
    fn fetch_mopro_wasm() {
        let cwd = std::env::current_dir().expect("Failed to get current directory");

        // This should be align the `mopro-wasm` dependency on `cli/Cargo.toml`
        let repo_url = "https://github.com/sifnoc/mopro";
        let branch = "mopro-cli-web";
        let repo_dir = cwd.join("mopro");
        let target_dir = "mopro-wasm";

        // Fetch without checkout for avoid download all
        let status = Command::new("git")
            .args(["clone", "--no-checkout", repo_url, "--branch", branch])
            .status()
            .expect("Failed to execute git clone");
        if !status.success() {
            panic!("Failed to clone the repository.");
        }

        let sparse_checkout = Command::new("git")
            .current_dir(repo_dir.clone())
            .args(["sparse-checkout", "init", "--cone"])
            .status()
            .expect("Failed to initialize sparse-checkout");
        if !sparse_checkout.success() {
            panic!("Failed to initialize sparse-checkout.");
        }

        let set_sparse_checkout = Command::new("git")
            .current_dir(repo_dir.clone())
            .args(["sparse-checkout", "set", target_dir])
            .status()
            .expect("Failed to set sparse-checkout path");
        if !set_sparse_checkout.success() {
            panic!("Failed to set sparse-checkout path.");
        }

        // Actual fetch file on 'mopro-wasm' in here
        let checkout = Command::new("git")
            .current_dir(repo_dir.clone())
            .args(["checkout"])
            .status()
            .expect("Failed to checkout the files on 'mopro-wasm'");
        if !checkout.success() {
            panic!("Failed to checkout the files on 'mopro-wasm.");
        }

        // Move from 'mopro/mopro-wasm' to 'mopro-wasm'
        let source_path = repo_dir.join(target_dir);
        let destination_path = cwd.join(target_dir);
        fs::rename(&source_path, &destination_path)
            .expect("Failed to move mopro-wasm directory to the root");
        let _ = fs::remove_dir_all(&repo_dir);
        println!("Fetch 'mopro-wasm' sucessfully")
    }
}
