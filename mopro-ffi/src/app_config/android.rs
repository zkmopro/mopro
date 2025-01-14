use std::fs;
use std::path::Path;
use std::process::Command;

use mopro_cli::AndroidArch;
use uniffi::generate_bindings;
use uniffi::KotlinBindingGenerator;

use super::cleanup_tmp_local;
use super::install_arch;
use super::install_ndk;
use super::mktemp_local;

pub fn build() {
    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| cwd.to_str().unwrap().to_string());
    let build_dir = Path::new(&manifest_dir).join("build");
    let work_dir = mktemp_local(&build_dir);
    let bindings_out = work_dir.join("MoproAndroidBindings");
    let bindings_dest = Path::new(&manifest_dir).join("MoproAndroidBindings");

    let mode = std::env::var("CONFIGURATION")
        .unwrap_or_else(|_| "debug".to_string())
        .to_lowercase();
    let mode = match mode.as_str() {
        "debug" | "release" => mode,
        _ => panic!("Unknown configuration: {}", mode),
    };

    let target_archs: Vec<AndroidArch> = if let Ok(archs_str) = std::env::var("ANDROID_ARCHS") {
        archs_str
            .split(',')
            .map(|arch| AndroidArch::from_str(arch))
            .collect()
    } else {
        // Default case: select all supported architectures if none are provided
        AndroidArch::all_strings()
            .iter()
            .map(|s| AndroidArch::from_str(s))
            .collect()
    };

    install_ndk();
    for arch in target_archs {
        build_for_arch(arch, &build_dir, &bindings_out, &mode);
    }

    generate_bindings(
        (manifest_dir + "/src/mopro.udl").as_str().into(),
        None,
        KotlinBindingGenerator,
        Some(bindings_out.to_str().unwrap().into()),
        None,
        None,
        false,
    )
    .expect("Failed to generate bindings");

    move_bindings(&bindings_out, &bindings_dest);
    cleanup_tmp_local(&build_dir);
}

fn build_for_arch(arch: AndroidArch, build_dir: &Path, bindings_out: &Path, mode: &str) {
    install_arch(arch.as_str().to_string());

    let mut build_cmd = Command::new("cargo");
    build_cmd
        .arg("ndk")
        .arg("-t")
        .arg(arch.as_str())
        .arg("build")
        .arg("--lib");
    if mode == "release" {
        build_cmd.arg("--release");
    }
    build_cmd
        .env("CARGO_BUILD_TARGET_DIR", build_dir)
        .env("CARGO_BUILD_TARGET", arch.as_str())
        .spawn()
        .expect("Failed to spawn cargo build")
        .wait()
        .expect("cargo build errored");

    // FIXME move these string to a single file
    let folder = match arch {
        AndroidArch::X8664Linux => "x86_64",
        AndroidArch::I686Linux => "x86",
        AndroidArch::Armv7LinuxAbi => "armeabi-v7a",
        AndroidArch::Aarch64Linux => "arm64-v8a",
    };

    let out_lib_path = build_dir.join(format!(
        "{}/{}/{}/libmopro_bindings.so",
        build_dir.display(),
        arch.as_str(),
        mode
    ));
    let out_lib_dest = bindings_out.join(format!("jniLibs/{}/libuniffi_mopro.so", folder));

    fs::create_dir_all(out_lib_dest.parent().unwrap()).expect("Failed to create jniLibs directory");
    fs::copy(&out_lib_path, &out_lib_dest).expect("Failed to copy file");
}

fn move_bindings(bindings_out: &Path, bindings_dest: &Path) {
    if let Ok(info) = fs::metadata(bindings_dest) {
        if !info.is_dir() {
            panic!("bindings directory exists and is not a directory");
        }
        fs::remove_dir_all(bindings_dest).expect("Failed to remove bindings directory");
    }
    fs::rename(bindings_out, bindings_dest).expect("Failed to move bindings into place");
}
