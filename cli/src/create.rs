use std::env;
use std::fs;
use std::path::Path;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use include_dir::include_dir;
use include_dir::Dir;

use crate::print::print_create_android_success_message;
use crate::print::print_create_ios_success_message;
use crate::style;

// TODO: add  "react-native", "flutter"
const TEMPLATES: [&str; 2] = ["ios", "android"];

pub fn create_project(arg_platform: &Option<String>) -> anyhow::Result<()> {
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

            // Copy keys
            const CIRCOM_KEYS_DIR: Dir =
                include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/circom");
            const HALO2_KEYS_DIR: Dir =
                include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/halo2");
            copy_embedded_file(&CIRCOM_KEYS_DIR, &target_dir)?;
            copy_embedded_file(&HALO2_KEYS_DIR, &target_dir)?;

            print_create_ios_success_message();
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

            // Copy keys
            let assets_dir = target_dir
                .join("app")
                .join("src")
                .join("main")
                .join("assets");
            const CIRCOM_KEYS_DIR: Dir =
                include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/circom");
            const HALO2_KEYS_DIR: Dir =
                include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/halo2");
            copy_embedded_file(&CIRCOM_KEYS_DIR, &assets_dir)?;
            copy_embedded_file(&HALO2_KEYS_DIR, &assets_dir)?;

            print_create_android_success_message();
        }
    }
    Ok(())
}

fn select_template() -> anyhow::Result<String> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Create template")
        .items(&TEMPLATES)
        .interact()?;

    Ok(TEMPLATES[idx].to_owned())
}

fn copy_embedded_file(dir: &Dir, output_dir: &Path) -> anyhow::Result<()> {
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
                    return Err(e.into());
                }
            }
            None => return Ok(()),
        }
    }
    Ok(())
}

fn copy_embedded_dir(dir: &Dir, output_dir: &Path) -> anyhow::Result<()> {
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
                    return Err(e.into());
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

fn copy_dir(input_dir: &Path, output_dir: &Path) -> anyhow::Result<()> {
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
