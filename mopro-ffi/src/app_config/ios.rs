use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::{cleanup_tmp_local, install_arch, mktemp_local};

// Load environment variables that are specified by by xcode
pub fn build() {
    let cwd = std::env::current_dir().unwrap();
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(cwd.to_str().unwrap().to_string());
    let build_dir = format!("{}/build", manifest_dir);
    let build_dir_path = Path::new(&build_dir);
    let work_dir = mktemp_local(build_dir_path);
    let swift_bindings_dir = work_dir.join(Path::new("SwiftBindings"));
    let bindings_out = work_dir.join("MoproiOSBindings");
    fs::create_dir(&bindings_out).expect("Failed to create bindings out directory");
    let bindings_dest = Path::new(&manifest_dir).join("MoproiOSBindings");
    let framework_out = bindings_out.join("MoproBindings.xcframework");

    let target_archs = vec![
        vec!["aarch64-apple-ios"],
        vec!["aarch64-apple-ios-sim", "x86_64-apple-ios"],
    ];

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

    // Take a list of architectures, build them, and combine them into
    // a single universal binary/archive
    let build_combined_archs = |archs: &Vec<&str>| -> PathBuf {
        let out_lib_paths: Vec<PathBuf> = archs
            .iter()
            .map(|arch| {
                Path::new(&build_dir).join(Path::new(&format!(
                    "{}/{}/{}/libmopro_bindings.a",
                    build_dir, arch, mode
                )))
            })
            .collect();
        for arch in archs {
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
        // now lipo the libraries together
        let mut lipo_cmd = Command::new("lipo");
        let lib_out = mktemp_local(build_dir_path).join("libmopro_bindings.a");
        lipo_cmd
            .arg("-create")
            .arg("-output")
            .arg(lib_out.to_str().unwrap());
        for p in out_lib_paths {
            lipo_cmd.arg(p.to_str().unwrap());
        }
        lipo_cmd
            .spawn()
            .expect("Failed to spawn lipo")
            .wait()
            .expect("lipo command failed");

        lib_out
    };

    let uniffi_bindgen_path = {
        let mut bin_path = std::env::current_exe().unwrap(); //Users/~/mopro/target/debug/ios
        bin_path.pop(); // Remove the current executable name: //Users/~/mopro/target/debug
        bin_path.pop(); //Users/~/mopro/target/
        bin_path.pop(); //Users/~/mopro
        bin_path.push("mopro-ffi");
        bin_path
    };

    let mut bindgen_cmd = Command::new("cargo");
    bindgen_cmd
        .current_dir(&uniffi_bindgen_path)
        .arg("run")
        .arg("--bin")
        .arg("uniffi-bindgen")
        .arg("generate")
        .arg(manifest_dir + "/src/mopro.udl")
        .arg("--language")
        .arg("swift")
        .arg("--out-dir")
        .arg(swift_bindings_dir.to_str().unwrap())
        .spawn()
        .expect("Failed to spawn uniffi-bindgen")
        .wait()
        .expect("uniffi-bindgen errored");

    fs::rename(
        swift_bindings_dir.join("mopro.swift"),
        bindings_out.join("mopro.swift"),
    )
    .expect("Failed to move mopro.swift into place");

    let out_lib_paths: Vec<PathBuf> = target_archs
        .iter()
        .map(|v| build_combined_archs(v))
        .collect();

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

    // The iOS project expects the module map to be named "module.modulemap",
    // but uniffi-bindgen creates "moproFFI.modulemap" by default,
    // therefore we need to rename it.
    rename_module_map_recursively(&bindings_out);

    if let Ok(info) = fs::metadata(&bindings_dest) {
        if !info.is_dir() {
            panic!("framework directory exists and is not a directory");
        }
        fs::remove_dir_all(&bindings_dest).expect("Failed to remove framework directory");
    }

    fs::rename(&bindings_out, &bindings_dest).expect("Failed to move framework into place");
    // Copy the mopro.swift file to the output directory
    cleanup_tmp_local(build_dir_path)
}

fn rename_module_map_recursively(bindings_out: &PathBuf) {
    for entry in fs::read_dir(bindings_out).expect("Failed to read bindings out directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() == "moproFFI.modulemap" {
            let dest_path = path.with_file_name("module.modulemap");
            fs::rename(&path, &dest_path).expect("Failed to rename module map");
        } else if path.is_dir() {
            rename_module_map_recursively(&path);
        }
    }
}
