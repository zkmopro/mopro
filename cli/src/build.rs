use anyhow::Error;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use dialoguer::Select;
use include_dir::include_dir;
use include_dir::Dir;
use std::env;

use crate::config::read_config;
use crate::config::write_config;
use crate::constants::Adapter;
use crate::constants::Platform;
use crate::constants::MODES;
use crate::constants::PLATFORMS;
use crate::create::utils::copy_embedded_dir;
use crate::print::print_build_success_message;
use crate::style;
use crate::style::blue_bold;
use crate::style::print_green_bold;
use crate::utils::PlatformSelector;

pub fn build_project(arg_mode: &Option<String>, arg_platforms: &Option<Vec<String>>) -> Result<()> {
    // Detect `Cargo.toml` file before starting build process
    let current_dir = env::current_dir()?;
    let cargo_toml_path = current_dir.join("Cargo.toml");

    if !cargo_toml_path.exists() {
        style::print_yellow(
            "'Cargo.toml' not found. Please check current project directory.".to_string(),
        );
        return Ok(());
    };

    // Detect `Config.toml`
    let config_path = current_dir.join("Config.toml");

    // Check if the config file exist
    if !config_path.exists() {
        return Err(Error::msg(
            "Config.toml does exists. Please run 'mopro init'",
        ));
    }
    let mut config = read_config(&config_path)?;

    // Mode selection, select `release` or `debug`
    let mode: String = match arg_mode.as_deref() {
        None => select_mode()?,
        Some(m) => {
            if MODES.contains(&m) {
                m.to_string()
            } else {
                style::print_yellow("Invalid mode selected. Please choose a valid mode (e.g., 'release' or 'debug').".to_string());
                select_mode()?
            }
        }
    };

    // Platform selection
    let platform: PlatformSelector = match arg_platforms {
        None => PlatformSelector::select(&config),
        Some(p) => {
            let valid_platforms: Vec<String> = p
                .iter()
                .filter(|&platform| PLATFORMS.contains(&platform.as_str()))
                .map(|platform| platform.to_owned())
                .collect();

            if valid_platforms.is_empty() {
                style::print_yellow(
                    "No platforms selected. Please select at least one platform.".to_string(),
                );
                PlatformSelector::select(&config)
            } else {
                if valid_platforms.len() != p.len() {
                    style::print_yellow(
                        format!(
                            "Invalid platform(s) selected. Only {:?} platform(s) is created.",
                            &valid_platforms
                        )
                        .to_string(),
                    );
                }
                PlatformSelector::construct(valid_platforms)
            }
        }
    };

    // Supported adapters and platforms:
    // | Platforms | Circom | Halo2 |
    // |-----------|--------|-------|
    // | iOS       | Yes    | Yes   |
    // | Android   | Yes    | Yes   |
    // | Web       | No     | Yes   |
    //
    // Note: 'Yes' indicates that the adapter is compatible with the platform.

    // If 'Circom' is the only selected adapter and 'Web' is the only selected platform,
    // Restart the build step as this combination is not supported.
    if config.adpater_eq(Adapter::Circom) && platform.eq(&vec![Platform::Web]) {
        style::print_yellow(
            "Web platform is not support Circom only, choose different platform".to_string(),
        );
        build_project(&Some(mode.clone()), &None)?;
    }

    // Notification when the user selects the 'circom' adapter and includes the 'web' platform in the selection.
    if config.adpater_eq(Adapter::Circom) && platform.contains(Platform::Web) {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("WASM code for Circom will not be generated for the web platform due to lack of support. Do you want to continue?")
                .default(true)
                .interact()?;

        if !confirm {
            build_project(&Some(mode.clone()), &None)?;
        }

        copy_mopro_wasm_lib()?;
    }

    // Notification when the user selects the 'halo2' adapter and includes the 'web' platform in the selection.
    if config.adpater_contains(Adapter::Halo2) && platform.contains(Platform::Web) {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Halo2 WASM code will only be generated for the web platform. Do you want to continue?")
                .default(true)
                .interact()?;

        if !confirm {
            style::print_yellow("Aborted build for web platform".to_string());
            std::process::exit(0);
        }

        copy_mopro_wasm_lib()?;
    }

    // Archtecture selection for iOS or Andriod
    let selected_architectures = platform.select_archs();

    for p in platform.platforms.clone() {
        let platform_str: &str = p.into();
        let selected_arch = selected_architectures
            .get(platform_str)
            .map(|archs| archs.join(","))
            .unwrap_or_default();

        let status = std::process::Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(platform_str)
            .env("CONFIGURATION", mode.clone())
            .env(p.arch_key(), selected_arch)
            .status()?;

        // Add only successfully compiled platforms to the config.
        if status.success() {
            config.target_platforms.insert(platform_str.into());
        } else {
            // Return a custom error if the command fails
            return Err(anyhow::anyhow!(
                "Output with status code {}",
                status.code().unwrap()
            ));
        }
    }

    // Save the updated config to the file
    write_config(&config_path, &config)?;

    print_binding_message(&platform.platforms)?;

    Ok(())
}

fn select_mode() -> Result<String> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Build mode")
        .items(&MODES)
        .interact()?;

    Ok(MODES[idx].to_owned())
}

fn print_binding_message(platforms: &Vec<Platform>) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    print_green_bold("✨ Bindings Built Successfully! ✨".to_string());
    println!("The Mopro bindings have been successfully generated and are available in the following directories:\n");
    for platform in platforms {
        let text = format!(
            "- {}/Mopro{}Bindings",
            current_dir.display(),
            platform.binding_name()
        );
        println!("{}", blue_bold(text.to_string()));
    }
    print_build_success_message();
    Ok(())
}

fn copy_mopro_wasm_lib() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let target_dir = cwd.join("mopro-wasm-lib");

    if !target_dir.exists() {
        const WASM_TEMPLATE_DIR: Dir =
            include_dir!("$CARGO_MANIFEST_DIR/src/template/mopro-wasm-lib");
        copy_embedded_dir(&WASM_TEMPLATE_DIR, &target_dir)?;
    }

    Ok(())
}
