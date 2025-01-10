use std::env;
use std::fmt::Display;
use std::path::PathBuf;

use anyhow::Error;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

mod android;
mod ios;
use android::Android;
use ios::Ios;
mod web;
use web::Web;
mod flutter;
use flutter::Flutter;
mod react_native;
use crate::config::read_config;
use crate::style;
use react_native::ReactNative;
pub mod utils;

trait Create {
    const NAME: &'static str;
    fn create(project_dir: PathBuf) -> Result<(), Error>;
    fn print_message();
}

#[derive(Clone, PartialEq, Eq)]
pub enum Framework {
    Ios,
    Android,
    Web,
    Flutter,
    ReactNative,
}

impl Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            Framework::Ios => "ios",
            Framework::Android => "android",
            Framework::Web => "web",
            Framework::Flutter => "flutter",
            Framework::ReactNative => "react-native",
        };
        write!(f, "{}", as_str)
    }
}

impl From<String> for Framework {
    fn from(app: String) -> Self {
        match app.to_lowercase().as_str() {
            "ios" => Framework::Ios,
            "android" => Framework::Android,
            "web" => Framework::Web,
            "flutter" => Framework::Flutter,
            "react-native" => Framework::ReactNative,
            _ => panic!("Unknown platform selected."),
        }
    }
}

impl From<Framework> for &str {
    fn from(app: Framework) -> Self {
        match app {
            Framework::Ios => "ios",
            Framework::Android => "android",
            Framework::Web => "web",
            Framework::Flutter => "flutter",
            Framework::ReactNative => "react-native",
        }
    }
}

const FRAMEWORKS: [Framework; 5] = [
    Framework::Ios,
    Framework::Android,
    Framework::Web,
    Framework::Flutter,
    Framework::ReactNative,
];

pub fn create_project(arg_framework: &Option<String>) -> anyhow::Result<()> {
    let framework: String = match arg_framework.as_deref() {
        None => select_framework()?,
        Some(m) => {
            if FRAMEWORKS.contains(&Framework::from(m.to_string())) {
                m.to_string()
            } else {
                style::print_yellow("Invalid template selected. Please choose a valid template (e.g., 'ios', 'android', 'web', 'react-native', 'flutter').".to_string());
                select_framework()?
            }
        }
    };

    let project_dir = env::current_dir()?;
    match framework.into() {
        Framework::Ios => Ios::create(project_dir)?,
        Framework::Android => Android::create(project_dir)?,
        Framework::Web => Web::create(project_dir)?,
        Framework::Flutter => Flutter::create(project_dir)?,
        Framework::ReactNative => ReactNative::create(project_dir)?,
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
                &FRAMEWORKS[selected_idx]
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

    for framework in FRAMEWORKS.iter() {
        match framework {
            Framework::Flutter | Framework::ReactNative => {
                // Adding more information to the list
                let requires = ["ios", "android"];
                let missing: Vec<&str> = requires
                    .iter()
                    .filter(|&&req| !config.target_platforms.contains(req))
                    .cloned()
                    .collect();

                if !missing.is_empty() {
                    items.push(format!(
                        "{:<12} - Requires {} binding(s)",
                        framework,
                        missing.join("/")
                    ));
                    unselectable.push(true);
                } else {
                    items.push(framework.to_string());
                    unselectable.push(false);
                }
            }
            _ => {
                if config.target_platforms.contains(&framework.to_string()) {
                    items.push(framework.to_string());
                    unselectable.push(false);
                } else {
                    items.push(format!("{:<12} - Require binding", framework));
                    unselectable.push(true);
                }
            }
        }
    }

    Ok((items, unselectable))
}
