use std::{env, fs, path::PathBuf};

use anyhow::Error;
use include_dir::include_dir;
use include_dir::Dir;

use super::Create;
use crate::create::utils::{check_bindings, copy_android_bindings, copy_embedded_dir, copy_keys};
use crate::create::Framework;
use crate::print::print_footer_message;
use crate::style::print_bold;
use crate::style::print_green_bold;

pub struct Android;

impl Create for Android {
    const NAME: &'static str = "android";

    fn create(project_dir: PathBuf) -> Result<(), Error> {
        let android_bindings_dir = check_bindings(&project_dir, Framework::Android)?;

        let target_dir = project_dir.join(Self::NAME);
        fs::create_dir(&target_dir)?;

        env::set_current_dir(&target_dir)?;
        const ANDROID_TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/android");
        copy_embedded_dir(&ANDROID_TEMPLATE_DIR, &target_dir)?;

        env::set_current_dir(&project_dir)?;
        let app_dir = target_dir.join("app");
        copy_android_bindings(&android_bindings_dir, &app_dir, "java")?;

        let assets_dir = app_dir.join("src/main/assets");
        copy_keys(assets_dir)?;

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
        print_bold(r"    open android -a Android\ Studio ".to_string());
        println!();
        print_green_bold("This will open the Android project in Android Studio.".to_string());
        print_footer_message();
    }
}
