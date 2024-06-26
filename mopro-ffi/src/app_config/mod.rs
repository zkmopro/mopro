use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

pub fn mktemp() -> PathBuf {
    let dir = std::env::temp_dir().join(Path::new(&Uuid::new_v4().to_string()));
    fs::create_dir(&dir).expect("Failed to create tmpdir");
    dir
}

// Load environment variables that are specified by by xcode
pub fn build() {
    install_archs();
    let work_dir = mktemp();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_dir = format!("{}/build", manifest_dir);
    let swift_bindings_dir = work_dir.join(Path::new("SwiftBindings"));
    let framework_out = work_dir.join("MoproBindings.xcframework");
    let framework_dest = Path::new(&manifest_dir).join("MoproBindings.xcframework");

    // https://developer.apple.com/documentation/xcode/build-settings-reference#Architectures
    if let Ok(platform_name) = std::env::var("PLATFORM_NAME") {
        if platform_name != "iphoneos" {
            panic!("non-iphoneos target");
        }
    }
    let arch;
    if let Ok(target_device) = std::env::var("PLATFORM_NAME") {
        arch = match target_device.as_str() {
            "iphonesimulator" => "aarch64-apple-ios-sim",
            "iphoneos" => "aarch64-apple-ios",
            _ => panic!("unknown target device"),
        };
    } else {
        arch = "aarch64-apple-ios";
    }

    let mode;
    if let Ok(configuration) = std::env::var("CONFIGURATION") {
        mode = match configuration.as_str() {
            "Debug" => "debug",
            "Release" => "release",
            _ => panic!("unknown configuration"),
        };
    } else {
        mode = "debug";
    }

    let library_out = Path::new(&build_dir).join(Path::new(&format!(
        "{}/{}/{}/libmopro_bindings.a",
        build_dir, arch, mode
    )));
    let library_out_final = Path::new(&build_dir).join(Path::new(&format!(
        "{}/{}/{}/libmopro_ffi.a",
        build_dir, arch, mode
    )));

    Command::new("cargo")
        .arg("build")
        .arg("--lib")
        .env("CARGO_BUILD_TARGET_DIR", &build_dir)
        .env("CARGO_BUILD_TARGET", arch)
        .env("CARGO_BUILD_MODE", mode)
        .spawn()
        .expect("Failed to spawn cargo build")
        .wait()
        .expect("cargo build errored");

    write_bindings_swift(&swift_bindings_dir);
    fs::rename(&library_out, &library_out_final).unwrap();

    Command::new("xcodebuild")
        .arg("-create-xcframework")
        .arg("-library")
        .arg(library_out_final.to_str().unwrap())
        .arg("-headers")
        .arg(swift_bindings_dir.to_str().unwrap())
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

pub const MOPRO_SWIFT: &str = include_str!("../../SwiftBindings/mopro.swift");
pub const MOPRO_FFI_H: &str = include_str!("../../SwiftBindings/moproFFI.h");
pub const MOPRO_MODULEMAP: &str = include_str!("../../SwiftBindings/moproFFI.modulemap");

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

pub const UDL: &str = include_str!("../mopro.udl");

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
