use anyhow::Context;
use camino::Utf8Path;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;
use uniffi::{generate, GenerateOptions, TargetLanguage};

use mopro_build_common::{
    build_from_env, cleanup_tmp_local, install_arch, mktemp_local, project_name_from_toml,
    Arch, IosArch, Mode, PlatformBuilder,
    ARCH_ARM_64, ARCH_X86_64, IOS_BINDINGS_DIR, IOS_SWIFT_FILE, IOS_XCFRAMEWORKS_DIR,
};

pub struct IosPlatform;

pub fn build() {
    build_from_env::<IosPlatform>()
}

#[derive(Default)]
pub struct IosBindingsParams {
    pub using_noir: bool,
}

impl PlatformBuilder for IosPlatform {
    type Arch = IosArch;
    type Params = IosBindingsParams;

    fn identifier() -> &'static str { "ios" }

    fn build(
        mode: Mode,
        project_dir: &Path,
        target_archs: Vec<Self::Arch>,
        params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        let uniffi_style_identifier = project_name_from_toml(project_dir)
            .expect("Failed to get project name from Cargo.toml");

        let gen_swift_file_name = format!("{uniffi_style_identifier}.swift");
        let lib_name = format!("lib{uniffi_style_identifier}.a");
        let header_name = format!("{uniffi_style_identifier}FFI.h");
        let modulemap_name = format!("{uniffi_style_identifier}FFI.modulemap");

        let build_dir_path = project_dir.join("build");
        let work_dir = mktemp_local(&build_dir_path);
        let swift_bindings_dir = work_dir.join("SwiftBindings");
        let bindings_out = work_dir.join(IOS_BINDINGS_DIR);
        fs::create_dir(&bindings_out).expect("Failed to create bindings out directory");
        let bindings_dest = project_dir.join(IOS_BINDINGS_DIR);
        let framework_out = bindings_out.join(IOS_XCFRAMEWORKS_DIR);

        let build_combined_archs = |archs: &[IosArch]| -> PathBuf {
            let out_lib_paths: Vec<PathBuf> = archs.iter().map(|arch| {
                build_dir_path.join(format!("{}/{}/{}", arch.as_str(), mode.as_str(), lib_name))
            }).collect();
            for arch in archs {
                install_arch(arch.as_str().to_string());
                let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
                let mut build_cmd = Command::new(&cargo);
                build_cmd.arg("build");
                if mode == Mode::Release { build_cmd.arg("--release"); }
                if params.using_noir { build_cmd.env("IPHONEOS_DEPLOYMENT_TARGET", "15.0"); }
                build_cmd
                    .arg("--lib")
                    .env("CARGO_BUILD_TARGET_DIR", &build_dir_path)
                    .env("CARGO_BUILD_TARGET", arch.as_str())
                    .spawn().expect("Failed to spawn cargo build")
                    .wait().expect("cargo build errored");
            }
            let mut lipo_cmd = Command::new("lipo");
            let lib_out = mktemp_local(&build_dir_path).join(&lib_name);
            lipo_cmd.arg("-create").arg("-output").arg(lib_out.to_str().unwrap());
            for p in out_lib_paths { lipo_cmd.arg(p.to_str().unwrap()); }
            lipo_cmd.spawn().expect("Failed to spawn lipo").wait().expect("lipo failed");
            lib_out
        };

        let out_lib_paths: Vec<PathBuf> = group_target_archs(&target_archs)
            .iter().map(|v| build_combined_archs(v)).collect();

        let out_dylib_path = build_dir_path.join(format!(
            "{}/{}/{}",
            target_archs[0].as_str(), mode.as_str(),
            lib_name.replace(".a", ".dylib")
        ));

        generate_ios_bindings(&out_dylib_path, &swift_bindings_dir)
            .expect("Failed to generate bindings for iOS");

        fs::rename(
            swift_bindings_dir.join(&gen_swift_file_name),
            bindings_out.join(IOS_SWIFT_FILE),
        ).context(format!(
            "Failed to rename bindings from {}/{gen_swift_file_name}",
            swift_bindings_dir.display()
        ))?;

        let mut xcbuild_cmd = Command::new("xcodebuild");
        if params.using_noir { xcbuild_cmd.env("IPHONEOS_DEPLOYMENT_TARGET", "15.0"); }
        xcbuild_cmd.arg("-create-xcframework");
        for lib_path in out_lib_paths {
            xcbuild_cmd
                .arg("-library").arg(lib_path.to_str().unwrap())
                .arg("-headers").arg(swift_bindings_dir.to_str().unwrap());
        }
        xcbuild_cmd
            .arg("-output").arg(framework_out.to_str().unwrap())
            .spawn().context("Failed to spawn xcodebuild")?
            .wait().context("xcodebuild failed")?;

        regroup_header_artifacts(&framework_out, &header_name, &modulemap_name, &uniffi_style_identifier)
            .expect("Failed to regroup header artifacts");

        if let Ok(info) = fs::metadata(&bindings_dest) {
            if !info.is_dir() { panic!("framework directory exists and is not a directory"); }
            fs::remove_dir_all(&bindings_dest).expect("Failed to remove framework directory");
        }
        fs::rename(&bindings_out, &bindings_dest).expect("Failed to move framework into place");
        cleanup_tmp_local(&build_dir_path);

        Ok(bindings_dest)
    }
}

fn group_target_archs(target_archs: &[IosArch]) -> Vec<Vec<IosArch>> {
    let current_arch = std::env::consts::ARCH;
    let device_prefix = match current_arch {
        arch if arch.starts_with(ARCH_X86_64) => ARCH_X86_64,
        arch if arch.starts_with(ARCH_ARM_64) => ARCH_ARM_64,
        _ => panic!("Unsupported host architecture: {current_arch}"),
    };

    let mut device_archs = Vec::new();
    let mut simulator_archs = Vec::new();
    target_archs.iter().for_each(|&arch| {
        let arch_str = arch.as_str();
        if arch_str.ends_with("sim") { simulator_archs.push(arch); }
        else if arch_str.starts_with(device_prefix) { device_archs.push(arch); }
        else { simulator_archs.push(arch); }
    });

    let mut grouped = Vec::new();
    if !device_archs.is_empty() { grouped.push(device_archs); }
    if !simulator_archs.is_empty() { grouped.push(simulator_archs); }
    grouped
}

pub fn regroup_header_artifacts(
    framework_out: &Path,
    header_name: &str,
    modulemap_name: &str,
    project_name: &str,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(framework_out).with_context(|| format!("reading {framework_out:?}"))? {
        let entry = entry?;
        let arch_path = entry.path();
        if !arch_path.is_dir() { continue; }

        let headers_dir = arch_path.join("Headers");
        if !headers_dir.is_dir() { continue; }

        let modmap_src = headers_dir.join(modulemap_name);
        let header_src = headers_dir.join(header_name);
        let target_dir = headers_dir.join(project_name);
        fs::create_dir_all(&target_dir).with_context(|| format!("creating {target_dir:?}"))?;

        if modmap_src.exists() {
            fs::rename(&modmap_src, target_dir.join("module.modulemap"))
                .with_context(|| format!("moving {modmap_src:?}"))?;
        }
        if header_src.exists() {
            fs::rename(&header_src, target_dir.join(header_name))
                .with_context(|| format!("moving {header_src:?}"))?;
        }
    }
    Ok(())
}

fn generate_ios_bindings(dylib_path: &Path, binding_dir: &Path) -> anyhow::Result<()> {
    if binding_dir.exists() { fs::remove_dir_all(binding_dir)?; }

    generate(GenerateOptions {
        languages: vec![TargetLanguage::Swift],
        source: Utf8Path::from_path(dylib_path)
            .ok_or(Error::new(ErrorKind::InvalidInput, "Invalid dylib path"))?.to_path_buf(),
        out_dir: Utf8Path::from_path(binding_dir)
            .ok_or(Error::new(ErrorKind::InvalidInput, "Invalid swift files directory"))?.to_path_buf(),
        crate_filter: None,
        ..GenerateOptions::default()
    }).map_err(|e| Error::other(e.to_string()))?;
    Ok(())
}
