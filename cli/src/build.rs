use anyhow::Ok;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use dialoguer::Select;
use include_dir::include_dir;
use include_dir::Dir;
use std::collections::HashSet;
use std::env;

use crate::config::read_config;
use crate::config::write_config;
use crate::config::Config;
use crate::constants::AndroidArch;
use crate::constants::Mode;
use crate::constants::Platform;
use crate::create::utils::copy_embedded_dir;
use crate::init::adapter::Adapter;
use crate::print::print_build_success_message;
use crate::style;
use crate::style::blue_bold;
use crate::style::print_green_bold;
use crate::utils::PlatformSelector;

pub fn build_project(
    arg_mode: &Option<String>,
    arg_platforms: &Option<Vec<String>>,
    arg_architectures: &Option<Vec<String>>,
) -> Result<()> {
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

    // Check if the config file exists, if not create a default one
    if !config_path.exists() {
        let default_config = Config {
            build_mode: Some("".to_string()),
            target_adapters: Some(HashSet::new()),
            target_platforms: Some(HashSet::new()),
            ios: Some(HashSet::new()),
            android: Some(HashSet::new()),
        };
        write_config(&config_path, &default_config)?;
    }
    let mut config = read_config(&config_path)?;

    // Mode selection, select `release` or `debug`
    let mode: Mode = match arg_mode.as_deref() {
        None => select_mode(&mut config)?,
        Some(m) => {
            if Mode::all_strings().contains(&m) {
                Mode::parse_from_str(m)
            } else {
                style::print_yellow("Invalid mode selected. Please choose a valid mode (e.g., 'release' or 'debug').".to_string());
                select_mode(&mut config)?
            }
        }
    };
    write_config(&config_path, &config)?;

    // Platform selection
    let mut platform: PlatformSelector = match arg_platforms {
        None => PlatformSelector::select(&mut config),
        Some(p) => {
            let valid_platforms: Vec<String> = p
                .iter()
                .filter(|&platform| Platform::all_strings().contains(&platform.as_str()))
                .map(|platform| platform.to_owned())
                .collect();

            if valid_platforms.is_empty() {
                style::print_yellow(
                    "No platforms selected. Please select at least one platform.".to_string(),
                );
                PlatformSelector::select(&mut config)
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
                PlatformSelector::construct_with_config(valid_platforms, &mut config)
            }
        }
    };
    write_config(&config_path, &config)?;

    // Supported adapters and platforms:
    // | Platforms | Circom | Halo2 | Noir |
    // |-----------|--------|-------|------|
    // | iOS       | Yes    | Yes   | Yes  |
    // | Android   | Yes    | Yes   | Yes  |
    // | Web       | No     | Yes   | No   |
    //
    // Note: 'Yes' indicates that the adapter is compatible with the platform.

    // Noir only supports `iOS` and `Android` platform.
    if config.adapter_contains(Adapter::Noir) && platform.contains(Platform::Web) {
        style::print_yellow(
            "Noir doesn't support Web platform, choose different platform".to_string(),
        );
        build_project(
            &Some(mode.as_str().to_string()),
            arg_platforms,
            arg_architectures,
        )?;
        return Ok(());
    }

    // Notification when the user selects the 'circom' adapter and includes the 'web' platform in the selection.
    if config.adapter_eq(Adapter::Circom) && platform.contains(Platform::Web) {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("WASM code for Circom will not be generated for the web platform due to lack of support. Do you want to continue?")
                .default(true)
                .interact()?;

        if !confirm {
            build_project(
                &Some(mode.as_str().to_string()),
                arg_platforms,
                arg_architectures,
            )?;
            return Ok(());
        }

        copy_mopro_wasm_lib()?;
    }

    // Notification when the user selects the 'halo2' adapter and includes the 'web' platform in the selection.
    if config.adapter_contains(Adapter::Halo2) && platform.contains(Platform::Web) {
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

    // Architecture selection for iOS or Android
    let selected_architectures = if let Some(archs) = arg_architectures {
        platform.construct_archs(archs, &mut config)
    } else {
        platform.select_archs(&mut config)
    };
    write_config(&config_path, &config)?;

    // Noir doesn't support `I686Linux` and `Armv7LinuxAbi`
    if config.adapter_contains(Adapter::Noir) {
        let not_allowed_archs = vec![
            AndroidArch::I686Linux.as_str(),
            AndroidArch::Armv7LinuxAbi.as_str(),
        ];

        if platform.contains_archs(not_allowed_archs.as_slice()) {
            style::print_yellow(
                format!(
                    "Noir doesn't support following architectures: {:?}, choose other architectures",
                    not_allowed_archs
                )
                .to_string(),
            );
            build_project(
                &Some(mode.as_str().to_string()),
                arg_platforms,
                arg_architectures,
            )?;
            return Ok(());
        }
    }

    for p in platform.platforms.clone() {
        let platform_str: &str = p.as_str();
        let selected_arch = selected_architectures
            .get(platform_str)
            .map(|archs| archs.join(","))
            .unwrap_or_default();

        let mut command = std::process::Command::new("cargo");
        command
            .arg("run")
            .arg("--bin")
            .arg(platform_str)
            .env("CONFIGURATION", mode.as_str())
            .env(p.arch_key(), selected_arch);

        // The dependencies of Noir libraries need iOS 15 and above.
        let status = if config.adapter_contains(Adapter::Noir) && p.eq(&Platform::Ios) {
            command.env("IPHONEOS_DEPLOYMENT_TARGET", "15").status()?
        } else {
            command.status()?
        };

        if !status.success() {
            // Return a custom error if the command fails
            return Err(anyhow::anyhow!(
                "Output with status code {}",
                status.code().unwrap()
            ));
        }
    }

    print_binding_message(&platform.platforms)?;

    Ok(())
}

fn select_mode(config: &mut Config) -> Result<Mode> {
    let theme = ColorfulTheme::default();
    let mut selection = Select::with_theme(&theme);

    // Get default based on previous selection.
    if let Some(build_mode) = config.build_mode.clone() {
        if let Some(idx) = Mode::idx(build_mode.as_ref()) {
            selection.default(idx);
        }
    };

    let idx = selection
        .with_prompt("Build mode")
        .items(Mode::all_strings().as_ref())
        .interact()?;

    // update user's selection
    config.build_mode = Some(Mode::from_idx(idx).as_str().to_string());

    Ok(Mode::from_idx(idx))
}

fn print_binding_message(platforms: &Vec<Platform>) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    print_green_bold("✨ Bindings Built Successfully! ✨".to_string());
    println!("The Mopro bindings have been successfully generated and are available in the following directories:\n");
    for platform in platforms {
        let text = format!("- {}/{}", current_dir.display(), platform.binding_dir());
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
