use std::path::Path;
use std::process::Command;
use std::{env, fs};

pub const UDL: &str = include_str!("../mopro.udl");

pub enum ProverType {
    Circom,
    Halo2,
}

pub struct CircuitConfig<'a> {
    pub prover: ProverType,
    pub zkey_path: &'a str,
    pub wasm_path: Option<&'a str>,
}

pub fn build_uniffi() {
    // TODO: don't rely on being in the src dir?
    let udl_path = Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
        .join(Path::new("src/mopro.udl"));
    fs::write(udl_path.clone(), UDL).expect("Failed to write UDL");
    uniffi::generate_scaffolding(udl_path.to_str().unwrap()).unwrap();
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
        // install is idempotent
        Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg(arch)
            .spawn()
            .expect("Failed to spawn rustup, is it installed?")
            .wait()
            .expect(format!("Failed to install target architecture {}", arch).as_str());
    }
}
