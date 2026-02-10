use std::path::Path;
use std::process::Command;
use std::{fs, path::PathBuf};

use crate::app_config::cleanup_tmp_local;
use crate::app_config::constants::{Mode, PlatformBuilder, WebArch, WebPlatform, WEB_BINDINGS_DIR};

use super::{mktemp_local, project_name_from_toml};

// Maintained for backwards compatibility
#[inline]
pub fn build() {
    super::build_from_env::<WebPlatform>()
}

impl PlatformBuilder for WebPlatform {
    type Arch = WebArch;
    type Params = ();

    fn build(
        mode: Mode,
        project_dir: &Path,
        _target_archs: Vec<Self::Arch>,
        _params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        let _wasm_style_identifier = project_name_from_toml(project_dir)
            .expect("Failed to get project name from Cargo.toml");
        let build_dir_path = project_dir.join("build");
        let work_dir = mktemp_local(&build_dir_path);
        let bindings_out = work_dir.join(WEB_BINDINGS_DIR);
        fs::create_dir(&bindings_out).expect("Failed to create bindings out directory");
        let bindings_dest = Path::new(&project_dir).join(WEB_BINDINGS_DIR);

        if !project_dir.join("Cargo.toml").exists() {
            panic!("No Cargo.toml found in {:?}", project_dir);
        }
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
            "run",
            "nightly-2024-07-18",
            "wasm-pack",
            "build",
            "--target",
            "web",
            mode_cmd,
            "--out-dir",
            bindings_out.to_str().unwrap(),
            "--out-name",
            "mopro_wasm_lib",
            "--no-default-features",
            "--features",
            "wasm",
        ]);

        cmd.env(
            "RUSTFLAGS",
            "-C target-feature=+atomics,+bulk-memory -C link-arg=--max-memory=4294967296",
        );
        cmd.current_dir(project_dir);

        let status = cmd.status().expect("Failed to run wasm-pack");

        if status.success() {
            println!("mopro-ffi wasm package build completed successfully.");
        } else {
            eprintln!("mopro-ffi wasm package build failed.");
            std::process::exit(1);
        }

        if let Ok(info) = fs::metadata(&bindings_dest) {
            if !info.is_dir() {
                panic!("framework directory exists and is not a directory");
            }
            fs::remove_dir_all(&bindings_dest).expect("Failed to remove framework directory");
        }

        fs::rename(&bindings_out, &bindings_dest).expect("Failed to move framework into place");

        cleanup_tmp_local(&build_dir_path);

        Ok(bindings_dest)
    }
}

fn patch_package_version(
    project_dir: &Path,
    package_name: &str,
    version: &str,
) -> anyhow::Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.args(["update", "-p", package_name, "--precise", version]);
    cmd.current_dir(project_dir);
    let status = cmd.status().expect("Failed to update package");
    if !status.success() {
        eprintln!("Failed to update package");
        return Err(anyhow::anyhow!("Failed to update package"));
    }
    Ok(())
}
