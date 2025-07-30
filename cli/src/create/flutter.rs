use anyhow::Error;
use std::{fs, path::PathBuf};

use super::Create;
use crate::constants::Platform;
use crate::create::utils::{
    check_bindings, copy_android_bindings, copy_dir, copy_keys, download_and_extract_template,
};
use crate::print::print_footer_message;
use crate::style::print_green_bold;

use mopro_ffi::app_config::constants::{IOS_SWIFT_FILE, IOS_XCFRAMEWORKS_DIR};

pub struct Flutter;

impl Create for Flutter {
    const NAME: &'static str = "flutter";

    /// Legacy behavior: build for BOTH platforms
    fn create(project_dir: PathBuf) -> Result<(), Error> {
        // Check both bindings
        let ios_bindings_dir = check_bindings(&project_dir, Platform::Ios)?;
        let android_bindings_dir = check_bindings(&project_dir, Platform::Android)?;

        Self::create_internal(
            project_dir,
            Some(ios_bindings_dir),
            Some(android_bindings_dir),
        )
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

impl Flutter {
    /// New API for platform-aware template creation
    pub fn create_with_platform(project_dir: PathBuf, platform: String) -> Result<(), Error> {
        // Validate platform input
        if platform != "ios" && platform != "android" {
            return Err(Error::msg(format!(
                "Invalid platform selected: {}. Must be 'ios' or 'android'.",
                platform
            )));
        }

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

        Self::create_internal(project_dir, ios_bindings_dir, android_bindings_dir)
    }

    /// Core creation logic
    fn create_internal(
        project_dir: PathBuf,
        ios_bindings_dir: Option<PathBuf>,
        android_bindings_dir: Option<PathBuf>,
    ) -> Result<(), Error> {
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

        let mopro_flutter_plugin_dir = target_dir.join("mopro_flutter_plugin");

        // Handle iOS if provided
        if let Some(ios_dir) = ios_bindings_dir {
            let xcframeworks_dir = ios_dir.join(IOS_XCFRAMEWORKS_DIR);
            let mopro_swift_file = ios_dir.join(IOS_SWIFT_FILE);

            let ios_target_dir = mopro_flutter_plugin_dir.join("ios");
            let mopro_bindings_dir = ios_target_dir.join(IOS_XCFRAMEWORKS_DIR);
            let classes_dir = ios_target_dir.join("Classes");

            fs::remove_dir_all(&mopro_bindings_dir)?;
            fs::create_dir(&mopro_bindings_dir)?;
            copy_dir(&xcframeworks_dir, &mopro_bindings_dir)?;

            fs::remove_file(classes_dir.join(IOS_SWIFT_FILE))?;
            fs::copy(mopro_swift_file, classes_dir.join(IOS_SWIFT_FILE))?;
        }

        // Handle Android if provided
        if let Some(android_dir) = android_bindings_dir {
            copy_android_bindings(
                &android_dir,
                &mopro_flutter_plugin_dir.join("android"),
                "kotlin",
            )?;
        }

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
}
