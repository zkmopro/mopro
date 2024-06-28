use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

pub mod ios;

pub fn mktemp() -> PathBuf {
    let dir = std::env::temp_dir().join(Path::new(&Uuid::new_v4().to_string()));
    fs::create_dir(&dir).expect("Failed to create tmpdir");
    dir
}

pub const UDL: &str = include_str!("../mopro.udl");

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

pub fn install_archs() {
    let archs = vec![
        "x86_64-apple-ios",
        "aarch64-apple-ios",
        "aarch64-apple-ios-sim",
        "aarch64-linux-android",
        "armv7-linux-androideabi",
        "i686-linux-android",
        "x86_64-linux-android",
    ];
    for arch in archs {
        install_arch(arch.to_string());
    }
}
