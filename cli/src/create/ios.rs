use anyhow::{Error, Result};
use include_dir::include_dir;
use include_dir::Dir;
use std::{env, fs, path::PathBuf};

use super::Create;
use crate::create::utils::{check_bindings, copy_embedded_dir, copy_ios_bindings, copy_keys};
use crate::create::APP;
use crate::print::print_footer_message;
use crate::style::print_bold;
use crate::style::print_green_bold;

pub struct Ios;

impl Create for Ios {
    const NAME: &'static str = "ios";

    fn create(project_dir: PathBuf) -> Result<()> {
        let ios_bindings_dir = check_bindings(&project_dir, APP::IOS)?;

        let target_dir = project_dir.join(Self::NAME);
        if target_dir.exists() {
            return Err(Error::msg(format!(
                "The directory {} already exists. Please remove it and try again.",
                target_dir.display()
            )));
        }
        fs::create_dir(&target_dir)?;

        env::set_current_dir(&target_dir)?;
        const IOS_TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/ios");
        copy_embedded_dir(&IOS_TEMPLATE_DIR, &target_dir)?;

        env::set_current_dir(&project_dir)?;
        copy_ios_bindings(ios_bindings_dir, target_dir.clone())?;
        copy_keys(target_dir)?;

        Self::print_message();
        Ok(())
    }

    fn print_message() {
        print_green_bold("Template created successfully!".to_string());
        println!();
        print_green_bold("Next steps:".to_string());
        println!();
        print_green_bold("  You can now use the following command to open the app:".to_string());
        println!();
        print_bold("    open ios/MoproApp.xcodeproj".to_string());
        println!();
        print_green_bold("This will open the iOS project in Xcode.".to_string());
        print_footer_message();
    }
}
