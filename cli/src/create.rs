use std::{env, error::Error, fs, path::Path};

use dialoguer::{theme::ColorfulTheme, Select};
use include_dir::{include_dir, Dir};

use crate::style::{self, print_bold, print_green_bold};

const TEMPLATES: [&str; 4] = ["ios", "android", "react-native", "flutter"];

pub fn create_project(arg_platform: &Option<String>) -> Result<(), Box<dyn Error>> {
    let platform: String = match arg_platform.as_deref() {
        None => select_template()?,
        Some(m) => {
            if TEMPLATES.contains(&m) {
                m.to_string()
            } else {
                style::print_yellow("Invalid template selected. Please choose a valid template (e.g., 'ios', 'android', 'react-native', 'flutter').".to_string());
                select_template()?
            }
        }
    };

    if platform.is_empty() {
        style::print_yellow("No adapters selected. Use space to select an adapter".to_string());
        create_project(arg_platform)?
    } else {
        let project_dir = env::current_dir()?;

        if platform.contains(TEMPLATES[0]) {
            let platform_name = "ios";
            let target_dir = project_dir.join(&platform_name);
            fs::create_dir(&target_dir)?;

            // Change directory to the project directory
            env::set_current_dir(&target_dir)?;
            const IOS_TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/ios");
            copy_embedded_dir(&IOS_TEMPLATE_DIR, &target_dir)?;

            // Copy ios bindings
            env::set_current_dir(&project_dir)?;
            let ios_bindings = "MoproiOSBindings";
            let ios_bindings_dir = project_dir.join(&ios_bindings);
            let target_ios_bindings_dir = target_dir.join(&ios_bindings);
            fs::create_dir(target_ios_bindings_dir.clone())?;
            copy_dir(&ios_bindings_dir, &target_ios_bindings_dir)?;

            print_green_bold("Template created successfully!".to_string());
            println!();
            print_green_bold("Next steps:".to_string());
            println!();
            print_green_bold(
                "  You can now use the following command to open the app:".to_string(),
            );
            println!();
            print_bold("    open ios/MoproApp.xcodeproj".to_string());
            println!();
            print_green_bold("This will open the iOS project in Xcode.".to_string());
        }
    }
    Ok(())
}

fn select_template() -> Result<String, Box<dyn Error>> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Create template")
        .items(&TEMPLATES)
        .interact()?;

    Ok(TEMPLATES[idx].to_owned())
}

fn copy_embedded_dir(dir: &Dir, output_dir: &Path) -> std::io::Result<()> {
    for file in dir.entries() {
        let relative_path = file.path();
        let output_path = output_dir.join(relative_path);

        // Create directories as needed
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the file to the output directory
        match file.as_file() {
            Some(file) => {
                if let Err(e) = fs::write(&output_path, file.contents()) {
                    return Err(e);
                }
            }
            None => {
                if let Err(e) = copy_embedded_dir(file.as_dir().unwrap(), &output_dir) {
                    return Err(e);
                };
            }
        }
    }
    Ok(())
}

fn copy_dir(input_dir: &Path, output_dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap();
            let new_output_dir = output_dir.join(dir_name);
            fs::create_dir(&new_output_dir)?;
            copy_dir(&path, &new_output_dir)?;
        } else {
            let file_name = path.file_name().unwrap();
            let new_output_file = output_dir.join(file_name);
            fs::copy(&path, &new_output_file)?;
        }
    }
    Ok(())
}
