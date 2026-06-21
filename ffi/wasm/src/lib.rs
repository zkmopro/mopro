use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use mopro_build_common::{
    build_from_env, cleanup_tmp_local, mktemp_local,
    PlatformBuilder, WEB_BINDINGS_DIR,
};
pub use mopro_build_common::{build_from_str_arch as wasm_build_from_str_arch, Mode, WebArch};

pub struct WebPlatform;

pub fn build() {
    build_from_env::<WebPlatform>()
}

impl PlatformBuilder for WebPlatform {
    type Arch = WebArch;
    type Params = ();

    fn identifier() -> &'static str { "wasm" }

    fn build(
        mode: Mode,
        project_dir: &Path,
        _target_archs: Vec<Self::Arch>,
        _params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        if !project_dir.join("Cargo.toml").exists() {
            panic!("No Cargo.toml found in {:?}", project_dir);
        }
        let build_dir_path = project_dir.join("build");
        let work_dir = mktemp_local(&build_dir_path);
        let bindings_out = work_dir.join(WEB_BINDINGS_DIR);
        fs::create_dir(&bindings_out).expect("Failed to create bindings out directory");
        let bindings_dest = project_dir.join(WEB_BINDINGS_DIR);

        patch_package_version(project_dir, "indexmap", "2.11.4")?;
        patch_package_version(project_dir, "backtrace", "0.3.73")?;
        patch_package_version(project_dir, "blake2b_simd", "1.0.3")?;
        patch_package_version(project_dir, "wasip2", "1.0.1+wasi-0.2.4")?;

        let mode_cmd = match mode {
            Mode::Release => "--release",
            Mode::Debug => "--dev",
        };

        let mut cmd = Command::new("rustup");
        cmd.args([
            "run", "nightly-2025-02-20",
            "wasm-pack", "build",
            "--target", "web",
            mode_cmd,
            "--out-dir", bindings_out.to_str().unwrap(),
            "--out-name", "mopro_wasm_lib",
            "--no-default-features",
            "--features", "wasm",
        ]);
        cmd.env(
            "RUSTFLAGS",
            "-C target-feature=+atomics,+bulk-memory -C link-arg=--max-memory=4294967296",
        );
        cmd.current_dir(project_dir);

        let status = cmd.status().expect("Failed to run wasm-pack");
        if status.success() {
            println!("mopro wasm package build completed successfully.");
        } else {
            eprintln!("mopro wasm package build failed.");
            std::process::exit(1);
        }

        if let Ok(info) = fs::metadata(&bindings_dest) {
            if !info.is_dir() { panic!("bindings directory exists and is not a directory"); }
            fs::remove_dir_all(&bindings_dest).expect("Failed to remove bindings directory");
        }
        fs::rename(&bindings_out, &bindings_dest).expect("Failed to move bindings into place");
        cleanup_tmp_local(&build_dir_path);

        Ok(bindings_dest)
    }
}

fn patch_package_version(project_dir: &Path, package_name: &str, version: &str) -> anyhow::Result<()> {
    let spec = package_spec_for_update(project_dir, package_name, version)?;
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(&cargo)
        .args(["update", "-p", &spec, "--precise", version])
        .current_dir(project_dir)
        .status()
        .with_context(|| format!("Failed to run cargo update for {package_name}"))?;
    if !status.success() {
        anyhow::bail!("Failed to update package {package_name} to {version}");
    }
    Ok(())
}

// Returns "name" when there is only one version in Cargo.lock, or "name@x.y.z"
// when multiple versions exist (disambiguates by matching the major version of `target`).
fn package_spec_for_update(project_dir: &Path, name: &str, target: &str) -> anyhow::Result<String> {
    let lock_path = project_dir.join("Cargo.lock");
    let content = std::fs::read_to_string(&lock_path)
        .with_context(|| format!("Failed to read {:?}", lock_path))?;

    // Collect all versions of `name` from the Cargo.lock [[package]] blocks.
    let mut versions: Vec<String> = Vec::new();
    let mut cur_name = String::new();
    for line in content.lines() {
        if line == "[[package]]" {
            cur_name.clear();
        } else if let Some(n) = line.strip_prefix("name = \"") {
            cur_name = n.trim_end_matches('"').to_string();
        } else if let Some(v) = line.strip_prefix("version = \"") {
            if cur_name == name {
                versions.push(v.trim_end_matches('"').to_string());
            }
        }
    }

    if versions.len() <= 1 {
        return Ok(name.to_string());
    }

    // Multiple versions: pick the one whose major version matches `target`.
    let target_major = target.split('.').next().unwrap_or("0");
    let matched = versions.iter()
        .find(|v| v.split('.').next().unwrap_or("") == target_major)
        .or_else(|| versions.first())
        .cloned()
        .unwrap_or_else(|| name.to_string());

    Ok(format!("{name}@{matched}"))
}
