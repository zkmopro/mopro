use std::env;
use std::fmt::Display;
use std::path::PathBuf;

use crate::style;
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

pub enum Framework {
    Ios,
    Android,
    Web,
    Flutter,
    ReactNative,
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

const TEMPLATES: [&str; 5] = ["ios", "android", "web", "flutter", "react-native"];

pub fn create_project(arg_framework: &Option<String>) -> anyhow::Result<()> {
    let framework: String = match arg_framework.as_deref() {
        None => select_framework()?,
        Some(m) => {
            if TEMPLATES.contains(&m) {
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
        .items(&TEMPLATES)
        .interact()?;

    Ok(TEMPLATES[idx].to_owned())
}
