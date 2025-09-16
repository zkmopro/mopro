use anyhow::Error;
use std::{env, fs, path::PathBuf};

use super::Create;
use crate::constants::Platform;
use crate::create::utils::{check_bindings, copy_keys, download_and_extract_template};
use crate::print::print_footer_message;
use crate::style::print_green_bold;
use crate::update::{update_file, update_folder};
use mopro_ffi::app_config::{project_name_from_toml, snake_to_pascal_case};

use mopro_ffi::app_config::constants::{
    ANDROID_JNILIBS_DIR, ANDROID_KT_FILE, ANDROID_UNIFFI_DIR, IOS_SWIFT_FILE, IOS_XCFRAMEWORKS_DIR,
};

pub struct Flutter;

impl Create for Flutter {
    const NAME: &'static str = "flutter";

    fn create(project_dir: PathBuf) -> Result<(), Error> {
        // Check both bindings
        let ios_bindings_dir = check_bindings(&project_dir, Platform::Ios)?;
        let android_bindings_dir = check_bindings(&project_dir, Platform::Android)?;

        let target_dir = project_dir.join(Self::NAME);
        if target_dir.exists() {
            return Err(Error::msg(format!(
                "The directory {} already exists. Please remove it and try again.",
                target_dir.display()
            )));
        }

        download_and_extract_template(
            "https://github.com/zkmopro/flutter-app/archive/refs/heads/main.zip",
            &project_dir,
            Self::NAME,
        )?;

        let flutter_dir = project_dir.join("flutter-app-main");
        fs::rename(flutter_dir, &target_dir)?;

        // Compute dynamic package name based on project (used for template substitution and Android bindings)
        let uniffi_identifier = project_name_from_toml(&project_dir)?;
        let package_name = snake_to_pascal_case(&uniffi_identifier);

        // Apply dynamic package name substitution to the downloaded template
        apply_package_name_substitution(&target_dir, &package_name)?;

        let mopro_flutter_plugin_dir = target_dir.join("mopro_flutter_plugin");
        let previous_dir = env::current_dir()?;
        env::set_current_dir(&mopro_flutter_plugin_dir)?;

        // Handle iOS if provided
        if let Some(ios_dir) = ios_bindings_dir {
            let xcframeworks_dir = ios_dir.join(IOS_XCFRAMEWORKS_DIR);
            let mopro_swift_file = ios_dir.join(IOS_SWIFT_FILE);
            let current_dir = env::current_dir()?;
            let _ = update_folder(&xcframeworks_dir, &current_dir, IOS_XCFRAMEWORKS_DIR, false)?;
            let _ = update_file(&mopro_swift_file, &current_dir, IOS_SWIFT_FILE)?;
        }

        // Handle Android if provided
        if let Some(android_dir) = android_bindings_dir {
            let jnilib_path = android_dir.join(ANDROID_JNILIBS_DIR);
            let kotlin_path = android_dir
                .join(ANDROID_UNIFFI_DIR)
                .join(&package_name)
                .join(ANDROID_KT_FILE);

            let current_dir = env::current_dir()?;
            let _ = update_file(&kotlin_path, &current_dir, ANDROID_KT_FILE)?;
            let _ = update_folder(&jnilib_path, &current_dir, ANDROID_JNILIBS_DIR, true)?;
        }
        env::set_current_dir(previous_dir)?;

        // Keys
        let assets_dir = target_dir.join("assets");
        if assets_dir.exists() {
            fs::remove_dir_all(&assets_dir)?;
        }
        fs::create_dir(&assets_dir)?;
        copy_keys(assets_dir)?;

        Self::print_message();
        Ok(())
    }

    fn print_message() {
        print_green_bold("Flutter template created successfully!".to_string());
        println!();
        print_green_bold("Next steps:".to_string());
        println!();
        print_green_bold(
            "  Refer to the README.md in the `flutter` folder for instructions on running the app."
                .to_string(),
        );
        print_footer_message();
    }
}

fn apply_package_name_substitution(target_dir: &PathBuf, package_name: &str) -> Result<(), Error> {
    // Update MoproFlutterPlugin.kt with {{PACKAGE_NAME}} placeholder
    let plugin_file = target_dir
        .join("mopro_flutter_plugin")
        .join("android")
        .join("src")
        .join("main")
        .join("kotlin")
        .join("com")
        .join("example")
        .join("mopro_flutter")
        .join("MoproFlutterPlugin.kt");

    if plugin_file.exists() {
        let content = fs::read_to_string(&plugin_file)?;
        let updated_content = content.replace("{{PACKAGE_NAME}}", &package_name);
        fs::write(&plugin_file, updated_content)?;
        println!(
            "Updated Flutter template with package name: {}",
            package_name
        );
    } else {
        println!("Warning: Flutter plugin file not found at expected path");
    }

    Ok(())
}
