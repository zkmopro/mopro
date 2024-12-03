use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use include_dir::include_dir;
use include_dir::Dir;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use reqwest::blocking::Client;
use zip::ZipArchive;

use crate::print::print_create_android_success_message;
use crate::print::print_create_flutter_success_message;
use crate::print::print_create_ios_success_message;
use crate::style;

const TEMPLATES: [&str; 4] = ["ios", "android", "flutter", "react-native"];

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
            let ios_bindings = "MoproiOSBindings";
            let ios_bindings_dir = project_dir.join(&ios_bindings);

            // Check if the dir exists and is not empty
            if !ios_bindings_dir.exists() || fs::read_dir(&ios_bindings_dir)?.count() == 0 {
                style::print_red_bold(
                    "iOS bindings are required to create the iOS template. Please run 'mopro build' to create them.".to_string(),
                );
                return Ok(());
            }

            let platform_name = "ios";
            let target_dir = project_dir.join(&platform_name);
            fs::create_dir(&target_dir)?;

            // Change directory to the project directory
            env::set_current_dir(&target_dir)?;
            const IOS_TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/ios");
            copy_embedded_dir(&IOS_TEMPLATE_DIR, &target_dir)?;

            // Copy ios bindings
            env::set_current_dir(&project_dir)?;

            let target_ios_bindings_dir = target_dir.join(&ios_bindings);
            fs::create_dir(target_ios_bindings_dir.clone())?;
            copy_dir(&ios_bindings_dir, &target_ios_bindings_dir)?;

            // Copy keys
            copy_keys(target_dir)?;

            print_create_ios_success_message();
        }

        if platform.contains(TEMPLATES[1]) {
            let android_bindings = "MoproAndroidBindings";
            let android_bindings_dir = project_dir.join(&android_bindings);
            // Check if the dir exists and is not empty
            if !android_bindings_dir.exists() || fs::read_dir(&android_bindings_dir)?.count() == 0 {
                style::print_red_bold(
                    "Android bindings are required to create Android template. Please run 'mopro build' to create them.".to_string(),
                );
                return Ok(());
            }

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
            let jni_libs_name = "jniLibs";
            let uniffi_name = "uniffi";
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
            copy_keys(assets_dir)?;

            print_create_android_success_message();
        }

        if platform.contains(TEMPLATES[2]) {
            let ios_bindings = "MoproiOSBindings";
            let ios_bindings_dir = project_dir.join(&ios_bindings);
            // Check if the dir exists and is not empty
            let mut ios_missing = false;
            if !ios_bindings_dir.exists() || fs::read_dir(&ios_bindings_dir)?.count() == 0 {
                ios_missing = true;
            }
            let android_bindings = "MoproAndroidBindings";
            let android_bindings_dir = project_dir.join(&android_bindings);
            // Check if the dir exists and is not empty
            let mut android_missing = false;
            if !android_bindings_dir.exists() || fs::read_dir(&android_bindings_dir)?.count() == 0 {
                android_missing = true;
            }

            if ios_missing || android_missing {
                style::print_red_bold(
                    "Both iOS and Android bindings are required to create a Flutter template. Please run 'mopro build' to create the missing bindings.".to_string(),
                );
                return Ok(());
            }

            download_and_extract_template(
                "https://github.com/zkmopro/flutter-app/archive/refs/heads/main.zip",
                &project_dir,
                TEMPLATES[2],
            )?;

            // The resulting directory will have -main attached to its name, we need to remove it
            let flutter_dir = project_dir.join("flutter-app-main");
            let target_dir = project_dir.join("flutter-app");
            fs::rename(&flutter_dir, &target_dir)?;

            // Copy iOS bindings
            let xcframeworks_dir = ios_bindings_dir.join("MoproBindings.xcframework");
            let mopro_swift_file = ios_bindings_dir.join("mopro.swift");

            let mopro_flutter_plugin_dir = target_dir.join("mopro_flutter_plugin");
            let ios_dir = mopro_flutter_plugin_dir.join("ios");
            let mopro_bindings_dir = ios_dir.join("MoproBindings.xcframework");
            let classes_dir = ios_dir.join("Classes");

            // Replace the existing MoproBindings.xcframework dir with the one from the MoproiOSBindings
            fs::remove_dir_all(&mopro_bindings_dir)?;
            fs::create_dir(&mopro_bindings_dir)?;
            copy_dir(&xcframeworks_dir, &mopro_bindings_dir)?;

            // Replace the mopro.swift file
            fs::remove_file(&classes_dir.join("mopro.swift"))?;
            fs::copy(&mopro_swift_file, &classes_dir.join("mopro.swift"))?;

            // Copy Android bindings

            let jni_libs_name = "jniLibs";
            let uniffi_name = "uniffi";
            let jni_libs_path = android_bindings_dir.join(&jni_libs_name);
            let uniffi_path = android_bindings_dir.join(&uniffi_name);
            let target_jni_libs_path = target_dir
                .join("mopro_flutter_plugin")
                .join("android")
                .join("src")
                .join("main")
                .join("jniLibs");
            let target_uniffi_path = target_dir
                .join("mopro_flutter_plugin")
                .join("android")
                .join("src")
                .join("main")
                .join("kotlin")
                .join("uniffi");
            fs::remove_dir_all(target_jni_libs_path.clone())?;
            fs::create_dir(target_jni_libs_path.clone())?;
            copy_dir(&jni_libs_path, &target_jni_libs_path)?;
            fs::remove_dir_all(target_uniffi_path.clone())?;
            fs::create_dir(target_uniffi_path.clone())?;
            copy_dir(&uniffi_path, &target_uniffi_path)?;

            // Copy the keys to the flutter assets dir
            let assets_dir = target_dir.join("assets");
            // Clear the assets dir before copying
            fs::remove_dir_all(&assets_dir)?;
            fs::create_dir(&assets_dir)?;

            copy_keys(assets_dir)?;

            print_create_flutter_success_message();
        }

        if platform.contains(TEMPLATES[3]) {
            download_and_extract_template(
                "https://codeload.github.com/zkmopro/react-native-app/zip/refs/heads/main",
                &project_dir,
                TEMPLATES[3],
            )?;
            // The resulting directory will have -main attached to its name, remove it
            let react_native_dir = project_dir.join("react-native-app-main");
            let target_dir = project_dir.join("react-native-app");
            fs::rename(&react_native_dir, &target_dir)?;
        }
    }
    Ok(())
}

fn copy_keys(target_dir: std::path::PathBuf) -> Result<(), anyhow::Error> {
    const CIRCOM_KEYS_DIR: Dir =
        include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/circom");
    const HALO2_KEYS_DIR: Dir =
        include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/halo2");
    copy_embedded_file(&CIRCOM_KEYS_DIR, &target_dir)?;
    copy_embedded_file(&HALO2_KEYS_DIR, &target_dir)?;
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

fn download_and_extract_template(url: &str, dest: &Path, platform: &str) -> anyhow::Result<()> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{msg} {spinner} {bytes} downloaded")
            .unwrap(),
    );
    pb.set_message(format!("Downloading {} template...", platform));

    // Save to a temporary file
    let temp_zip_path = dest.join("template.zip");
    let mut dest_file = File::create(&temp_zip_path)?;

    // Create a buffer and copy while updating the progress bar
    let mut buffer = [0u8; 8192];
    let mut downloaded: u64 = 0;
    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        dest_file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete!");

    let zip_file = File::open(&temp_zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    archive.extract(dest)?;

    // Clean up
    std::fs::remove_file(&temp_zip_path)?;

    Ok(())
}
