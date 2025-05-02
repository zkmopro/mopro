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

pub fn toml_lib_name(ext_name: &str) -> Option<String> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_toml_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");
    let cargo_toml_content = std::fs::read_to_string(cargo_toml_path).unwrap();
    let cargo_toml: Value = cargo_toml_content.parse::<Value>().unwrap();

    cargo_toml
        .get("lib")
        .and_then(|lib| lib.get("name"))
        .and_then(|name| name.as_str())
        .map(|name_str| format!("lib{}.{}", name_str, ext_name))
}
