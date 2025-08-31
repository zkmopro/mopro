use anyhow::Error;
use std::{fs, path::PathBuf};

use super::Create;
use crate::constants::Platform;
use crate::create::utils::{
    check_bindings, copy_android_bindings, copy_dir, copy_keys, download_and_extract_template,
};
use crate::update::{update_file, update_folder};

use mopro_ffi::app_config::constants::{
    ANDROID_JNILIBS_DIR, ANDROID_KT_FILE, ANDROID_PACKAGE_NAME, ANDROID_UNIFFI_DIR, IOS_SWIFT_FILE,
    IOS_XCFRAMEWORKS_DIR,
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

        // Handle iOS if provided
        if let Some(ios_dir) = ios_bindings_dir {
            let xcframeworks_dir = ios_dir.join(IOS_XCFRAMEWORKS_DIR);
            let mopro_swift_file = ios_dir.join(IOS_SWIFT_FILE);
            let _ = update_folder(&xcframeworks_dir, IOS_XCFRAMEWORKS_DIR, false)?;
            let _ = update_file(&mopro_swift_file, IOS_SWIFT_FILE)?;
        }

        // Handle Android if provided
        if let Some(android_dir) = android_bindings_dir {
            let jnilib_path = android_dir.join(ANDROID_JNILIBS_DIR);
            let kotlin_path = android_dir
                .join(ANDROID_UNIFFI_DIR)
                .join(ANDROID_PACKAGE_NAME)
                .join(ANDROID_KT_FILE);

            let _ = update_file(&kotlin_path, ANDROID_KT_FILE)?;
            let _ = update_folder(&jnilib_path, ANDROID_JNILIBS_DIR, true)?;
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
