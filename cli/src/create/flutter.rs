use anyhow::Error;
use mopro_ffi::app_config::project_name_from_toml;
use std::{fs, path::PathBuf};

use super::Create;
use crate::constants::Platform;
use crate::create::utils::{check_bindings, copy_keys, download_and_extract_template};
use crate::print::print_footer_message;
use crate::style::print_green_bold;

pub struct Flutter;

impl Create for Flutter {
    const NAME: &'static str = "flutter";

    fn create(project_dir: PathBuf) -> Result<(), Error> {
        // Check both bindings
        let _ = check_bindings(&project_dir, Platform::Flutter)?;

        let target_dir = project_dir.join(Self::NAME);
        if target_dir.exists() {
            return Err(Error::msg(format!(
                "The directory {} already exists. Please remove it and try again.",
                target_dir.display()
            )));
        }

        download_and_extract_template(
            "https://github.com/zkmopro/flutter-app/archive/refs/heads/frb.zip",
            &project_dir,
            Self::NAME,
        )?;

        let flutter_dir = project_dir.join("flutter-app-frb");
        fs::rename(flutter_dir, &target_dir)?;

        // update podspecs
        let podspec_path = target_dir.join("pubspec.yaml");
        let podspec_content = fs::read_to_string(podspec_path.clone())?;
        let updated_content = podspec_content.replace(
            "path: ./mopro_flutter_bindings",
            "path: ../mopro_flutter_bindings",
        );
        fs::write(&podspec_path, updated_content)?;

        // remove mopro_flutter_bindings in flutter to avoid confusion
        fs::remove_dir_all(target_dir.join("mopro_flutter_bindings"))?;

        // Update library name
        let main_path = target_dir.join("lib/main.dart");
        let main_content = fs::read_to_string(main_path.clone())?;
        let project_name = project_name_from_toml(&project_dir)?;
        let updated_content = main_content.replace(
            "import 'package:mopro_flutter_bindings/src/rust/third_party/test_e2e.dart';",
            &format!(
                "import 'package:mopro_flutter_bindings/src/rust/third_party/{}.dart';",
                project_name
            ),
        );
        fs::write(&main_path, updated_content)?;

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
