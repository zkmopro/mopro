use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

pub mod android;
pub mod ios;

#[derive(Debug, thiserror::Error)]
pub enum MoproBuildError {
    #[error("Failed to build the release library: {0}")]
    LibraryBuildError(String),
    #[error("Failed to generate bindings: {0}")]
    GenerateBindingsError(String),
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
    let dir = tmp_local(build_path).join(&Uuid::new_v4().to_string());
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
        .expect(format!("Failed to install target architecture {}", arch).as_str());
}

fn setup_directories() -> (String, String, String, PathBuf, PathBuf) {
    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(cwd.to_str().unwrap().to_string());

    // Library name is the name of the crate with all `-` replaced with `_`
    // TODO - find a way to get the real name of the library as it might not be the same as the crate name
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let library_name = crate_name.replace("-", "_");

    let build_dir = format!("{}/build", manifest_dir);
    let build_dir_path = Path::new(&build_dir).to_path_buf();
    let work_dir = mktemp_local(&build_dir_path);
    (
        manifest_dir,
        library_name,
        build_dir,
        build_dir_path,
        work_dir,
    )
}
