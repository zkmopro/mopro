use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::Value;
use uuid::Uuid;

pub mod android;
pub mod constants;
pub mod ios;

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

pub fn project_name_from_toml() -> String {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_toml_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");
    let cargo_toml_content = std::fs::read_to_string(cargo_toml_path).unwrap();
    let cargo_toml: Value = cargo_toml_content.parse::<Value>().unwrap();

    // If the `name` under [lib] section is set, using the `name` as library name.
    // Otherwise, using the package name.
    let project_name = cargo_toml
        .get("lib")
        .and_then(|lib| lib.get("name"))
        .and_then(|lib| lib.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            // '-' in the package name is replaced with '_' if we don't specify the lib name
            // So we need to replace '-' with '_' as below
            cargo_toml
                .get("package")
                .and_then(|pkg| pkg.get("name"))
                .and_then(|pkg| pkg.as_str().map(|s| s.replace("-", "_")))
                .expect("Cannot find package name")
        });
    project_name
}
