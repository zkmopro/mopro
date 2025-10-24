use super::Create;
use crate::constants::Platform;
use crate::create::utils::{check_bindings, copy_dir, copy_keys, download_and_extract_template};
use crate::print::print_footer_message;
use crate::style::print_green_bold;

use anyhow::{Error, Result};
use std::{fs, path::PathBuf};

pub struct ReactNative;

impl Create for ReactNative {
    const NAME: &'static str = "react-native";

    fn create(project_dir: PathBuf) -> Result<()> {
        let react_native_bindings_dir = check_bindings(&project_dir, Platform::ReactNative)?;

        let target_dir = project_dir.join(Self::NAME);
        if target_dir.exists() {
            return Err(Error::msg(format!(
                "The directory {} already exists. Please remove it and try again.",
                target_dir.display()
            )));
        }
        download_and_extract_template(
            "https://github.com/zkmopro/react-native-app/archive/refs/heads/ubrn.zip",
            &project_dir,
            Self::NAME,
        )?;

        let react_native_dir = project_dir.join("react-native-app-ubrn");
        fs::rename(react_native_dir, &target_dir)?;

        let mopro_module_dir = target_dir.join("modules/mopro");
        copy_dir(
            &react_native_bindings_dir.as_ref().unwrap(),
            &mopro_module_dir,
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
