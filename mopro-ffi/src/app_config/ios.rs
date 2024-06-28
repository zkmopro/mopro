use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::{install_arch, mktemp};

pub const MOPRO_SWIFT: &str = include_str!("../../SwiftBindings/mopro.swift");
pub const MOPRO_FFI_H: &str = include_str!("../../SwiftBindings/moproFFI.h");
pub const MOPRO_MODULEMAP: &str = include_str!("../../SwiftBindings/moproFFI.modulemap");

// Load environment variables that are specified by by xcode
pub fn build() {
    let work_dir = mktemp();
    let cwd = std::env::current_dir().unwrap();
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(cwd.to_str().unwrap().to_string());
    let build_dir = format!("{}/build", manifest_dir);
    let swift_bindings_dir = work_dir.join(Path::new("SwiftBindings"));
    let framework_out = work_dir.join("MoproBindings.xcframework");
    let framework_dest = Path::new(&manifest_dir).join("MoproBindings.xcframework");

    // https://developer.apple.com/documentation/xcode/build-settings-reference#Architectures
    let mode;
    if let Ok(configuration) = std::env::var("CONFIGURATION") {
        mode = match configuration.as_str() {
            "Debug" => "debug",
            "Release" => "release",
            "debug" => "debug",
            "release" => "release",
            _ => panic!("unknown configuration"),
        };
    } else {
        mode = "debug";
    }

    // Build multiple architectures into a single xcframework
    // this let's us use a single build command for all xcode targets
    let sim_arch = match std::env::consts::ARCH {
        "aarch64" => "aarch64-apple-ios-sim",
        "x86_64" => "x86_64-apple-ios",
        v => panic!("Unknown architecture for host system: {}", v),
    };
    let mut target_archs = vec!["aarch64-apple-ios", sim_arch];
    // accept EXTRA_ARCHS as a comma separated list
    let extra_archs = std::env::var("EXTRA_ARCHS").unwrap_or("".to_string());
    for v in extra_archs.split(",").collect::<Vec<_>>() {
        if v.len() == 0 {
            continue;
        }
        if !target_archs.contains(&v) {
            target_archs.push(v);
        }
    }
    let out_lib_paths: Vec<PathBuf> = target_archs
        .iter()
        .map(|arch| {
            Path::new(&build_dir).join(Path::new(&format!(
                "{}/{}/{}/libmopro_bindings.a",
                build_dir, arch, mode
            )))
        })
        .collect();
    for arch in target_archs {
        install_arch(arch.to_string());
        let mut build_cmd = Command::new("cargo");
        build_cmd.arg("build");
        if mode == "release" {
            build_cmd.arg("--release");
        }
        build_cmd
            .arg("--lib")
            .env("CARGO_BUILD_TARGET_DIR", &build_dir)
            .env("CARGO_BUILD_TARGET", arch)
            .spawn()
            .expect("Failed to spawn cargo build")
            .wait()
            .expect("cargo build errored");
    }

    write_bindings_swift(&swift_bindings_dir);

    let mut xcbuild_cmd = Command::new("xcodebuild");
    xcbuild_cmd.arg("-create-xcframework");
    for lib_path in out_lib_paths {
        xcbuild_cmd
            .arg("-library")
            .arg(lib_path.to_str().unwrap())
            .arg("-headers")
            .arg(swift_bindings_dir.to_str().unwrap());
    }
    xcbuild_cmd
        .arg("-output")
        .arg(framework_out.to_str().unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    if let Ok(info) = fs::metadata(&framework_dest) {
        if !info.is_dir() {
            panic!("framework directory exists and is not a directory");
        }
        fs::remove_dir_all(&framework_dest).expect("Failed to remove framework directory");
    }
    fs::rename(&framework_out, &framework_dest).expect("Failed to move framework into place");
}

pub fn write_bindings_swift(out_dir: &Path) {
    if let Ok(info) = fs::metadata(out_dir) {
        if !info.is_dir() {
            panic!("out_dir exists and is not a directory");
        }
    } else {
        fs::create_dir(out_dir).expect("Failed to create output dir");
    }
    fs::write(out_dir.join("mopro.swift"), MOPRO_SWIFT).expect("Failed to write mopro.swift");
    fs::write(out_dir.join("moproFFI.h"), MOPRO_FFI_H).expect("Failed to write moproFFI.h");
    fs::write(out_dir.join("module.modulemap"), MOPRO_MODULEMAP)
        .expect("Failed to write module.modulemap");
}
