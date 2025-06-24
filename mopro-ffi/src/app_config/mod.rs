use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::Value;
use uuid::Uuid;

use self::constants::{Arch, Mode, PlatformBuilder, BUILD_MODE_ENV};

pub mod android;
pub mod constants;
pub mod ios;

/// Builds bindings for the specified platform using environment variables to determine
/// the build mode, project directory, and target architectures.
fn build_from_env<Builder: PlatformBuilder>() {
    let mode = get_build_mode();
    let project_dir = get_project_dir();
    let target_archs: Vec<Builder::Arch> = get_target_archs();
    let params = Builder::Params::default();

    // Do not build if no target architectures are specified
    if target_archs.is_empty() {
        return;
    }

    Builder::build(mode, &project_dir, target_archs, params)
        .context(format!(
            "Failed to build {} bindings",
            Builder::identifier()
        ))
        .unwrap();
}

/// Builds bindings for the specified platform using a string representation of the target architectures.
pub fn build_from_str_arch<Builder: PlatformBuilder>(
    mode: Mode,
    project_dir: &Path,
    target_archs: Vec<&String>,
    params: Builder::Params,
) -> anyhow::Result<PathBuf> {
    if target_archs.is_empty() {
        return Err(anyhow::anyhow!(
            "No target architectures specified for {} bindings",
            Builder::identifier()
        ));
    }

    let target_archs: Vec<Builder::Arch> = target_archs
        .iter()
        .map(Builder::Arch::parse_from_str)
        .collect();

    Builder::build(mode, project_dir, target_archs, params).context(format!(
        "Failed to build {} bindings",
        Builder::identifier()
    ))
}

pub fn mktemp() -> PathBuf {
    let dir = std::env::temp_dir().join(Path::new(&Uuid::new_v4().to_string()));
    fs::create_dir(&dir).expect("Failed to create tmpdir");
    dir
}

fn tmp_local(build_path: &Path) -> PathBuf {
    let tmp_path = build_path.join("tmp");
    if let Ok(metadata) = fs::metadata(&tmp_path) {
        if !metadata.is_dir() {
            panic!("non-directory tmp");
        }
    } else {
        fs::create_dir_all(&tmp_path).expect("Failed to create local tmpdir");
    }
    tmp_path
}

pub fn mktemp_local(build_path: &Path) -> PathBuf {
    let dir = tmp_local(build_path).join(Uuid::new_v4().to_string());
    fs::create_dir(&dir).expect("Failed to create tmpdir");
    dir
}

pub fn cleanup_tmp_local(build_path: &Path) {
    fs::remove_dir_all(tmp_local(build_path)).expect("Failed to remove tmpdir");
}

pub fn install_ndk() {
    Command::new("cargo")
        .arg("install")
        .arg("cargo-ndk")
        .spawn()
        .expect("Failed to spawn cargo, is it installed?")
        .wait()
        .expect("Failed to install cargo-ndk");
}

pub fn install_arch(arch: String) {
    Command::new("rustup")
        .arg("target")
        .arg("add")
        .arg(arch.clone())
        .spawn()
        .expect("Failed to spawn rustup, is it installed?")
        .wait()
        .unwrap_or_else(|_| panic!("Failed to install target architecture {}", arch));
}

pub fn project_name_from_toml(project_dir: &Path) -> anyhow::Result<String> {
    let cargo_toml_path = project_dir.join("Cargo.toml");
    let cargo_toml_content =
        fs::read_to_string(cargo_toml_path).context("Failed to read Cargo.toml")?;
    let cargo_toml: Value = cargo_toml_content
        .parse::<Value>()
        .context("Failed to parse Cargo.toml")?;

    // If the `name` under [lib] section is set, using the `name` as library name.
    // Otherwise, using the package name.
    let project_name = cargo_toml
        .get("lib")
        .and_then(|lib| lib.get("name"))
        .and_then(|lib| lib.as_str())
        .map(|s| s.to_string())
        .or_else(|| {
            // '-' in the package name is replaced with '_' if we don't specify the lib name
            // So we need to replace '-' with '_' as below
            cargo_toml
                .get("package")
                .and_then(|pkg| pkg.get("name"))
                .and_then(|pkg| pkg.as_str().map(|s| s.replace("-", "_")))
        });

    project_name.ok_or(anyhow::anyhow!("Failed to find project name in Cargo.toml"))
}

fn get_project_dir() -> PathBuf {
    std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn get_build_mode() -> Mode {
    Mode::parse_from_str(
        std::env::var(BUILD_MODE_ENV)
            .unwrap_or_else(|_| Mode::Debug.as_str().to_string())
            .as_str(),
    )
}

fn get_target_archs<A: Arch>() -> Vec<A> {
    if let Ok(archs_str) = std::env::var(A::env_var_name()) {
        if archs_str.is_empty() {
            return vec![];
        }

        archs_str.split(',').map(Arch::parse_from_str).collect()
    } else {
        // Default case: select all supported architectures if none are provided
        A::all_strings()
            .iter()
            .map(|s| A::parse_from_str(s))
            .collect()
    }
}
