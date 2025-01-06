use anyhow::Error;
use std::{fs, path::PathBuf};

use super::Create;
use crate::create::utils::{
    check_android_bindings, check_ios_bindings, copy_android_bindings, copy_dir, copy_keys,
    download_and_extract_template,
};
use crate::print::print_footer_message;
use crate::style::print_bold;
use crate::style::print_green_bold;

pub struct Flutter;

impl Create for Flutter {
    const NAME: &'static str = "flutter";

    fn create(project_dir: PathBuf) -> Result<(), Error> {
        let ios_bindings_dir = check_ios_bindings(&project_dir)?;
        let android_bindings_dir = check_android_bindings(&project_dir)?;

        let target_dir = project_dir.join("flutter-app");
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
        fs::rename(&flutter_dir, &target_dir)?;

        let xcframeworks_dir = ios_bindings_dir.join("MoproBindings.xcframework");
        let mopro_swift_file = ios_bindings_dir.join("mopro.swift");

        let mopro_flutter_plugin_dir = target_dir.join("mopro_flutter_plugin");
        let ios_dir = mopro_flutter_plugin_dir.join("ios");
        let mopro_bindings_dir = ios_dir.join("MoproBindings.xcframework");
        let classes_dir = ios_dir.join("Classes");

        fs::remove_dir_all(&mopro_bindings_dir)?;
        fs::create_dir(&mopro_bindings_dir)?;
        copy_dir(&xcframeworks_dir, &mopro_bindings_dir)?;

        fs::remove_file(classes_dir.join("mopro.swift"))?;
        fs::copy(&mopro_swift_file, classes_dir.join("mopro.swift"))?;

        copy_android_bindings(
            &android_bindings_dir,
            &target_dir.join("mopro_flutter_plugin/android"),
            "kotlin",
        )?;

        let assets_dir = target_dir.join("assets");
        fs::remove_dir_all(&assets_dir)?;
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
            "  You can now use the following command to open the app in Android Studio:"
                .to_string(),
        );
        println!();
        print_bold(r"    open flutter-app -a Android\ Studio ".to_string());
        println!();
        print_green_bold("  or VS Code::".to_string());
        println!();
        print_bold(r"    code flutter-app".to_string());
        println!();
        print_green_bold(
            "To learn more about setting up the Flutter app with mopro, visit https://zkmopro.org/docs/setup/flutter-setup/".to_string(),
        );

        print_footer_message();
    }
}
