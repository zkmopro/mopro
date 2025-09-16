use super::Create;
use crate::constants::Platform;
use crate::create::utils::{check_bindings, copy_keys, download_and_extract_template};
use crate::print::print_footer_message;
use crate::style::print_green_bold;
use crate::update::{update_file, update_folder};

use anyhow::{Error, Result};
use mopro_ffi::app_config::constants::{
    ANDROID_JNILIBS_DIR, ANDROID_KT_FILE, ANDROID_UNIFFI_DIR, IOS_SWIFT_FILE, IOS_XCFRAMEWORKS_DIR,
};
use mopro_ffi::app_config::{project_name_from_toml, snake_to_pascal_case};
use std::{env, fs, path::PathBuf};

pub struct ReactNative;

impl Create for ReactNative {
    const NAME: &'static str = "react-native";

    fn create(project_dir: PathBuf) -> Result<()> {
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
            "https://codeload.github.com/zkmopro/react-native-app/zip/refs/heads/main",
            &project_dir,
            Self::NAME,
        )?;

        let react_native_dir = project_dir.join("react-native-app-main");
        fs::rename(react_native_dir, &target_dir)?;

        // Compute dynamic package name based on project
        let uniffi_identifier = project_name_from_toml(&project_dir)?;
        let package_name = snake_to_pascal_case(&uniffi_identifier);

        // Apply dynamic package name substitution to the downloaded template
        apply_package_name_substitution(&target_dir, &package_name)?;

        // Update bindings in the React Native module directory
        update_bindings_in_react_native_module(
            &target_dir,
            ios_bindings_dir,
            android_bindings_dir,
            &package_name,
        )?;

        // Setup assets directory with keys
        setup_assets_directory(&target_dir)?;

        Self::print_message();
        Ok(())
    }

    fn print_message() {
        print_green_bold("React Native template created successfully!".to_string());
        println!();
        print_green_bold("Next steps:".to_string());
        println!();
        print_green_bold(
            "  Refer to the README.md in the `react-native` folder for instructions on running the app.".to_string(),
        );
        print_footer_message();
    }
}

fn apply_package_name_substitution(target_dir: &PathBuf, package_name: &str) -> Result<()> {
    // Update MoproModule.kt with {{PACKAGE_NAME}} placeholder
    let module_file = target_dir
        .join("modules")
        .join("mopro")
        .join("android")
        .join("src")
        .join("main")
        .join("java")
        .join("expo")
        .join("modules")
        .join("mopro")
        .join("MoproModule.kt");

    if module_file.exists() {
        let content = fs::read_to_string(&module_file)?;
        let updated_content = content.replace("{{PACKAGE_NAME}}", package_name);
        fs::write(&module_file, updated_content)?;
        println!(
            "Updated React Native template with package name: {}",
            package_name
        );
    } else {
        println!("Warning: React Native module file not found at expected path");
    }

    Ok(())
}

fn update_bindings_in_react_native_module(
    target_dir: &PathBuf,
    ios_bindings_dir: Option<PathBuf>,
    android_bindings_dir: Option<PathBuf>,
    package_name: &str,
) -> Result<()> {
    let mopro_module_dir = target_dir.join("modules/mopro");
    let previous_dir = env::current_dir()?;

    // Handle iOS bindings
    if let Some(ios_dir) = ios_bindings_dir {
        let ios_target_dir = mopro_module_dir.join("ios");
        env::set_current_dir(&ios_target_dir)?;

        let xcframeworks_dir = ios_dir.join(IOS_XCFRAMEWORKS_DIR);
        let mopro_swift_file = ios_dir.join(IOS_SWIFT_FILE);
        let current_dir = env::current_dir()?;

        update_folder(&xcframeworks_dir, &current_dir, IOS_XCFRAMEWORKS_DIR, false)?;
        update_file(&mopro_swift_file, &current_dir, IOS_SWIFT_FILE)?;
    }

    // Handle Android bindings
    if let Some(android_dir) = android_bindings_dir {
        let android_target_dir = mopro_module_dir.join("android");
        env::set_current_dir(&android_target_dir)?;

        let jnilib_path = android_dir.join(ANDROID_JNILIBS_DIR);
        let kotlin_path = android_dir
            .join(ANDROID_UNIFFI_DIR)
            .join(package_name)
            .join(ANDROID_KT_FILE);
        let current_dir = env::current_dir()?;

        update_file(&kotlin_path, &current_dir, ANDROID_KT_FILE)?;
        update_folder(&jnilib_path, &current_dir, ANDROID_JNILIBS_DIR, true)?;
    }

    env::set_current_dir(previous_dir)?;
    Ok(())
}

fn setup_assets_directory(target_dir: &PathBuf) -> Result<()> {
    let assets_dir = target_dir.join("assets/keys");

    if assets_dir.exists() {
        fs::remove_dir_all(&assets_dir)?;
    }
    fs::create_dir_all(&assets_dir)?;
    copy_keys(assets_dir)?;

    Ok(())
}
