use super::Create;
use crate::constants::Platform;
use crate::create::utils::{
    check_bindings, copy_android_bindings, copy_dir, copy_keys, download_and_extract_template,
};
use crate::print::print_footer_message;
use crate::style::print_green_bold;
use crate::utils::project_name_from_toml;
use anyhow::Error;
use convert_case::{Case, Casing};
use std::ops::Add;
use std::{fs, path::PathBuf};

pub struct Flutter;

impl Create for Flutter {
    const NAME: &'static str = "flutter";

    fn create(project_dir: PathBuf) -> Result<(), Error> {
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

        let identifier = project_name_from_toml(&project_dir);
        let xcframework_name = identifier
            .to_case(Case::UpperCamel)
            .add("Bindings.xcframework");
        let swift_name = identifier.add(".swift");

        let xcframeworks_dir = ios_bindings_dir.join(&xcframework_name);
        let mopro_swift_file = ios_bindings_dir.join(&swift_name);

        let mopro_flutter_plugin_dir = target_dir.join("mopro_flutter_plugin");
        let ios_dir = mopro_flutter_plugin_dir.join("ios");
        let mopro_bindings_dir = ios_dir.join(&xcframework_name);
        let classes_dir = ios_dir.join("Classes");

        fs::remove_dir_all(&mopro_bindings_dir)?;
        fs::create_dir(&mopro_bindings_dir)?;
        copy_dir(&xcframeworks_dir, &mopro_bindings_dir)?;

        fs::remove_file(classes_dir.join(&swift_name))?;
        fs::copy(mopro_swift_file, classes_dir.join(&swift_name))?;

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
            "  Refer to the README.md in the `flutter` folder for instructions on running the app."
                .to_string(),
        );
        print_footer_message();
    }
}
