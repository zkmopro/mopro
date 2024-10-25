use std::{env, error::Error, fs, path::Path};

use dialoguer::{theme::ColorfulTheme, Select};
use include_dir::{include_dir, Dir};

use crate::style::{self, print_bold, print_green_bold};

// TODO: add  "react-native", "flutter"
const TEMPLATES: [&str; 2] = ["ios", "android"];

pub fn create_project(arg_platform: &Option<String>) -> Result<(), Box<dyn Error>> {
    let platform: String = match arg_platform.as_deref() {
        None => select_template()?,
        Some(m) => {
            if TEMPLATES.contains(&m) {
                m.to_string()
            } else {
                style::print_yellow("Invalid template selected. Please choose a valid template (e.g., 'ios', 'android', 'react-native', 'flutter').".to_string());
                select_template()?
            }
        }
    };

    if platform.is_empty() {
        style::print_yellow("No adapters selected. Use space to select an adapter".to_string());
        create_project(arg_platform)?
    } else {
        let project_dir = env::current_dir()?;

        if platform.contains(TEMPLATES[0]) {
            let platform_name = "ios";
            let target_dir = project_dir.join(&platform_name);
            fs::create_dir(&target_dir)?;

            // Change directory to the project directory
            env::set_current_dir(&target_dir)?;
            const IOS_TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/ios");
            copy_embedded_dir(&IOS_TEMPLATE_DIR, &target_dir)?;

            // Copy ios bindings
            env::set_current_dir(&project_dir)?;
            let ios_bindings = "MoproiOSBindings";
            let ios_bindings_dir = project_dir.join(&ios_bindings);
            let target_ios_bindings_dir = target_dir.join(&ios_bindings);
            fs::create_dir(target_ios_bindings_dir.clone())?;
            copy_dir(&ios_bindings_dir, &target_ios_bindings_dir)?;

            print_ios_success_message();
        }

        if platform.contains(TEMPLATES[1]) {
            let platform_name = "android";
            let target_dir = project_dir.join(&platform_name);
            fs::create_dir(&target_dir)?;

            // Change directory to the project directory
            env::set_current_dir(&target_dir)?;
            const ANDROID_TEMPLATE_DIR: Dir =
                include_dir!("$CARGO_MANIFEST_DIR/src/template/android");
            copy_embedded_dir(&ANDROID_TEMPLATE_DIR, &target_dir)?;

            // Copy Android bindings
            env::set_current_dir(&project_dir)?;
            let android_bindings = "MoproAndroidBindings";
            let jni_libs_name = "jniLibs";
            let uniffi_name = "uniffi";
            let android_bindings_dir = project_dir.join(&android_bindings);
            let jni_libs_path = android_bindings_dir.join(&jni_libs_name);
            let uniffi_path = android_bindings_dir.join(&uniffi_name);
            let target_jni_libs_path = target_dir
                .join("app")
                .join("src")
                .join("main")
                .join("jniLibs");
            let target_uniffi_path = target_dir
                .join("app")
                .join("src")
                .join("main")
                .join("java")
                .join("uniffi");
            fs::create_dir(target_jni_libs_path.clone())?;
            copy_dir(&jni_libs_path, &target_jni_libs_path)?;
            fs::create_dir(target_uniffi_path.clone())?;
            copy_dir(&uniffi_path, &target_uniffi_path)?;

            print_android_success_message();
        }
    }
    Ok(())
}

fn select_template() -> Result<String, Box<dyn Error>> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Create template")
        .items(&TEMPLATES)
        .interact()?;

    Ok(TEMPLATES[idx].to_owned())
}

fn copy_embedded_dir(dir: &Dir, output_dir: &Path) -> std::io::Result<()> {
    for file in dir.entries() {
        let relative_path = file.path();
        let output_path = output_dir.join(relative_path);

        // Create directories as needed
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the file to the output directory
        match file.as_file() {
            Some(file) => {
                if let Err(e) = fs::write(&output_path, file.contents()) {
                    return Err(e);
                }
            }
            None => {
                if let Err(e) = copy_embedded_dir(file.as_dir().unwrap(), &output_dir) {
                    return Err(e);
                };
            }
        }
    }
    Ok(())
}

fn copy_dir(input_dir: &Path, output_dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap();
            let new_output_dir = output_dir.join(dir_name);
            fs::create_dir(&new_output_dir)?;
            copy_dir(&path, &new_output_dir)?;
        } else {
            let file_name = path.file_name().unwrap();
            let new_output_file = output_dir.join(file_name);
            fs::copy(&path, &new_output_file)?;
        }
    }
    Ok(())
}

fn print_ios_success_message() {
    print_green_bold("Template created successfully!".to_string());
    println!();
    print_green_bold("Next steps:".to_string());
    println!();
    print_green_bold("  You can now use the following command to open the app:".to_string());
    println!();
    print_bold("    open ios/MoproApp.xcodeproj".to_string());
    println!();
    print_green_bold("This will open the iOS project in Xcode.".to_string());
    println!();
    println!(
        "📚 To learn more about mopro, visit: {}",
        style::blue_bold("https://zkmopro.org".to_string())
    );
    println!("Happy coding! 🚀");
}

fn print_android_success_message() {
    print_green_bold("Template created successfully!".to_string());
    println!();
    print_green_bold("Next steps:".to_string());
    println!();
    print_green_bold("  You can now use the following command to open the app:".to_string());
    println!();
    print_bold(r"    open android -a Android\ Studio ".to_string());
    println!();
    print_green_bold("This will open the Android project in Android Studio.".to_string());
    println!();
    println!(
        "📚 To learn more about mopro, visit: {}",
        style::blue_bold("https://zkmopro.org".to_string())
    );
    println!("Happy coding! 🚀");
}
