use std::fs;
use std::path::Path;
use std::process::Command;

use super::{cleanup_tmp_local, install_arch, install_ndk, mktemp_local};

pub const MOPRO_KOTLIN: &str = include_str!("../../KotlinBindings/uniffi/mopro/mopro.kt");

pub fn build() {
    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| cwd.to_str().unwrap().to_string());
    let build_dir = Path::new(&manifest_dir).join("build");
    let work_dir = mktemp_local(&build_dir);
    let bindings_out = work_dir.join("MoproAndroidBindings");
    let bindings_dest = Path::new(&manifest_dir).join("MoproAndroidBindings");

    let target_archs = vec![
        "x86_64-linux-android",
        "i686-linux-android",
        "armv7-linux-androideabi",
        "aarch64-linux-android",
    ];

    let mode = std::env::var("CONFIGURATION")
        .unwrap_or_else(|_| "debug".to_string())
        .to_lowercase();
    let mode = match mode.as_str() {
        "debug" | "release" => mode,
        _ => panic!("Unknown configuration: {}", mode),
    };

    install_ndk();
    for arch in target_archs {
        build_for_arch(&arch, &build_dir, &bindings_out, &mode);
    }

    write_kotlin_bindings(&bindings_out);
    move_bindings(&bindings_out, &bindings_dest);
    cleanup_tmp_local(&build_dir);
}

fn build_for_arch(arch: &str, build_dir: &Path, bindings_out: &Path, mode: &str) {
    install_arch(arch.to_string());

    let mut build_cmd = Command::new("cargo");
    build_cmd
        .arg("ndk")
        .arg("-t")
        .arg(arch)
        .arg("build")
        .arg("--lib");
    if mode == "release" {
        build_cmd.arg("--release");
    }
    build_cmd
        .env("CARGO_BUILD_TARGET_DIR", build_dir)
        .env("CARGO_BUILD_TARGET", arch)
        .spawn()
        .expect("Failed to spawn cargo build")
        .wait()
        .expect("cargo build errored");

    let folder = match arch {
        "x86_64-linux-android" => "x86_64",
        "i686-linux-android" => "x86",
        "armv7-linux-androideabi" => "armeabi-v7a",
        "aarch64-linux-android" => "arm64-v8a",
        _ => panic!("Unknown target architecture: {}", arch),
    };

    let out_lib_path = build_dir.join(format!(
        "{}/{}/{}/libmopro_bindings.so",
        build_dir.display(),
        arch,
        mode
    ));
    let out_lib_dest = bindings_out.join(format!("jniLibs/{}/libuniffi_mopro.so", folder));

    fs::create_dir_all(out_lib_dest.parent().unwrap()).expect("Failed to create jniLibs directory");
    fs::copy(&out_lib_path, &out_lib_dest).expect("Failed to copy file");
}

fn write_kotlin_bindings(bindings_out: &Path) {
    let mopro_kt_path = bindings_out.join("uniffi/mopro/mopro.kt");
    fs::create_dir_all(mopro_kt_path.parent().unwrap())
        .expect("Failed to create uniffi/mopro directory");
    fs::write(&mopro_kt_path, MOPRO_KOTLIN).expect("Failed to write mopro.kt");
}

fn move_bindings(bindings_out: &Path, bindings_dest: &Path) {
    if let Ok(info) = fs::metadata(&bindings_dest) {
        if !info.is_dir() {
            panic!("bindings directory exists and is not a directory");
        }
        fs::remove_dir_all(&bindings_dest).expect("Failed to remove bindings directory");
    }
    fs::rename(&bindings_out, &bindings_dest).expect("Failed to move bindings into place");
}
