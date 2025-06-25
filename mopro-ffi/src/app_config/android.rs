use anyhow::Context;
use camino::Utf8Path;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;
use uniffi::generate_bindings_library_mode;
use uniffi::CargoMetadataConfigSupplier;
use uniffi::KotlinBindingGenerator;

use crate::app_config::{project_name_from_toml, PlatformBuilder};

use super::cleanup_tmp_local;
use super::constants::{
    AndroidArch, AndroidPlatform, Arch, Mode, ANDROID_BINDINGS_DIR, ANDROID_KT_FILE,
    ANDROID_PACKAGE_NAME, ARCH_ARM_64_V8, ARCH_ARM_V7_ABI, ARCH_I686, ARCH_X86_64,
    BUILD_BINDINGS_ENV,
};
use super::install_arch;
use super::install_ndk;
use super::mktemp_local;

// Maintained for backwards compatibility
#[inline]
pub fn build() {
    super::build_from_env::<AndroidPlatform>()
}

pub type AndroidBindingsParams = ();

impl PlatformBuilder for AndroidPlatform {
    type Arch = AndroidArch;
    type Params = AndroidBindingsParams;

    fn build(
        mode: Mode,
        project_dir: &Path,
        target_archs: Vec<Self::Arch>,
        _params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        let uniffi_style_identifier = project_name_from_toml(project_dir)
            .expect("Failed to get project name from Cargo.toml");

        // Names for the files that will be outputted (can be changed)
        let binding_dir_name = ANDROID_BINDINGS_DIR;
        let out_android_package_name = ANDROID_PACKAGE_NAME;
        let out_android_kt_file_name = ANDROID_KT_FILE;

        // Names for the generated files by uniffi
        let lib_name = format!("lib{}.so", &uniffi_style_identifier);
        let gen_android_module_name = &uniffi_style_identifier;
        let gen_android_kt_file_name = format!("{}.kt", &uniffi_style_identifier);

        #[cfg(feature = "witnesscalc")]
        let _ = std::env::var("ANDROID_NDK").context("ANDROID_NDK is not set")?;

        // Paths for the generated files
        let build_dir = Path::new(&project_dir).join("build");
        let work_dir = mktemp_local(&build_dir);
        let bindings_out = work_dir.join(binding_dir_name);
        let bindings_dest = Path::new(&project_dir).join(binding_dir_name);

        install_ndk();
        let mut latest_out_lib_path = PathBuf::new();
        for arch in target_archs {
            latest_out_lib_path =
                build_for_arch(arch, &lib_name, &build_dir, &bindings_out, mode).context(
                    format!("Failed to build for architecture: {}", arch.as_str()),
                )?;
        }

        generate_android_bindings(&latest_out_lib_path, &bindings_out)
            .expect("Failed to generate bindings");

        reformat_kotlin_package(
            gen_android_module_name,
            &gen_android_kt_file_name,
            out_android_package_name,
            &out_android_kt_file_name,
            &bindings_out,
        )
        .expect("Failed to reformat generated Kotlin package");

        move_bindings(&bindings_out, &bindings_dest);
        cleanup_tmp_local(&build_dir);

        Ok(bindings_out)
    }
}

fn build_for_arch(
    arch: AndroidArch,
    lib_name: &str,
    build_dir: &Path,
    bindings_out: &Path,
    mode: Mode,
) -> anyhow::Result<PathBuf> {
    let arch_str = arch.as_str();
    install_arch(arch_str.to_string());
    let cpp_lib_dest = bindings_out.join("jniLibs");

    let mut build_cmd = Command::new("cargo");
    build_cmd
        .arg("ndk")
        .arg("-t")
        .arg(arch_str)
        .arg("build")
        .arg("--lib");
    if mode == Mode::Release {
        build_cmd.arg("--release");
    }
    build_cmd
        .env_remove("CARGO_MAKEFLAGS") // Remove CARGO_MAKEFLAGS to avoid deadlock when run inside the build script
        .env_remove(BUILD_BINDINGS_ENV) // Remove the environment variable that indicates that we want to build bindings to prevent build.rs from running build bindings again
        .env("CARGO_BUILD_TARGET_DIR", build_dir)
        .env("CARGO_BUILD_TARGET", arch_str)
        .env("CARGO_NDK_OUTPUT_PATH", cpp_lib_dest)
        .spawn()
        .expect("Failed to spawn cargo build")
        .wait()
        .expect("cargo build errored");

    let folder = match arch {
        AndroidArch::X8664Linux => ARCH_X86_64,
        AndroidArch::I686Linux => ARCH_I686,
        AndroidArch::Armv7LinuxAbi => ARCH_ARM_V7_ABI,
        AndroidArch::Aarch64Linux => ARCH_ARM_64_V8,
    };

    let out_lib_path = build_dir.join(format!(
        "{}/{}/{}/{}",
        build_dir.display(),
        arch_str,
        mode.as_str(),
        lib_name
    ));
    let out_lib_dest = bindings_out.join(format!("jniLibs/{}/{}", folder, lib_name));

    let parent_dir = out_lib_dest.parent().context(format!(
        "Failed to get parent directory for {}",
        out_lib_dest.display()
    ))?;

    fs::create_dir_all(parent_dir).context("Failed to create jniLibs directory")?;
    fs::copy(&out_lib_path, &out_lib_dest).context("Failed to copy file")?;

    Ok(out_lib_path)
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

fn generate_android_bindings(dylib_path: &Path, binding_dir: &Path) -> anyhow::Result<()> {
    let content = "[bindings.kotlin]\nandroid = true";
    let parent_dir = binding_dir
        .parent()
        .context("Failed to get parent directory")?;
    let config_path = parent_dir.join("uniffi_config.toml");
    fs::write(&config_path, content).expect("Failed to write uniffi_config.toml");

    generate_bindings_library_mode(
        Utf8Path::from_path(dylib_path)
            .ok_or(Error::new(ErrorKind::InvalidInput, "Invalid dylib path"))?,
        None,
        &KotlinBindingGenerator,
        &CargoMetadataConfigSupplier::default(),
        None,
        Utf8Path::from_path(binding_dir).ok_or(Error::new(
            ErrorKind::InvalidInput,
            "Invalid kotlin files directory",
        ))?,
        true,
    )
    .map_err(|e| Error::other(e.to_string()))?;
    Ok(())
}

fn reformat_kotlin_package(
    gen_android_module_name: &str,
    gen_android_kt_file_name: &str,
    out_android_module_name: &str,
    out_android_kt_file_name: &&str,
    bindings_out: &Path,
) -> anyhow::Result<()> {
    let generated_kt_file = bindings_out
        .join("uniffi")
        .join(gen_android_module_name)
        .join(gen_android_kt_file_name);
    let out_android_kt_file = bindings_out
        .join("uniffi")
        .join(out_android_module_name)
        .join(out_android_kt_file_name);

    fs::create_dir(bindings_out.join("uniffi").join(out_android_module_name))
        .context("Failed to create new package directory")?;
    fs::rename(generated_kt_file, &out_android_kt_file).context("Failed to move kotlin file")?;
    fs::remove_dir(bindings_out.join("uniffi").join(gen_android_module_name))
        .context("Failed to remove gen android kotlin package directory")?;

    // Remove `package uniffi.<gen_android_module_name>` from the generated Kotlin file
    let content =
        fs::read_to_string(&out_android_kt_file).context("Failed to read generated Kotlin file")?;
    let modified_content = content.replace(
        &format!("package uniffi.{}", gen_android_module_name),
        &format!("package uniffi.{}", out_android_module_name),
    );
    fs::write(&out_android_kt_file, modified_content)
        .context("Failed to write modified Kotlin file")
}
