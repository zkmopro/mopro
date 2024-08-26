use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use std::{env, error::Error};

const MODES: [&str; 2] = ["debug", "release"];
const PLATFORMS: [&str; 2] = ["ios", "android"];

pub fn build_project(
    arg_mode: &Option<String>,
    arg_platforms: &Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let mode: String = match arg_mode.as_deref() {
        None => select_mode()?,
        Some(m) => {
            if MODES.contains(&m) {
                m.to_string()
            } else {
                println!("\x1b[33mInvalid mode selected. Please choose a valid mode (e.g., 'release' or 'debug').\x1b[0m");
                select_mode()?
            }
        }
    };

    let platforms: Vec<String> = match arg_platforms {
        None => select_platforms()?,
        Some(p) => {
            let mut valid_platforms: Vec<String> = p
                .iter()
                .filter(|&platform| PLATFORMS.contains(&platform.as_str()))
                .map(|platform| platform.to_owned())
                .collect();

            if valid_platforms.is_empty() {
                println!(
                    "\x1b[33mNo platforms selected. Please select at least one platform.\x1b[0m"
                );
                valid_platforms = select_platforms()?;
            } else if valid_platforms.len() != p.len() {
                println!(
                    "\x1b[33mInvalid platform(s) selected. Only {:?} platform(s) is createds\x1b[0m",
                    valid_platforms
                );
            }

            valid_platforms
        }
    };

    if platforms.is_empty() {
        println!("\x1b[33mNo platform selected. Use space to select platform(s).\x1b[0m");
        build_project(&Some(mode), &None)?;
    } else {
        for platform in platforms.clone() {
            let status = std::process::Command::new("cargo")
                .arg("run")
                .arg("--bin")
                .arg(platform.clone())
                .env("CONFIGURATION", mode.clone())
                .status()?;

            if !status.success() {
                // Return a custom error if the command fails
                return Err(format!("Output with status code {}", status.code().unwrap()).into());
            }
        }

        print_binding_message(platforms)?;
    }

    Ok(())
}

fn select_mode() -> Result<String, Box<dyn Error>> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Build mode")
        .items(&MODES)
        .interact()?;

    Ok(MODES[idx].to_owned())
}

fn select_platforms() -> Result<Vec<String>, Box<dyn Error>> {
    let selected_platforms = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select platform(s) to build for (multiple selection with space)")
        .items(&PLATFORMS)
        .interact()?;

    Ok(selected_platforms
        .iter()
        .map(|&idx| PLATFORMS[idx].to_owned())
        .collect())
}

fn print_binding_message(platforms: Vec<String>) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    println!("\x1b[1;32m✨ Bindings Built Successfully! ✨\x1b[0m\n");
    println!("The Mopro bindings have been successfully generated and are available in the following directories:\n");
    for platform in platforms {
        println!(
            "\x1b[1;34m- {}/Mopro{}Bindings\x1b[0m",
            current_dir.display(),
            platform
                .to_lowercase()
                .replace("ios", "iOS")
                .replace("android", "Android")
        );
    }
    println!();
    println!("📚 To learn more about mopro, visit: \x1b[1;34mhttps://zkmopro.org\x1b[0m");
    Ok(())
}
