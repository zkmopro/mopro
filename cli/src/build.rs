use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

use anyhow::Error;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use dialoguer::Select;
use include_dir::include_dir;
use include_dir::Dir;

use crate::config::read_config;
use crate::config::write_config;
use crate::config::Config;
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

    let platform: PlatformSelector = match arg_platforms {
        None => PlatformSelector::select(),
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
                PlatformSelector::select()
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

    // Initialize target platform for preventing add more platforms when the user build again
    let selected_adapters = config.target_adapters.clone();

    // If 'Circom' is the only selected adapter and 'Web' is the only selected platform,
    // Restart the build step as this combination is not supported.
    if selected_adapters == HashSet::from(["circom".to_string()])
        && selected_platforms == HashSet::from(["web".to_string()])
    {
        style::print_yellow(
            "Web platform is not support Circom only, choose different platform".to_string(),
        );
        build_project(&Some(mode.clone()), &None)?;
    }

    // Notification when the user selects the 'circom' adapter and includes the 'web' platform in the selection.
    if selected_adapters == HashSet::from(["circom".to_string()])
        && selected_platforms.contains("web")
    {
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
    if selected_adapters.contains("halo2") && selected_platforms.contains("web") {
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
    let mut selected_architectures: HashMap<String, Vec<String>> = HashMap::new();

    for platform in &selected_platforms {
        let archs = match platform.as_str() {
            "ios" => select_architectures("iOS", &IOS_ARCHS)?,
            "android" => select_architectures("Android", &ANDROID_ARCHS)?,
            _ => vec![],
        };

        selected_architectures.insert(platform.clone(), archs);
    }

    for platform in selected_platforms.clone() {
        let arch_key: &str = match platform.as_str() {
            "ios" => "IOS_ARCHS",
            "android" => "ANDROID_ARCHS",
            "web" => "",
            _ => unreachable!(),
        };

        let selected_arch = selected_architectures
            .get(&platform)
            .map(|archs| archs.join(","))
            .unwrap_or_default();

        let status = std::process::Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(platform.clone())
            .env("CONFIGURATION", mode.clone())
            .env(arch_key, selected_arch)
            .status()?;

        // Add only successfully compiled platforms to the config.
        if status.success() {
            config.target_platforms.insert(platform.to_string());
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

    print_binding_message(selected_platforms)?;

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

fn select_mode() -> Result<String> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Build mode")
        .items(&MODES)
        .interact()?;

    Ok(MODES[idx].to_owned())
}

fn select_platforms(config: &Config) -> anyhow::Result<HashSet<String>> {
    let theme = create_custom_theme();

    // defaults based on previous selections.
    let defaults: Vec<bool> = PLATFORMS
        .iter()
        .map(|&platform| config.target_platforms.contains(platform))
        .collect();

    let selected_platforms = MultiSelect::with_theme(&theme)
        .with_prompt("Select platform(s) to build for (multiple selection with space)")
        .items(&PLATFORMS)
        .defaults(&defaults)
        .interact()?;

    Ok(selected_platforms
        .iter()
        .map(|&idx| PLATFORMS[idx].to_owned())
        .collect::<HashSet<_>>())
}

fn select_architectures(platform: &str, archs: &[&str]) -> anyhow::Result<Vec<String>> {
    // At least one architecture must be selected
    loop {
        let theme = create_custom_theme();
        let selected_archs = MultiSelect::with_theme(&theme)
            .with_prompt(format!(
                "Select {} architecture(s) to compile (default: all)",
                platform
            ))
            .items(archs)
            .defaults(&vec![true; archs.len()])
            .interact()?;

        if selected_archs.is_empty() {
            style::print_yellow(format!(
                "No architectures selected for {}. Please select at least one architecture.",
                platform
            ));
        } else {
            return Ok(selected_archs
                .iter()
                .map(|&idx| archs[idx].to_owned())
                .collect());
        }
    }
}

fn print_binding_message(platforms: HashSet<String>) -> anyhow::Result<()> {
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
