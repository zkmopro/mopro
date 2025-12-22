use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::app_config::constants::{
    Arch, Mode, ReactNativeArch, ReactNativePlatform, REACT_NATIVE_BINDINGS_DIR,
};

use super::PlatformBuilder;

// Maintained for backwards compatibility
#[inline]
pub fn build() {
    super::build_from_env::<ReactNativePlatform>()
}

impl PlatformBuilder for ReactNativePlatform {
    type Arch = ReactNativeArch;
    type Params = ();

    fn build(
        mode: Mode,
        project_dir: &Path,
        target_archs: Vec<Self::Arch>,
        _params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        install_uniffi_bindgen_react_native()?;

        fs::create_dir_all(project_dir.join(REACT_NATIVE_BINDINGS_DIR))
            .expect("failed to create bindings directory");

        // Copy the react_native template to the project directory
        // Get the path to the template directory relative to this source file
        let template_dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/app_config/template/react_native");
        let mut copy_options = fs_extra::dir::CopyOptions::new();
        copy_options.overwrite = true;
        copy_options.content_only = true;
        fs_extra::dir::copy(
            &template_dir,
            project_dir.join(REACT_NATIVE_BINDINGS_DIR),
            &copy_options,
        )
        .with_context(|| format!("Failed to copy react_native folder from {:?}", template_dir))?;

        // Replace the <%PATH_TO_PROJECT%> in the ubrn.config.yaml template with the project directory
        let target_file = project_dir
            .join(REACT_NATIVE_BINDINGS_DIR)
            .join("ubrn.config.yaml");

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
    let output = Command::new("uniffi-bindgen-react-native").output();
    match output {
        Ok(_) => {
            // Command exists, no need to install
            println!("uniffi-bindgen-react-native already installed.");
            return Ok(());
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Command not found, proceed with installation
            println!("uniffi-bindgen-react-native not found, installing...");
            let current_path: PathBuf = std::env::current_dir()?;
            let status = Command::new("git")
                .args([
                    "clone",
                    "https://github.com/jhugman/uniffi-bindgen-react-native.git",
                ])
                .current_dir(current_path.clone())
                .status()
                .expect("failed to download uniffi-bindgen-react-native");
            if !status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to download uniffi-bindgen-react-native"
                ));
            }

            let status = Command::new("cargo")
                .args(["install", "--path", "."])
                .current_dir(current_path.join("uniffi-bindgen-react-native/crates/ubrn_cli"))
                .status()
                .expect("failed to install uniffi-bindgen-react-native");
            if !status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to install uniffi-bindgen-react-native"
                ));
            }
            fs::remove_dir_all(current_path.join("uniffi-bindgen-react-native"))
                .expect("failed to remove uniffi-bindgen-react-native");
        }
        Err(e) => {
            // Other error, propagate it
            return Err(anyhow::anyhow!(
                "Failed to check for uniffi-bindgen-react-native: {}",
                e
            ));
        }
    }

    Ok(())
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
        .status()
        .expect("failed to generate react native bindings");
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to generate react native bindings"));
    }

    for target_arch in target_archs {
        let mut platform = "android";
        if target_arch.as_str().contains("ios") {
            platform = "ios";
        }
        let mut args = vec![
            "build".to_string(),
            platform.to_string(),
            "--and-generate".to_string(),
        ];

        if mode == Mode::Release {
            args.push("--release".to_string());
        }

        args.push("--targets".to_string());
        args.push(target_arch.as_str().to_string());

        let status = Command::new("uniffi-bindgen-react-native")
            .args(&args)
            .current_dir(&bindings_dir)
            .status()
            .expect("failed to build react native bindings");
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to build react native bindings"));
        }
    }
    Ok(())
}
