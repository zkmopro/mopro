use std::io;
use std::path::Path;
use std::process::Command;

pub fn build_cdylib(build_dir_str: &str) -> io::Result<()> {
    let build_dir = Path::new(build_dir_str);
    // Set up the command to run `cargo build --release`
    let output = Command::new("cargo")
        .env("CARGO_BUILD_TARGET_DIR", build_dir)
        .arg("build")
        .spawn()
        .expect("Failed to spawn cargo build")
        .wait()
        .expect("cargo build errored");

    if !output.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to build the release library".to_string(),
        ));
    }

    Ok(())
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
