use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use include_dir::include_dir;
use include_dir::Dir;
use mopro_ffi::app_config::constants::{AndroidArch, AndroidPlatform, Arch, IosPlatform, Mode};
use std::env;

use mopro_ffi::app_config::build_from_str_arch;
use mopro_ffi::app_config::ios::IosBindingsParams;

use crate::build::mode_selector::select_mode;
use crate::config::read_config;
use crate::config::write_config;
use crate::config::Config;
use crate::constants::Platform;
use crate::create::utils::copy_embedded_dir;
use crate::init::adapter::Adapter;
use crate::print::print_build_success_message;
use crate::style;
use crate::style::blue_bold;
use crate::style::print_green_bold;
use crate::update::update_bindings;
use target_selector::TargetSelection;

mod mode_selector;
mod target_selector;

pub fn build_project(
    arg_mode: &Option<String>,
    arg_platforms: &Option<Vec<String>>,
    arg_architectures: &Option<Vec<String>>,
    auto_update_flag: Option<bool>,
    quiet: bool,
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
        let default_config = Config::default();
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
    let target_selection =
        TargetSelection::select_target(arg_platforms, arg_architectures, &mut config);
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
    if config.adapter_contains(Adapter::Noir) && target_selection.contains_platform(Platform::Web) {
        style::print_yellow(
            "Noir doesn't support Web platform, choose different platform".to_string(),
        );
        return build_project(
            &Some(mode.as_str().to_string()),
            &None,
            &None,
            auto_update_flag,
            quiet,
        );
    }

    // Notification when the user selects the 'circom' adapter and includes the 'web' platform in the selection.
    if config.adapter_eq(Adapter::Circom) && target_selection.contains_platform(Platform::Web) {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("WASM code for Circom will not be generated for the web platform due to lack of support. Do you want to continue?")
                .default(true)
                .interact()?;

        if !confirm {
            return build_project(
                &Some(mode.as_str().to_string()),
                arg_platforms,
                arg_architectures,
                auto_update_flag,
                quiet,
            );
        }

        copy_mopro_wasm_lib()?;
    }

    // Notification when the user selects the 'halo2' adapter and includes the 'web' platform in the selection.
    if config.adapter_contains(Adapter::Halo2) && target_selection.contains_platform(Platform::Web)
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

    // Noir doesn't support `I686Linux` and `Armv7LinuxAbi`
    if config.adapter_contains(Adapter::Noir) {
        let not_allowed_archs = vec![
            AndroidArch::I686Linux.as_str(),
            AndroidArch::Armv7LinuxAbi.as_str(),
        ];

        for not_allowed_arch in not_allowed_archs {
            if target_selection.contains_architecture(not_allowed_arch) {
                style::print_yellow(
                    format!(
                        "Noir doesn't support the following architecture: {not_allowed_arch}, choose other architectures",
                    )
                    .to_string(),
                );

                return build_project(
                    &Some(mode.as_str().to_string()),
                    arg_platforms,
                    &None,
                    auto_update_flag,
                    quiet,
                );
            }
        }
    }

    let platforms: Vec<Platform> = target_selection.platforms().collect();

    for selection in target_selection.iter() {
        match selection.platform() {
            Platform::Ios => {
                let arch_strings = selection.architecture_strings();
                let arch_refs: Vec<&String> = arch_strings.iter().collect();
                build_from_str_arch::<IosPlatform>(
                    mode,
                    &current_dir,
                    arch_refs,
                    IosBindingsParams {
                        using_noir: config.adapter_contains(Adapter::Noir),
                    },
                )?;
            }
            Platform::Android => {
                let arch_strings = selection.architecture_strings();
                let arch_refs: Vec<&String> = arch_strings.iter().collect();
                build_from_str_arch::<AndroidPlatform>(mode, &current_dir, arch_refs, ())?;
            }
            Platform::Web => {
                let platform_str = selection.platform().as_str();
                let mut command = std::process::Command::new("cargo");
                command.arg("run").arg("--bin").arg(platform_str);

                let status = command.status()?;

                if !status.success() {
                    return Err(anyhow::anyhow!(
                        "Output with status code {}",
                        status.code().unwrap()
                    ));
                }
            }
        }
    }

    if !quiet {
        print_binding_message(&platforms)?;
    }
    handle_auto_update(&config_path, &mut config, auto_update_flag)?;
    print_build_success_message();

    Ok(())
}

fn print_binding_message(platforms: &[Platform]) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    print_green_bold("✨ Bindings Built Successfully! ✨".to_string());
    println!("The Mopro bindings have been successfully generated and are available in the following directories:\n");
    for platform in platforms {
        let text = format!("- {}/{}", current_dir.display(), platform.binding_dir());
        println!("{}", blue_bold(text.to_string()));
    }
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

fn handle_auto_update(
    config_path: &std::path::Path,
    config: &mut Config,
    auto_update_flag: Option<bool>,
) -> Result<()> {
    if let Some(auto_update_flag) = auto_update_flag {
        if auto_update_flag {
            update_bindings(&None, &None, false)?;
        }
        return Ok(());
    }

    if let Some(auto) = config.auto_update {
        if auto {
            update_bindings(&None, &None, false)?;
        }
        return Ok(());
    }

    println!();
    let run_now = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Run `mopro update` now to copy them into your platform projects?")
        .default(true)
        .interact()?;

    if run_now {
        update_bindings(&None, &None, false)?;
    }

    let remember = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Remember this choice for future builds?")
        .default(false)
        .interact()?;

    if remember {
        config.auto_update = Some(run_now);
        write_config(&config_path.to_path_buf(), config)?;
    }

    Ok(())
}
