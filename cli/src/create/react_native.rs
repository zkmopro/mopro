use super::Create;
use crate::constants::Platform;
use crate::create::utils::{
    check_bindings, copy_android_bindings, copy_ios_bindings, copy_keys,
    download_and_extract_template,
};
use crate::print::print_footer_message;
use crate::style::print_green_bold;

use anyhow::{Error, Result};
use std::{fs, path::PathBuf};

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

        let mopro_module_dir = target_dir.join("modules/mopro");
        copy_ios_bindings(ios_bindings_dir, mopro_module_dir.join("ios"))?;

        copy_android_bindings(
            &android_bindings_dir,
            &mopro_module_dir.join("android"),
            "java",
        )?;

        let assets_dir = target_dir.join("assets/keys");
        fs::remove_dir_all(&assets_dir)?;
        fs::create_dir(&assets_dir)?;

        copy_keys(assets_dir)?;

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

impl ReactNative {
    pub fn create_with_platform(project_dir: PathBuf, platform: String) -> Result<()> {
        // Validate platform input
        if platform != "ios" && platform != "android" {
            return Err(Error::msg(format!(
                "Invalid platform selected: {}. Must be 'ios' or 'android'.",
                platform
            )));
        }

        // Only check the selected platform
        let ios_bindings_dir = if platform == "ios" {
            Some(check_bindings(&project_dir, Platform::Ios)?)
        } else {
            None
        };
        let android_bindings_dir = if platform == "android" {
            Some(check_bindings(&project_dir, Platform::Android)?)
        } else {
            None
        };

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

        let mopro_module_dir = target_dir.join("modules/mopro");

        // Copy bindings based on selected platform
       
        match platform.as_str() {
            "ios" => {
                if let Some(dir) = ios_bindings_dir {
                    copy_ios_bindings(dir, mopro_module_dir.join("ios"))?;
                }
            }
            "android" => {
                if let Some(dir) = android_bindings_dir {
                    copy_android_bindings(&dir, &mopro_module_dir.join("android"), "java")?;
                }
            }
            _ => unreachable!("Platform should already be validated"),
        }
        

        // Keys directory (same as before)
        let assets_dir = target_dir.join("assets/keys");
        if assets_dir.exists() {
            fs::remove_dir_all(&assets_dir)?;
        }
        fs::create_dir_all(&assets_dir)?;
        copy_keys(assets_dir)?;

        Self::print_message();
        Ok(())
    }
}


