use std::env;
use std::path::PathBuf;

use crate::config::read_config;
use crate::constants::{Framework, Platform};
use crate::style;
use anyhow::Error;
use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

mod android;
mod ios;
pub use android::Android;
pub use ios::Ios;
mod web;
pub use web::Web;
mod flutter;
pub use flutter::Flutter;
mod react_native;
pub use react_native::ReactNative;
pub mod utils;
pub mod write_toml;

pub trait Create {
    const NAME: &'static str;
    fn create(project_dir: PathBuf) -> Result<(), Error>;
    fn print_message();
}

pub fn create_project(
    arg_framework: &Option<String>,
    arg_platform: &Option<String>,
) -> anyhow::Result<()> {
    // 1. Determine framework
    let framework: String = match arg_framework.as_deref() {
        None => select_framework()?,
        Some(m) => {
            if Framework::contains(m) {
                m.to_string()
            } else {
                style::print_yellow("Invalid template selected.".to_string());
                select_framework()?
            }
        }
    };

    // 2. Determine platform if required
    let project_dir = env::current_dir()?;

    match Framework::parse_from_str(&framework) {
        Framework::ReactNative => {
            if let Some(platform) = arg_platform {
                ReactNative::create_with_platform(project_dir, platform.to_lowercase())?;
            } else {
                let selected_platform = select_platform()?;
                ReactNative::create_with_platform(project_dir, selected_platform)?;
            }
        }
        Framework::Flutter => {
            if let Some(platform) = arg_platform {
                Flutter::create_with_platform(project_dir, platform.to_lowercase())?;
            } else {
                let selected_platform = select_platform()?;
                Flutter::create_with_platform(project_dir, selected_platform)?;
            }
        }
        Framework::Ios => Ios::create(project_dir)?,
        Framework::Android => Android::create(project_dir)?,
        Framework::Web => Web::create(project_dir)?,
    }

    Ok(())
}

fn select_framework() -> anyhow::Result<String> {
    let (items, unselectable) = get_target_platforms_with_status()?;

    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Create template")
        .items(&items)
        .interact_on_opt(&Term::stderr())?;

    if let Some(selected_idx) = idx {
        if unselectable[selected_idx] {
            style::print_yellow(format!(
                "Cannot create {} template - build binding first",
                Framework::from_idx(selected_idx).as_str()
            ));
            return select_framework();
        }
        Ok(items[selected_idx].to_owned()) // Only available items will be matched with 'platform'
    } else {
        Err(Error::msg("Template selection failed"))
    }
}

fn get_target_platforms_with_status() -> anyhow::Result<(Vec<String>, Vec<bool>)> {
    let current_dir = env::current_dir()?;
    let config = read_config(&current_dir.join("Config.toml"))?;

    let mut items = Vec::new();
    let mut unselectable = Vec::new();

    for framework_str in Framework::all_strings() {
        let framework = Framework::parse_from_str(framework_str);
        match framework {
            Framework::Flutter | Framework::ReactNative => {
                // Adding more information to the list
                let requires = [Platform::Ios, Platform::Android];
                let missing: Vec<&str> = requires
                    .iter()
                    .filter(|&req| {
                        if let Some(platforms) = &config.target_platforms {
                            !platforms.contains(req.as_str())
                        } else {
                            false
                        }
                    })
                    .map(|r| r.as_str())
                    .collect();

                if !missing.is_empty() {
                    items.push(format!(
                        "{:<12} - Requires {} binding(s)",
                        framework_str.to_string(),
                        missing.join("/")
                    ));
                    unselectable.push(true);
                } else {
                    items.push(framework_str.to_string());
                    unselectable.push(false);
                }
            }
            _ => {
                if let Some(platforms) = &config.target_platforms {
                    if platforms.contains(framework_str) {
                        items.push(framework_str.to_string());
                        unselectable.push(false);
                    }
                } else {
                    items.push(format!(
                        "{:<12} - Require binding",
                        framework_str.to_string()
                    ));
                    unselectable.push(true);
                }
            }
        }
    }

    Ok((items, unselectable))
}

fn select_platform() -> anyhow::Result<String> {
    let items = vec!["ios", "android"];
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose target platform")
        .items(&items)
        .interact_on_opt(&Term::stderr())?;

    if let Some(selected_idx) = idx {
        Ok(items[selected_idx].to_string())
    } else {
        Err(Error::msg("Platform selection failed"))
    }
}
