use std::io::Error;
use std::path::Path;
use std::process::Command;
use std::{fs, io};

use camino::Utf8Path;
use uniffi_bindgen::bindings::KotlinBindingGenerator;
use uniffi_bindgen::library_mode::generate_bindings;

use super::{install_arch, install_ndk, setup_directories};

pub fn build() {
    let (manifest_dir, library_name, build_dir, build_dir_path, work_dir) = setup_directories();

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
        build_for_arch(&arch, &build_dir_path, &bindings_out, &mode);
    }

    // To reuse build assets we take `dylib` from the build directory of one of `archs`
    let out_dylib_path = Path::new(&build_dir).join(Path::new(&format!(
        "{}/{}/{}/lib{}.dylib",
        build_dir, target_archs[0], mode, library_name
    )));
    let bindings_build_path = Path::new(&build_dir).join("out");
    generate_kotlin_bindings(&out_dylib_path, &bindings_build_path)
        .expect("Failed to prepare bindings for iOS");

    move_bindings(&bindings_build_path, &bindings_dest);
    // cleanup_tmp_local(&build_dir_path);
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

fn generate_kotlin_bindings(dylib_path: &Path, binding_dir: &Path) -> Result<(), Error> {
    // Generate the bindings for IOS
    generate_bindings(
        Utf8Path::from_path(&dylib_path).ok_or(Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid dylib path",
        ))?,
        None,
        &KotlinBindingGenerator,
        None,
        Utf8Path::from_path(&binding_dir).ok_or(Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid kotlin files directory",
        ))?,
        true,
    )
    .map_err(|e| Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(())
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
