use std::collections::HashMap;
use std::env;

use anyhow::Error;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use dialoguer::MultiSelect;
use dialoguer::Select;
use include_dir::include_dir;
use include_dir::Dir;

use crate::config::read_config;
use crate::config::write_config;
use crate::create::utils::copy_embedded_dir;
use crate::print::print_build_success_message;
use crate::style;
use crate::style::blue_bold;
use crate::style::create_custom_theme;
use crate::style::print_green_bold;

const MODES: [&str; 2] = ["debug", "release"];
const PLATFORMS: [&str; 3] = ["ios", "android", "web"];
// Note that *_ARCH should align with `ios.rs` and `andriod.rs` in "mopro-ffi/src/app_config"
const IOS_ARCHS: [&str; 3] = [
    "aarch64-apple-ios",
    "aarch64-apple-ios-sim",
    "x86_64-apple-ios",
];
const ANDROID_ARCHS: [&str; 4] = [
    "x86_64-linux-android",
    "i686-linux-android",
    "armv7-linux-androideabi",
    "aarch64-linux-android",
];

pub fn build_project(
    arg_mode: &Option<String>,
    arg_platforms: &Option<Vec<String>>,
) -> anyhow::Result<()> {
    // Detect `Cargo.toml` file before starting build process
    let current_dir = env::current_dir()?;
    let cargo_toml_path = current_dir.join("Cargo.toml");

    if !cargo_toml_path.exists() {
        style::print_yellow(
            "'Cargo.toml' not found. Please check current project directory.".to_string(),
        );
        return Ok(());
    };

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

    let selected_platforms: Vec<String> = match arg_platforms {
        None => select_platforms()?,
        Some(p) => {
            let mut valid_platforms: Vec<String> = p
                .iter()
                .filter(|&platform| PLATFORMS.contains(&platform.as_str()))
                .map(|platform| platform.to_owned())
                .collect();

            if valid_platforms.is_empty() {
                style::print_yellow(
                    "No platforms selected. Please select at least one platform.".to_string(),
                );
                valid_platforms = select_platforms()?;
            } else if valid_platforms.len() != p.len() {
                style::print_yellow(
                    format!(
                        "Invalid platform(s) selected. Only {:?} platform(s) is created.",
                        &valid_platforms
                    )
                    .to_string(),
                );
            }

            valid_platforms
        }
    };

    if selected_platforms.is_empty() {
        style::print_yellow("No platform selected. Use space to select platform(s).".to_string());
        build_project(&Some(mode), &None)?;
    } else {
        let config_path = current_dir.join("Config.toml");

        // Check if the config file exist
        if !config_path.exists() {
            return Err(Error::msg(
                "Config.toml does exists. Please run 'mopro init'",
            ));
        }
        let mut config = read_config(&config_path)?;

        // Supported adapters and platforms:
        // | Platforms | Circom | Halo2 |
        // |-----------|--------|-------|
        // | iOS       | Yes    | Yes   |
        // | Android   | Yes    | Yes   |
        // | Web       | No     | Yes   |
        //
        // Note: 'Yes' indicates that the adapter is compatible with the platform.

        // Initialize target platform for preventing add more platforms when the user build again
        config.target_platforms = vec![];

        let selected_adapters = config.target_adapters.clone();

        // If 'Circom' is the only selected adapter and 'Web' is the only selected platform,
        // Restart the build step as this combination is not supported.
        if selected_adapters.as_slice() == ["circom"] && selected_platforms == vec!["web"] {
            style::print_yellow(
                "Web platform is not support Circom only, choose different platform".to_string(),
            );
            build_project(&Some(mode.clone()), &None)?;
        }

        // Notification when the user selects the 'circom' adapter and includes the 'web' platform in the selection.
        if selected_adapters.as_slice() == ["circom"]
            && selected_platforms.contains(&"web".to_string())
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
        if selected_adapters.contains(&"halo2".to_string())
            && selected_platforms.contains(&"web".to_string())
        {
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

            if !status.success() {
                // Return a custom error if the command fails
                return Err(anyhow::anyhow!(
                    "Output with status code {}",
                    status.code().unwrap()
                ));
            }
        }

        // Write selected platforms for create command
        for p in &selected_platforms {
            config.target_platforms.push(p.to_string())
        }
        write_config(&config_path, &config)?;

        print_binding_message(selected_platforms)?;
    }

    Ok(())
}

fn select_mode() -> anyhow::Result<String> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Build mode")
        .items(&MODES)
        .interact()?;

    Ok(MODES[idx].to_owned())
}

fn select_platforms() -> anyhow::Result<Vec<String>> {
    let theme = create_custom_theme();
    let selected_platforms = MultiSelect::with_theme(&theme)
        .with_prompt("Select platform(s) to build for (multiple selection with space)")
        .items(&PLATFORMS)
        .interact()?;

    Ok(selected_platforms
        .iter()
        .map(|&idx| PLATFORMS[idx].to_owned())
        .collect())
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

fn print_binding_message(platforms: Vec<String>) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    print_green_bold("✨ Bindings Built Successfully! ✨".to_string());
    println!("The Mopro bindings have been successfully generated and are available in the following directories:\n");
    for platform in platforms {
        let text = format!(
            "- {}/Mopro{}Bindings",
            current_dir.display(),
            platform
                .to_lowercase()
                .replace("ios", "iOS")
                .replace("android", "Android")
                .replace("web", "WASM")
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
