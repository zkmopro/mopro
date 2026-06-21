use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use mopro_build_common::{
    build_from_env, Arch, PlatformBuilder,
    REACT_NATIVE_BINDINGS_DIR,
};
pub use mopro_build_common::{ReactNativeArch, Mode};

pub struct ReactNativePlatform;

pub fn build() {
    build_from_env::<ReactNativePlatform>()
}

pub fn build_at(project_dir: &std::path::Path) {
    mopro_build_common::build_from_env_at::<ReactNativePlatform>(project_dir)
}

impl PlatformBuilder for ReactNativePlatform {
    type Arch = ReactNativeArch;
    type Params = ();

    fn identifier() -> &'static str { "react-native" }

    fn build(
        mode: Mode,
        project_dir: &Path,
        target_archs: Vec<Self::Arch>,
        _params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        install_uniffi_bindgen_react_native()?;

        fs::create_dir_all(project_dir.join(REACT_NATIVE_BINDINGS_DIR))
            .expect("failed to create bindings directory");

        let template_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("template");
        let mut copy_options = fs_extra::dir::CopyOptions::new();
        copy_options.overwrite = true;
        copy_options.content_only = true;
        fs_extra::dir::copy(
            &template_dir,
            project_dir.join(REACT_NATIVE_BINDINGS_DIR),
            &copy_options,
        ).with_context(|| format!("Failed to copy react_native template from {:?}", template_dir))?;

        let target_file = project_dir.join(REACT_NATIVE_BINDINGS_DIR).join("ubrn.config.yaml");
        let contents = fs::read_to_string(&target_file)
            .with_context(|| format!("Failed to read ubrn.config.yaml from {:?}", target_file))?
            .replace("<%PATH_TO_PROJECT%>", &project_dir.to_string_lossy());
        fs::write(&target_file, contents)
            .with_context(|| format!("Failed to write ubrn.config.yaml to {:?}", target_file))?;

        generate_react_native_bindings(project_dir, target_archs, mode)?;
        Ok(PathBuf::from(REACT_NATIVE_BINDINGS_DIR))
    }
}

fn install_uniffi_bindgen_react_native() -> anyhow::Result<()> {
    match Command::new("uniffi-bindgen-react-native").output() {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let current_path: PathBuf = std::env::current_dir()?;
            let status = Command::new("git")
                .args(["clone", "https://github.com/jhugman/uniffi-bindgen-react-native.git"])
                .current_dir(current_path.clone())
                .status().expect("failed to download uniffi-bindgen-react-native");
            if !status.success() {
                return Err(anyhow::anyhow!("Failed to download uniffi-bindgen-react-native"));
            }
            let status = Command::new("cargo")
                .args(["install", "--path", "."])
                .current_dir(current_path.join("uniffi-bindgen-react-native/crates/ubrn_cli"))
                .status().expect("failed to install uniffi-bindgen-react-native");
            if !status.success() {
                return Err(anyhow::anyhow!("Failed to install uniffi-bindgen-react-native"));
            }
            fs::remove_dir_all(current_path.join("uniffi-bindgen-react-native"))
                .expect("failed to remove uniffi-bindgen-react-native");
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!("Failed to check for uniffi-bindgen-react-native: {}", e)),
    }
}

fn generate_react_native_bindings(
    project_dir: &Path,
    target_archs: Vec<ReactNativeArch>,
    mode: Mode,
) -> anyhow::Result<()> {
    let bindings_dir = project_dir.join(REACT_NATIVE_BINDINGS_DIR);
    let status = Command::new("uniffi-bindgen-react-native")
        .args(["generate", "jsi", "turbo-module"])
        .current_dir(bindings_dir.clone())
        .status().expect("failed to generate react native bindings");
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to generate react native bindings"));
    }

    let ios_targets = target_archs.iter()
        .filter(|a| a.as_str().contains("ios")).map(|a| a.as_str()).collect::<Vec<_>>().join(",");
    let android_targets = target_archs.iter()
        .filter(|a| a.as_str().contains("android")).map(|a| a.as_str()).collect::<Vec<_>>().join(",");

    if !ios_targets.is_empty() {
        build_for_arch("ios", mode, &ios_targets, &bindings_dir)?;
    } else if !android_targets.is_empty() {
        build_for_arch("android", mode, &android_targets, &bindings_dir)?;
    }

    let npm_status = Command::new("npm")
        .args(["pkg", "set", "files[]=*.xcframework/**"])
        .current_dir(bindings_dir)
        .status().expect("failed to set files in package.json");
    if !npm_status.success() {
        return Err(anyhow::anyhow!("Failed to set files in package.json"));
    }
    Ok(())
}

fn build_for_arch(platform: &str, mode: Mode, target_string: &str, bindings_dir: &Path) -> anyhow::Result<()> {
    let mut args = vec!["build".to_string(), platform.to_string(), "--and-generate".to_string()];
    if mode == Mode::Release { args.push("--release".to_string()); }
    args.push("--targets".to_string());
    args.push(target_string.to_string());

    let status = Command::new("uniffi-bindgen-react-native")
        .args(&args).current_dir(bindings_dir)
        .status().expect("failed to build react native bindings");
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to build react native bindings"));
    }
    Ok(())
}
