use super::{Create, Framework};
use crate::create::utils::{
    check_bindings, copy_android_bindings, copy_ios_bindings, copy_keys,
    download_and_extract_template,
};
use crate::print::print_footer_message;
use crate::style::print_bold;
use crate::style::print_green_bold;

use anyhow::{Error, Result};
use std::{fs, path::PathBuf};

pub struct ReactNative;

impl Create for ReactNative {
    const NAME: &'static str = "react-native";

    fn create(project_dir: PathBuf) -> Result<()> {
        let ios_bindings_dir = check_bindings(&project_dir, Framework::IOS)?;
        let android_bindings_dir = check_bindings(&project_dir, Framework::Android)?;

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
        fs::rename(&react_native_dir, &target_dir)?;

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
            "  You can now use the following command to open the app in VS Code:".to_string(),
        );
        println!();
        print_bold(r"    code react-native-app".to_string());
        println!();
        print_green_bold(
            "To learn more about setting up the React Native app with mopro, visit https://zkmopro.org/docs/setup/react-native-setup/".to_string(),
        );

        print_footer_message();
    }
}
