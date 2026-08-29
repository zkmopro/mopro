//! Noir/barretenberg-specific React Native Android build.
//!
//! Mirrors `android_noir`'s handling for the plain Android builder (see
//! `super::android_noir`), but `uniffi-bindgen-react-native` needs a different build
//! strategy than a single `cargo ndk` invocation:
//!
//! - It can't take per-target env in one multi-target invocation, and each
//!   arch may need a different `BB_LIB_DIR` (barretenberg's Android prebuilt
//!   is arch-specific), so each arch is built in its own invocation.
//! - Its `--and-generate` reads UniFFI metadata straight from the compiled
//!   Android `.so`, and the Zig-linked build (needed to link barretenberg's
//!   prebuilt) has stripped the `.symtab` that needs. So metadata is instead
//!   extracted from a separate NDK-linked (non-Zig) scratch build, mirroring
//!   the plain Android builder's `relink_with_ndk_for_bindgen`, via
//!   `uniffi-bindgen-react-native generate all <lib_file>`.

use anyhow::Context;
use mopro_ffi::app_config::android::AndroidBindingsParams;
use mopro_ffi::app_config::constants::Mode;
use mopro_ffi::app_config::project_name_from_toml;
use mopro_ffi::app_config::react_native::{remove_unrequested_wasm_stubs, android_abi_for_triple};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Build every requested Android arch for React Native with `params`' overrides
/// applied, then generate JSI/turbo-module bindings from a separate NDK-linked
/// scratch build. Assumes `bindings_dir` is already set up (template copied,
/// `ubrn.config.yaml` pointed at `project_dir`, `generate jsi turbo-module` run).
pub fn build(
    project_dir: &Path,
    bindings_dir: &Path,
    android_arch_strings: &[String],
    mode: Mode,
    params: &AndroidBindingsParams,
) -> anyhow::Result<()> {
    patch_ubrn_config_for_noir(bindings_dir, params)?;

    let lib_name = format!(
        "lib{}.so", 
        project_name_from_toml(project_dir)
            .context("Failed to get project name from Cargo.toml")?
    );
    
    let target_dir = project_dir.join("target");
    let jni_libs = bindings_dir.join("android/src/main/jniLibs");

    // `ubrn build android` wipes jniLibs on every invocation, and Noir needs one
    // invocation per arch (each has its own BB_LIB_DIR and Zig linker), so the
    // last arch would be the only survivor. Wipe once here and install each arch
    // below, with the builds themselves run under `--no-jniLibs`
    if jni_libs.exists() {
        fs::remove_dir_all(&jni_libs).context("Failed to clear jniLibs")?;
    }

    for arch in android_arch_strings {
        let env = params
            .arch_overrides
            .get(arch)
            .map(|config| config.extra_env.clone())
            .unwrap_or_default();
        build_android_once(mode, arch, bindings_dir, &target_dir, &env)?;
        install_jni_lib(arch, mode, &target_dir, &jni_libs, &lib_name)?;
    }

    let bindgen_arch = android_arch_strings
        .first()
        .context("No Android architectures provided for binding generation")?;
    let bindgen_env = params
        .arch_overrides
        .get(bindgen_arch)
        .map(|config| config.extra_env.clone())
        .unwrap_or_default();
    let bindgen_lib = build_ndk_linked_lib_for_bindgen(
        project_dir,
        bindgen_arch,
        mode,
        params.min_sdk_version,
        &bindgen_env,
    )?;

    let status = Command::new("uniffi-bindgen-react-native")
        .args([
            "generate",
            "all",
            &bindgen_lib.to_string_lossy(),
            "--flavor",
            "jsi",
        ])
        .current_dir(bindings_dir)
        .status()
        .expect("failed to generate react native bindings");
    if !status.success() {
        anyhow::bail!(
            "Failed to generate react native bindings from {}",
            bindgen_lib.display()
        );
    }

    remove_unrequested_wasm_stubs(bindings_dir)?;
    Ok(())
}

fn build_android_once(
    mode: Mode,
    target_string: &str,
    bindings_dir: &Path,
    target_dir: &Path,
    env: &[(String, String)],
) -> anyhow::Result<()> {
    let mut args = vec![
        "build".to_string(),
        "android".to_string(),
        "--targets".to_string(),
        target_string.to_string(),
        "--no-jniLibs".to_string(),
    ];
    if mode == Mode::Release {
        args.push("--release".to_string());
    }

    let mut cmd = Command::new("uniffi-bindgen-react-native");
    cmd.args(&args)
        .current_dir(bindings_dir)
        .env("CARGO_TARGET_DIR", target_dir);
    for (key, value) in env {
        cmd.env(key, value);
    }
    let status = cmd.status().expect("failed to build react native bindings");
    if !status.success() {
        anyhow::bail!("Failed to build react native bindings for {target_string}");
    }
    Ok(())
}

/// Build `arch` with the plain NDK linker (no Zig override) purely to extract
/// UniFFI metadata via `uniffi-bindgen-react-native generate all <lib_file>`.
/// This lib is never shipped or run; a separate target dir keeps the
/// already-built (Zig-linked) jniLibs untouched.
fn build_ndk_linked_lib_for_bindgen(
    project_dir: &Path,
    arch: &str,
    mode: Mode,
    min_sdk_version: Option<u32>,
    extra_env: &[(String, String)],
) -> anyhow::Result<PathBuf> {
    let lib_name = format!(
        "lib{}.so",
        project_name_from_toml(project_dir)
            .context("Failed to get project name from Cargo.toml")?
    );
    let bindgen_target = project_dir.join("build").join("rn-bindgen");

    let mut cmd = Command::new("cargo");
    cmd.arg("ndk").arg("-t").arg(arch);
    if let Some(min_sdk) = min_sdk_version {
        cmd.arg("--platform").arg(min_sdk.to_string());
    }
    cmd.arg("build").arg("--link-libcxx-shared").arg("--lib");
    if mode == Mode::Release {
        cmd.arg("--release");
    }
    for (key, value) in extra_env {
        cmd.env(key, value);
    }
    cmd.env("CARGO_BUILD_TARGET_DIR", &bindgen_target)
        .env("CARGO_BUILD_TARGET", arch)
        .env("CARGO_NDK_OUTPUT_PATH", bindgen_target.join("jniLibs"));

    let status = cmd
        .status()
        .expect("failed to build NDK-linked lib for bindgen");
    if !status.success() {
        anyhow::bail!("cargo ndk build (bindgen lib) failed for {arch}");
    }

    let profile_dir = mode.as_str();
    let out_lib_path = bindgen_target.join(arch).join(profile_dir).join(&lib_name);
    if !out_lib_path.exists() {
        anyhow::bail!(
            "NDK bindgen lib missing at {} (needed for uniffi metadata)",
            out_lib_path.display()
        );
    }
    Ok(out_lib_path)
}

/// Bump `ubrn.config.yaml`'s Android `apiLevel` and add `cargoExtras` so
/// `uniffi-bindgen-react-native`'s own `cargo ndk` invocation picks up the
/// per-arch linker override. This can't go through `RUSTFLAGS`/
/// `CARGO_TARGET_<triple>_RUSTFLAGS`: ubrn already sets the latter itself (for
/// a 16KB-page-size flag) on every target build, which would clobber ours.
fn patch_ubrn_config_for_noir(
    bindings_dir: &Path,
    params: &AndroidBindingsParams,
) -> anyhow::Result<()> {
    let config_path = bindings_dir.join("ubrn.config.yaml");
    let mut contents =
        fs::read_to_string(&config_path).context("Failed to read ubrn.config.yaml")?;

    if let Some(min_sdk) = params.min_sdk_version {
        if let Some(pos) = contents.find("apiLevel: ") {
            let line_end = contents[pos..]
                .find('\n')
                .map(|i| pos + i)
                .unwrap_or(contents.len());
            let current: u32 = contents[pos + "apiLevel: ".len()..line_end]
                .trim()
                .parse()
                .unwrap_or(0);
            if min_sdk > current {
                contents.replace_range(pos..line_end, &format!("apiLevel: {min_sdk}"));
            }
        }
    }

    let cargo_extras: Vec<String> = params
        .arch_overrides
        .iter()
        .filter_map(|(triple, config)| {
            config
                .linker
                .as_ref()
                .map(|linker| format!("target.{triple}.linker='{linker}'"))
        })
        .flat_map(|entry| ["--config".to_string(), entry])
        .collect();

    if !cargo_extras.is_empty() {
        let yaml_list = cargo_extras
            .iter()
            .map(|entry| format!("      - \"{}\"", entry.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join("\n");
        contents.push_str(&format!("\n    cargoExtras:\n{yaml_list}\n"));
    }

    fs::write(&config_path, contents).context("Failed to write ubrn.config.yaml")?;
    Ok(())
}

/// helper to copy the built library through ubrn into `jniLibs/<abi>/`
/// The copying step was suppressed in `build_android_once` by `--no-jniLibs`.
fn install_jni_lib(
    arch: &str,
    mode: Mode,
    target_dir: &Path,
    jni_libs: &Path,
    lib_name: &str,
) -> anyhow::Result<()> {
    let abi = android_abi_for_triple(arch)
        .with_context(|| format!("No Android ABI known for target triple {arch}"))?;
    let src = target_dir.join(arch).join(mode.as_str()).join(lib_name);
    let dst_dir = jni_libs.join(abi);

    fs::create_dir_all(&dst_dir).context("Failed to create jniLibs directory")?;
    fs::copy(&src, dst_dir.join(lib_name))
        .with_context(|| format!("Failed to copy {} into {}", src.display(), dst_dir.display()))?;
    Ok(())
}
