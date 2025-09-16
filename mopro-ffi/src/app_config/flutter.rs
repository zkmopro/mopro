use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::Value;

use super::constants::{FlutterArch, FlutterPlatform, Mode, FLUTTER_BINDINGS_DIR};

use super::raw_project_name_from_toml;
use super::PlatformBuilder;

// Maintained for backwards compatibility
#[inline]
pub fn build() {
    if cfg!(feature = "uniffi") {
        panic!("\"uniffi\" and \"flutter\" features cannot be enabled at the same time, please disable one of them in your Cargo.toml");
    }
    super::build_from_env::<FlutterPlatform>()
}

#[derive(Default)]
pub struct FlutterBindingsParams {
    pub using_noir: bool,
}

impl PlatformBuilder for FlutterPlatform {
    type Arch = FlutterArch;
    type Params = FlutterBindingsParams;

    fn build(
        _mode: Mode,
        project_dir: &Path,
        _target_archs: Vec<Self::Arch>,
        _params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        // Init flutter bindings template
        init_flutter_bindings(project_dir)?;

        // Init workspace for bindings template
        let cargo_toml_path = project_dir
            .join(FLUTTER_BINDINGS_DIR)
            .join("rust/Cargo.toml");
        ensure_workspace_toml(&cargo_toml_path.to_string_lossy().to_string());

        // Import user defined crates
        let third_party_crate_name = raw_project_name_from_toml(project_dir)?;
        let cargo_add_status = Command::new("cargo")
            .args([
                "add",
                &third_party_crate_name,
                "--path",
                &project_dir.to_string_lossy().to_string(),
            ])
            .current_dir(project_dir.join(FLUTTER_BINDINGS_DIR).join("rust"))
            .status()
            .expect("failed to run cargo add");
        if !cargo_add_status.success() {
            return Err(anyhow::anyhow!("Failed to add third party crate"));
        }

        // Replace relative path with absolute path
        replace_relative_path_with_absolute(
            &cargo_toml_path,
            &third_party_crate_name,
            &project_dir,
        )?;

        // Patch cargokit build script
        // See: https://github.com/fzyzcjy/flutter_rust_bridge/issues/2839
        // TODO: remove this once the issue is fixed
        patch_cargokit_build_script(project_dir)?;

        // Generate flutter bindings
        let rust_root = project_dir.join(FLUTTER_BINDINGS_DIR).join("rust");
        let dart_output = project_dir.join(FLUTTER_BINDINGS_DIR).join("lib/src/rust");
        let generate_status = Command::new("flutter_rust_bridge_codegen")
            .args(["generate"])
            .args([
                "--rust-root",
                &rust_root.to_string_lossy(),
                "--rust-input",
                &third_party_crate_name,
                "--dart-output",
                &dart_output.to_string_lossy(),
            ])
            .current_dir(project_dir)
            .status()
            .expect("failed to run flutter_rust_bridge_codegen");
        if !generate_status.success() {
            return Err(anyhow::anyhow!("Failed to generate simple.rs"));
        }

        Ok(PathBuf::from(FLUTTER_BINDINGS_DIR))
    }
}

fn install_flutter_rust_bridge_codegen() -> anyhow::Result<()> {
    let output = Command::new("flutter_rust_bridge_codegen").output();
    match output {
        Ok(_) => {
            // Command exists, no need to install
            println!("flutter_rust_bridge_codegen already installed.");
            return Ok(());
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Command not found, proceed with installation
            println!("flutter_rust_bridge_codegen not found, installing...");
            let status = Command::new("cargo")
                .args(["install", "flutter_rust_bridge_codegen@=2.11.1"])
                .status()
                .expect("failed to run flutter_rust_bridge_codegen");
            if !status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to install flutter_rust_bridge_codegen"
                ));
            }
        }
        Err(e) => {
            // Other error, propagate it
            return Err(anyhow::anyhow!(
                "Failed to check for flutter_rust_bridge_codegen: {}",
                e
            ));
        }
    }

    Ok(())
}

fn init_flutter_bindings(project_dir: &Path) -> anyhow::Result<()> {
    let flutter_bindings_dir = project_dir.join(FLUTTER_BINDINGS_DIR);

    install_flutter_rust_bridge_codegen()?;

    if !flutter_bindings_dir.exists() {
        let status = Command::new("flutter_rust_bridge_codegen")
            .args(["create", FLUTTER_BINDINGS_DIR, "--template", "plugin"])
            .status()
            .expect("failed to run flutter_rust_bridge_codegen");

        if !status.success() {
            return Err(anyhow::anyhow!("flutter_rust_bridge_codegen failed"));
        }
    }

    Ok(())
}

fn ensure_workspace_toml(cargo_toml_path: &str) {
    let content = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

    if !content.contains("[workspace]") {
        let new_content = format!("{content}\n\n[workspace]\n");
        fs::write(cargo_toml_path, new_content).expect("Failed to write updated Cargo.toml");
    }
}

fn replace_relative_path_with_absolute(
    cargo_toml_path: &Path,
    crate_name: &str,
    abs_path: &Path,
) -> anyhow::Result<()> {
    let cargo_toml_content =
        fs::read_to_string(cargo_toml_path).context("Failed to read Cargo.toml")?;
    let mut cargo_toml: Value = cargo_toml_content
        .parse::<Value>()
        .context("Failed to parse Cargo.toml")?;

    // If the `name` under [lib] section is set, using the `name` as library name.
    // Otherwise, using the package name.
    let crate_path = cargo_toml
        .get_mut("dependencies")
        .and_then(|pkg| pkg.get_mut(crate_name));
    // .and_then(|pkg| pkg.as_str().map(|s| s.to_string()));

    if let Some(Value::Table(table)) = crate_path {
        table.insert(
            "path".to_string(),
            Value::String(abs_path.to_string_lossy().to_string()),
        );
    }

    let updated_cargo_toml_content =
        toml::to_string_pretty(&cargo_toml).context("Failed to serialize updated Cargo.toml")?;

    fs::write(cargo_toml_path, updated_cargo_toml_content)
        .context("Failed to write updated Cargo.toml")?;

    Ok(())
}

fn patch_cargokit_build_script(project_dir: &Path) -> anyhow::Result<()> {
    let cargo_kit_build_script_path = project_dir
        .join(FLUTTER_BINDINGS_DIR)
        .join("cargokit")
        .join("gradle")
        .join("plugin.gradle");
    let cargo_kit_build_script_content = fs::read_to_string(cargo_kit_build_script_path.clone())
        .context("Failed to read plugin.gradle")?;
    let updated_content = cargo_kit_build_script_content.replace(
        "if (plugin.class.name == \"com.flutter.gradle.FlutterPlugin\")",
        "if (plugin.class.name == \"com.flutter.gradle.FlutterPlugin\" || plugin.class.name == \"FlutterPlugin\")"
    );
    let updated_content = updated_content.replace(
        "        def platforms = com.flutter.gradle.FlutterPluginUtils.getTargetPlatforms(project).collect()",
        "        def List<String> platforms\n            try {\n                platforms = com.flutter.gradle.FlutterPluginUtils.getTargetPlatforms(project).collect()\n            } catch (Exception ignored) {\n                platforms = plugin.getTargetPlatforms().collect()\n            }"
    );

    fs::write(&cargo_kit_build_script_path, updated_content)
        .context("Failed to write updated plugin.gradle")?;

    Ok(())
}
