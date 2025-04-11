use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;

use camino::Utf8Path;
use uniffi::generate_bindings_library_mode;
use uniffi::CargoMetadataConfigSupplier;
use uniffi::SwiftBindingGenerator;

use super::cleanup_tmp_local;
use super::constants::{IosArch, Mode, ARCH_ARM_64, ARCH_X86_64, ENV_CONFIG, ENV_IOS_ARCHS};
use super::install_arch;
use super::mktemp_local;

// Load environment variables that are specified by by xcode
pub fn build() {
    const BINDING_NAME: &str = "MoproiOSBindings";

    let cwd = std::env::current_dir().unwrap();
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(cwd.to_str().unwrap().to_string());
    let build_dir = format!("{}/build", manifest_dir);
    let build_dir_path = Path::new(&build_dir);
    let work_dir = mktemp_local(build_dir_path);
    let swift_bindings_dir = work_dir.join(Path::new("SwiftBindings"));
    let bindings_out = work_dir.join(BINDING_NAME);
    fs::create_dir(&bindings_out).expect("Failed to create bindings out directory");
    let bindings_dest = Path::new(&manifest_dir).join(BINDING_NAME);
    let framework_out = bindings_out.join("MoproBindings.xcframework");

    // https://developer.apple.com/documentation/xcode/build-settings-reference#Architectures
    let mode = Mode::parse_from_str(
        std::env::var(ENV_CONFIG)
            .unwrap_or_else(|_| Mode::Debug.as_str().to_string())
            .as_str(),
    );

    let target_archs: Vec<IosArch> = if let Ok(archs_str) = std::env::var(ENV_IOS_ARCHS) {
        archs_str.split(',').map(IosArch::parse_from_str).collect()
    } else {
        // Default case: select all supported architectures if none are provided
        IosArch::all_strings()
            .iter()
            .map(|s| IosArch::parse_from_str(s))
            .collect()
    };

    // Take a list of architectures, build them, and combine them into
    // a single universal binary/archive
    let build_combined_archs = |archs: &[IosArch]| -> PathBuf {
        let out_lib_paths: Vec<PathBuf> = archs
            .iter()
            .map(|arch| {
                Path::new(&build_dir).join(Path::new(&format!(
                    "{}/{}/{}/libmopro_bindings.a",
                    build_dir,
                    arch.as_str(),
                    mode.as_str()
                )))
            })
            .collect();
        for arch in archs {
            install_arch(arch.as_str().to_string());
            let mut build_cmd = Command::new("cargo");
            build_cmd.arg("build");
            if mode == Mode::Release {
                build_cmd.arg("--release");
                build_cmd.env(
                    "RUSTFLAGS",
                    "-C opt-level=z -C panic=abort -C strip=symbols",
                );
            }
            build_cmd
                .arg("--lib")
                .env("CARGO_BUILD_TARGET_DIR", &build_dir)
                .env("CARGO_BUILD_TARGET", arch.as_str())
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

    let out_lib_paths: Vec<PathBuf> = group_target_archs(&target_archs)
        .iter()
        .map(|v| build_combined_archs(v))
        .collect();

    let out_dylib_path = build_dir_path.join(format!(
        "{}/{}/libmopro_bindings.dylib",
        target_archs[0].as_str(),
        mode.as_str()
    ));

    generate_ios_bindings(&out_dylib_path, &swift_bindings_dir)
        .expect("Failed to generate bindings for iOS");

    fs::rename(
        swift_bindings_dir.join("mopro.swift"),
        bindings_out.join("mopro.swift"),
    )
    .expect("Failed to move mopro.swift into place");

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

    // The iOS project expects the module map files to be named "module.modulemap",
    // but uniffi-bindgen creates "moproFFI.modulemap" files by default,
    // therefore we need to rename all of them.
    rename_module_maps_recursively(&bindings_out);

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

// More general cases
fn group_target_archs(target_archs: &[IosArch]) -> Vec<Vec<IosArch>> {
    // Detect the current architecture
    let current_arch = std::env::consts::ARCH;

    // Determine the device architecture prefix based on the current architecture
    let device_prefix = match current_arch {
        arch if arch.starts_with(ARCH_X86_64) => ARCH_X86_64,
        arch if arch.starts_with(ARCH_ARM_64) => ARCH_ARM_64,
        _ => panic!("Unsupported host architecture: {}", current_arch),
    };

    let mut device_archs = Vec::new();
    let mut simulator_archs = Vec::new();

    target_archs.iter().for_each(|&arch| {
        let arch_str = arch.as_str();
        if arch_str.ends_with("sim") {
            simulator_archs.push(arch);
        } else if arch_str.starts_with(device_prefix) {
            device_archs.push(arch);
        } else {
            simulator_archs.push(arch);
        }
    });

    let mut grouped_archs = Vec::new();
    if !device_archs.is_empty() {
        grouped_archs.push(device_archs);
    }
    if !simulator_archs.is_empty() {
        grouped_archs.push(simulator_archs);
    }

    grouped_archs
}

/// Recursively renames all module maps in the given directory to "module.modulemap".
/// This is necessary because uniffi-bindgen creates module maps with the name "moproFFI.modulemap"
/// by default. We're looking for multiple files as there are separate modules for the
/// physical device and the simulator.
fn rename_module_maps_recursively(bindings_out: &PathBuf) {
    for entry in fs::read_dir(bindings_out).expect("Failed to read bindings out directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() == "moproFFI.modulemap" {
            let dest_path = path.with_file_name("module.modulemap");
            fs::rename(&path, &dest_path).expect("Failed to rename module map");
        } else if path.is_dir() {
            rename_module_maps_recursively(&path);
        }
    }
}

fn generate_ios_bindings(dylib_path: &Path, binding_dir: &Path) -> anyhow::Result<()> {
    if binding_dir.exists() {
        fs::remove_dir_all(binding_dir)?;
    }

    generate_bindings_library_mode(
        Utf8Path::from_path(dylib_path)
            .ok_or(Error::new(ErrorKind::InvalidInput, "Invalid dylib path"))?,
        None,
        &SwiftBindingGenerator,
        &CargoMetadataConfigSupplier::default(),
        None,
        Utf8Path::from_path(binding_dir).ok_or(Error::new(
            ErrorKind::InvalidInput,
            "Invalid swift files directory",
        ))?,
        true,
    )
    .map_err(|e| Error::other(e.to_string()))?;
    Ok(())
}
