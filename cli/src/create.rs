use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Error;
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
use crate::print::print_create_react_native_success_message;
use crate::print::print_create_web_success_message;
use crate::style;

const TEMPLATES: [&str; 5] = ["ios", "android", "web", "flutter", "react-native"];

pub fn create_project(arg_platform: &Option<String>) -> anyhow::Result<()> {
    let platform: String = match arg_platform.as_deref() {
        None => select_template()?,
        Some(m) => {
            if TEMPLATES.contains(&m) {
                m.to_string()
            } else {
                style::print_yellow("Invalid template selected. Please choose a valid template (e.g., 'ios', 'android', 'web', 'react-native', 'flutter').".to_string());
                select_template()?
            }
        }
    };

    let project_dir = env::current_dir()?;

    match platform.as_str() {
        "ios" => {
            let ios_bindings_dir = check_ios_bindings(&project_dir)?;

            let platform_name = "ios";
            let target_dir = project_dir.join(platform_name);
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

            print_create_ios_success_message();
        }
        "android" => {
            let android_bindings_dir = check_android_bindings(&project_dir)?;

            let platform_name = "android";
            let target_dir = project_dir.join(platform_name);
            fs::create_dir(&target_dir)?;

            env::set_current_dir(&target_dir)?;
            const ANDROID_TEMPLATE_DIR: Dir =
                include_dir!("$CARGO_MANIFEST_DIR/src/template/android");
            copy_embedded_dir(&ANDROID_TEMPLATE_DIR, &target_dir)?;

            env::set_current_dir(&project_dir)?;
            let app_dir = target_dir.join("app");
            copy_android_bindings(&android_bindings_dir, &app_dir, "java")?;

            let assets_dir = app_dir.join("src/main/assets");
            copy_keys(assets_dir)?;

            print_create_android_success_message();
        }
        "web" => {
            let wasm_bindings_dir = check_web_bindings(&project_dir)?;
            let target_dir = project_dir.join("web");
            fs::create_dir(&target_dir)?;

            env::set_current_dir(&target_dir)?;
            const WEB_TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/web");
            copy_embedded_dir(&WEB_TEMPLATE_DIR, &target_dir)?;

            env::set_current_dir(&project_dir)?;

            let target_wasm_bindings_dir = target_dir.join("MoproWasmBindings");
            fs::create_dir(target_wasm_bindings_dir.clone())?;
            copy_dir(&wasm_bindings_dir, &target_wasm_bindings_dir)?;

            let asset_dir = target_dir.join("assets");
            const HALO2_KEYS_DIR: Dir =
                include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/halo2");
            copy_embedded_file(&HALO2_KEYS_DIR, &asset_dir)?;

            print_create_web_success_message();
        }
        "flutter" => {
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
                "flutter",
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

            print_create_flutter_success_message();
        }
        "react-native" => {
            let ios_bindings_dir = check_ios_bindings(&project_dir)?;
            let android_bindings_dir = check_android_bindings(&project_dir)?;

            let target_dir = project_dir.join("react-native-app");
            if target_dir.exists() {
                return Err(Error::msg(format!(
                    "The directory {} already exists. Please remove it and try again.",
                    target_dir.display()
                )));
            }
            download_and_extract_template(
                "https://codeload.github.com/zkmopro/react-native-app/zip/refs/heads/main",
                &project_dir,
                "react-native",
            )?;

            let react_native_dir = project_dir.join("react-native-app-main");
            fs::rename(&react_native_dir, &target_dir)?;

            let mopro_module_dir = target_dir.join("modules/mopro");
            copy_ios_bindings(ios_bindings_dir, mopro_module_dir.join("ios"))?;

            copy_android_bindings(
                &android_bindings_dir,
                &mopro_module_dir.join("android"),
                "java",
            )?;

            let assets_dir = target_dir.join("assets/keys");
            fs::remove_dir_all(&assets_dir)?;
            fs::create_dir(&assets_dir)?;

            copy_keys(assets_dir)?;

            print_create_react_native_success_message();
        }
        _ => {
            return Err(Error::msg("Unknown platform selected."));
        }
    }

    Ok(())
}

fn copy_android_bindings(
    android_bindings_dir: &Path,
    target_dir: &Path,
    language: &str,
) -> anyhow::Result<()> {
    let jni_libs_name = "jniLibs";
    let uniffi_name = "uniffi";
    let jni_libs_path = android_bindings_dir.join(jni_libs_name);
    let uniffi_path = android_bindings_dir.join(uniffi_name);
    let main_dir = target_dir.join("src").join("main");
    let target_jni_libs_path = main_dir.join(jni_libs_name);
    let target_uniffi_path = main_dir.join(language).join(uniffi_name);

    if target_jni_libs_path.exists() {
        fs::remove_dir_all(target_jni_libs_path.clone())?;
    }
    fs::create_dir(&target_jni_libs_path)?;
    copy_dir(&jni_libs_path, &target_jni_libs_path)?;
    if target_uniffi_path.exists() {
        fs::remove_dir_all(target_uniffi_path.clone())?;
    }
    fs::create_dir(&target_uniffi_path)?;
    copy_dir(&uniffi_path, &target_uniffi_path)?;

    Ok(())
}

fn copy_ios_bindings(input_dir: PathBuf, output_dir: PathBuf) -> Result<(), Error> {
    let ios_bindings_target_dir = output_dir.join("MoproiOSBindings");
    if ios_bindings_target_dir.exists() {
        fs::remove_dir_all(&ios_bindings_target_dir)?;
    }
    fs::create_dir(&ios_bindings_target_dir)?;
    copy_dir(&input_dir, &ios_bindings_target_dir)?;
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
        // Skip .wasm files
        if file.path().extension().map_or(false, |ext| ext == "wasm") {
            continue;
        }

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
                copy_embedded_dir(file.as_dir().unwrap(), output_dir)?;
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

fn copy_keys(target_dir: std::path::PathBuf) -> Result<(), anyhow::Error> {
    const CIRCOM_KEYS_DIR: Dir =
        include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/circom");
    const HALO2_KEYS_DIR: Dir =
        include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/halo2");
    copy_embedded_file(&CIRCOM_KEYS_DIR, &target_dir)?;
    copy_embedded_file(&HALO2_KEYS_DIR, &target_dir)?;
    Ok(())
}

fn check_ios_bindings(project_dir: &Path) -> anyhow::Result<PathBuf> {
    let ios_bindings_dir = project_dir.join("MoproiOSBindings");
    if ios_bindings_dir.exists() && fs::read_dir(&ios_bindings_dir)?.count() > 0 {
        Ok(ios_bindings_dir)
    } else {
        Err(Error::msg("iOS bindings are required to create the template. Please run 'mopro build' to generate them."))
    }
}

fn check_android_bindings(project_dir: &Path) -> anyhow::Result<PathBuf> {
    let android_bindings_dir = project_dir.join("MoproAndroidBindings");
    if android_bindings_dir.exists() && fs::read_dir(&android_bindings_dir)?.count() > 0 {
        Ok(android_bindings_dir)
    } else {
        Err(Error::msg("Android bindings are required to create the template. Please run 'mopro build' to generate them."))
    }
}

fn check_web_bindings(project_dir: &Path) -> anyhow::Result<PathBuf> {
    let web_bindings_dir = project_dir.join("MoproWasmBindings");
    if web_bindings_dir.exists() && fs::read_dir(&web_bindings_dir)?.count() > 0 {
        Ok(web_bindings_dir)
    } else {
        Err(Error::msg("Web(WASM) bindings are required to create the template. Please run 'mopro build' to generate them."))
    }
}

fn download_and_extract_template(url: &str, dest: &Path, platform: &str) -> anyhow::Result<()> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{msg} {spinner} {bytes} downloaded")
            .unwrap(),
    );
    spinner.set_message(format!("Downloading {} template...", platform));

    // Save to a temporary file
    let temp_zip_path = dest.join("template.zip");
    let mut dest_file = File::create(&temp_zip_path)?;

    // Create a buffer and copy while updating the progress bar
    let mut buffer = [0u8; 8192];
    let mut downloaded: u64 = 0;
    let mut start_time = std::time::Instant::now();
    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        dest_file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;
        let current_time = std::time::Instant::now();
        // Tick every 50 ms
        if (current_time - start_time).as_millis() > 50 {
            spinner.set_position(downloaded);
            start_time = current_time;
        }
    }

    spinner.finish_with_message("Download complete!");

    let zip_file = File::open(&temp_zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    archive.extract(dest)?;

    // Clean up
    std::fs::remove_file(&temp_zip_path)?;

    Ok(())
}
