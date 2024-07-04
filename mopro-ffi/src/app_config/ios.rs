use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

use super::{cleanup_tmp_local, install_arch, mktemp_local};

// Load environment variables that are specified by by xcode
pub fn build() {
    let cwd = std::env::current_dir().unwrap();
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(cwd.to_str().unwrap().to_string());
    let build_dir = format!("{}/build", manifest_dir);
    let build_dir_path = Path::new(&build_dir);
    let bindings_dir = format!("{}/target/out", manifest_dir);
    let bindings_dir_path = Path::new(&bindings_dir);
    let work_dir = mktemp_local(&build_dir_path);
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
        let lib_out = mktemp_local(&build_dir_path).join("libmopro_bindings.a");
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

    prepare_bindings(&bindings_dir_path, &bindings_out)
        .expect("Failed to prepare bindings for iOS");
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
            .arg(bindings_dir_path.to_str().unwrap());
    }
    xcbuild_cmd
        .arg("-output")
        .arg(framework_out.to_str().unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    if let Ok(info) = fs::metadata(&bindings_dest) {
        if !info.is_dir() {
            panic!("framework directory exists and is not a directory");
        }
        fs::remove_dir_all(&bindings_dest).expect("Failed to remove framework directory");
    }
    fs::rename(&bindings_out, &bindings_dest).expect("Failed to move framework into place");
    // Copy the mopro.swift file to the output directory
    cleanup_tmp_local(&build_dir_path)
}

pub fn prepare_bindings(binding_dir: &Path, swift_files_dir: &Path) -> io::Result<()> {
    // Iterate over each `.swift` file in the `binding_dir` directory
    for entry in fs::read_dir(binding_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and has a `.swift` extension
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("swift") {
            // Get the file name without the extension
            if let Some(file_stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                // Copy the `.swift` file to the `swift_files_dir` directory
                let dest_path = swift_files_dir.join(path.file_name().unwrap());
                fs::rename(&path, &dest_path)?;

                // Create a directory with the name `<name>FFI`
                let ffi_dir = binding_dir.join(format!("{}FFI", file_stem));
                fs::create_dir_all(&ffi_dir)?;

                // Copy the `<name>FFI.h` file into the `<name>FFI` directory
                let header_src = binding_dir.join(format!("{}FFI.h", file_stem));
                if header_src.exists() {
                    let header_dest = ffi_dir.join(format!("{}FFI.h", file_stem));
                    fs::rename(header_src, header_dest)?;
                }

                // Rename the `<name>FFI.modulemap` file to `module.modulemap`
                let modulemap_src = binding_dir.join(format!("{}FFI.modulemap", file_stem));
                if modulemap_src.exists() {
                    let modulemap_dest = ffi_dir.join("module.modulemap");
                    fs::rename(modulemap_src, modulemap_dest)?;
                }
            }
        }
    }

    Ok(())
}
