use anyhow::Context;
use camino::Utf8Path;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;
use uniffi::{generate, GenerateOptions, TargetLanguage};

use crate::app_config::{project_name_from_toml, PlatformBuilder};

use super::cleanup_tmp_local;
use super::constants::{
    AndroidArch, AndroidPlatform, Arch, Mode, ANDROID_BINDINGS_DIR, ANDROID_KT_FILE,
    ANDROID_PACKAGE_NAME, ARCH_ARM_64_V8, ARCH_ARM_V7_ABI, ARCH_I686, ARCH_X86_64,
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
            latest_out_lib_path = build_for_arch(
                arch,
                &lib_name,
                project_dir,
                &build_dir,
                &bindings_out,
                mode,
            )
            .context(format!(
                "Failed to build for architecture: {}",
                arch.as_str()
            ))?;
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
    project_dir: &Path,
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
        // Raise the min API: the barretenberg Android prebuilt imports symbols
        // (e.g. __tls_get_addr, API 29 on x86_64) absent from cargo-ndk's default 21.
        .arg("--platform")
        .arg("30")
        .arg("build")
        .arg("--link-libcxx-shared")
        .arg("--lib");
    if mode == Mode::Release {
        build_cmd.arg("--release");
    }

    // barretenberg-rs's build.rs matches "linux" before "android", so for
    // *-linux-android it fetches the glibc prebuilt Android can't dlopen.
    // BB_LIB_DIR overrides that download; point it at the correct archive.
    if let Some(bb_lib_dir) = setup_barretenberg_android_lib(arch, project_dir, build_dir)? {
        build_cmd.env("BB_LIB_DIR", &bb_lib_dir);
    }

    build_cmd
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
    let out_lib_dest = bindings_out.join(format!("jniLibs/{folder}/{lib_name}"));

    let parent_dir = out_lib_dest.parent().context(format!(
        "Failed to get parent directory for {}",
        out_lib_dest.display()
    ))?;

    fs::create_dir_all(parent_dir).context("Failed to create jniLibs directory")?;
    fs::copy(&out_lib_path, &out_lib_dest).context("Failed to copy file")?;

    Ok(out_lib_path)
}

/// Fetch the correct barretenberg Android static library for `arch` and return
/// the dir to expose via `BB_LIB_DIR`. `None` when the project does not use
/// barretenberg-rs or the arch has no published Android prebuilt.
fn setup_barretenberg_android_lib(
    arch: AndroidArch,
    project_dir: &Path,
    build_dir: &Path,
) -> anyhow::Result<Option<PathBuf>> {
    // Aztec only publishes static Android prebuilts for arm64 and x86_64.
    let bb_arch = match arch {
        AndroidArch::X8664Linux => "x86_64-android",
        AndroidArch::Aarch64Linux => "arm64-android",
        _ => return Ok(None),
    };

    let Some(version) = barretenberg_rs_version(project_dir)? else {
        return Ok(None);
    };

    let dest = build_dir.join("bb-android-prebuilt").join(bb_arch);
    let lib = dest.join("libbb-external.a");
    if lib.exists() {
        return Ok(Some(dest));
    }
    fs::create_dir_all(&dest).context("Failed to create barretenberg prebuilt dir")?;

    let url = format!(
        "https://github.com/AztecProtocol/aztec-packages/releases/download/v{version}/barretenberg-static-{bb_arch}.tar.gz"
    );
    let tarball = dest.join("barretenberg-static.tar.gz");

    let status = Command::new("curl")
        .args(["-L", "-f", "-o"])
        .arg(&tarball)
        .arg(&url)
        .status()
        .context("Failed to run curl for barretenberg Android prebuilt")?;
    if !status.success() {
        anyhow::bail!("Failed to download barretenberg Android prebuilt from {url}");
    }

    let status = Command::new("tar")
        .arg("-xzf")
        .arg(&tarball)
        .arg("-C")
        .arg(&dest)
        .status()
        .context("Failed to run tar for barretenberg Android prebuilt")?;
    if !status.success() {
        anyhow::bail!("Failed to extract barretenberg Android prebuilt");
    }
    let _ = fs::remove_file(&tarball);

    if !lib.exists() {
        anyhow::bail!(
            "barretenberg Android prebuilt extracted but libbb-external.a is missing in {}",
            dest.display()
        );
    }
    Ok(Some(dest))
}

/// Resolve the pinned barretenberg-rs version from `project_dir`'s `Cargo.lock`
/// (generating one if absent). `None` when the crate is not a dependency.
fn barretenberg_rs_version(project_dir: &Path) -> anyhow::Result<Option<String>> {
    let lock_path = project_dir.join("Cargo.lock");
    if !lock_path.exists() {
        // We need the resolved version before cargo ndk would create the lock.
        let status = Command::new("cargo")
            .arg("generate-lockfile")
            .arg("--manifest-path")
            .arg(project_dir.join("Cargo.toml"))
            .status()
            .context("Failed to run cargo generate-lockfile")?;
        if !status.success() {
            return Ok(None);
        }
    }
    let lock = match fs::read_to_string(&lock_path) {
        Ok(lock) => lock,
        Err(_) => return Ok(None),
    };
    Ok(parse_barretenberg_rs_version(&lock))
}

/// Extract the `barretenberg-rs` package version from a `Cargo.lock` body.
fn parse_barretenberg_rs_version(lock: &str) -> Option<String> {
    let mut lines = lock.lines();
    while let Some(line) = lines.next() {
        if line.trim() == r#"name = "barretenberg-rs""# {
            for next in lines.by_ref() {
                let trimmed = next.trim();
                if let Some(rest) = trimmed.strip_prefix(r#"version = ""#) {
                    return rest.strip_suffix('"').map(str::to_string);
                }
                if trimmed == "[[package]]" {
                    break;
                }
            }
        }
    }
    None
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

    generate(GenerateOptions {
        languages: vec![TargetLanguage::Kotlin],
        source: Utf8Path::from_path(dylib_path)
            .ok_or(Error::new(ErrorKind::InvalidInput, "Invalid dylib path"))?
            .to_path_buf(),
        out_dir: Utf8Path::from_path(binding_dir)
            .ok_or(Error::new(
                ErrorKind::InvalidInput,
                "Invalid kotlin files directory",
            ))?
            .to_path_buf(),
        crate_filter: None,
        ..GenerateOptions::default()
    })
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

    if generated_kt_file.eq(&out_android_kt_file) {
        return Ok(());
    }

    fs::create_dir(bindings_out.join("uniffi").join(out_android_module_name))
        .context("Failed to create new package directory")?;
    fs::rename(generated_kt_file, &out_android_kt_file).context("Failed to move kotlin file")?;
    fs::remove_dir(bindings_out.join("uniffi").join(gen_android_module_name))
        .context("Failed to remove gen android kotlin package directory")?;

    // Remove `package uniffi.<gen_android_module_name>` from the generated Kotlin file
    let content =
        fs::read_to_string(&out_android_kt_file).context("Failed to read generated Kotlin file")?;
    let modified_content = content.replace(
        &format!("package uniffi.{gen_android_module_name}"),
        &format!("package uniffi.{out_android_module_name}"),
    );
    fs::write(&out_android_kt_file, modified_content)
        .context("Failed to write modified Kotlin file")
}

#[cfg(test)]
mod tests {
    use super::parse_barretenberg_rs_version;

    #[test]
    fn parses_barretenberg_rs_version_from_lockfile() {
        let lock = r#"
[[package]]
name = "anyhow"
version = "1.0.86"

[[package]]
name = "barretenberg-rs"
version = "4.2.0-aztecnr-rc.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
dependencies = [
 "serde",
]

[[package]]
name = "camino"
version = "1.1.9"
"#;
        assert_eq!(
            parse_barretenberg_rs_version(lock).as_deref(),
            Some("4.2.0-aztecnr-rc.2")
        );
    }

    #[test]
    fn returns_none_when_barretenberg_rs_absent() {
        let lock = r#"
[[package]]
name = "anyhow"
version = "1.0.86"

[[package]]
name = "camino"
version = "1.1.9"
"#;
        assert_eq!(parse_barretenberg_rs_version(lock), None);
    }

    #[test]
    fn does_not_confuse_a_dependency_mention_with_the_package_entry() {
        // A dependency mention must not be read as the package's version stanza.
        let lock = r#"
[[package]]
name = "noir"
version = "1.0.0-beta.19"
dependencies = [
 "barretenberg-rs",
]

[[package]]
name = "barretenberg-rs"
version = "4.3.0"
"#;
        assert_eq!(
            parse_barretenberg_rs_version(lock).as_deref(),
            Some("4.3.0")
        );
    }
}
