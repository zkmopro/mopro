use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Ok;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm};
use include_dir::include_dir;
use include_dir::Dir;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use reqwest::blocking::Client;
use zip::ZipArchive;

use mopro_ffi::app_config::constants::{ANDROID_JNILIBS_DIR, ANDROID_UNIFFI_DIR};

use crate::{build::build_project, constants::Platform, style};

pub fn copy_android_bindings(
    android_bindings_dir: &Path,
    target_dir: &Path,
    language: &str,
) -> Result<()> {
    let jni_libs_path = android_bindings_dir.join(ANDROID_JNILIBS_DIR);
    let uniffi_path = android_bindings_dir.join(ANDROID_UNIFFI_DIR);
    let main_dir = target_dir.join("src").join("main");
    let target_jni_libs_path = main_dir.join(ANDROID_JNILIBS_DIR);
    let target_uniffi_path = main_dir.join(language).join(ANDROID_UNIFFI_DIR);

    if target_jni_libs_path.exists() {
        fs::remove_dir_all(target_jni_libs_path.clone())?;
    }
    fs::create_dir_all(&target_jni_libs_path)?;
    copy_dir(&jni_libs_path, &target_jni_libs_path)?;
    if target_uniffi_path.exists() {
        fs::remove_dir_all(target_uniffi_path.clone())?;
    }
    fs::create_dir_all(&target_uniffi_path)?;
    copy_dir(&uniffi_path, &target_uniffi_path)?;

    Ok(())
}

pub fn copy_ios_bindings(input_dir: PathBuf, output_dir: PathBuf) -> Result<()> {
    let ios_bindings_target_dir = output_dir.join(Platform::Ios.binding_dir());
    if ios_bindings_target_dir.exists() {
        fs::remove_dir_all(&ios_bindings_target_dir)?;
    }
    fs::create_dir_all(&ios_bindings_target_dir)?;
    copy_dir(&input_dir, &ios_bindings_target_dir)?;
    Ok(())
}

pub fn copy_embedded_file(dir: &Dir, output_dir: &Path) -> Result<()> {
    for file in dir.entries() {
        // Skip .wasm files
        if file.path().extension().is_some_and(|ext| ext == "wasm") {
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

pub fn copy_embedded_dir(dir: &Dir, output_dir: &Path) -> Result<()> {
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
                    if e.kind() == ErrorKind::AlreadyExists {
                        println!("File already exists: {output_path:?}");
                    } else {
                        return Err(e.into());
                    }
                }
            }
            None => {
                copy_embedded_dir(file.as_dir().unwrap(), output_dir)?;
            }
        }
    }
    Ok(())
}

pub fn copy_dir(input_dir: &Path, output_dir: &Path) -> Result<()> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap();
            let new_output_dir = output_dir.join(dir_name);
            fs::create_dir_all(&new_output_dir)?;
            copy_dir(&path, &new_output_dir)?;
        } else {
            let file_name = path.file_name().unwrap();
            let new_output_file = output_dir.join(file_name);
            fs::copy(&path, &new_output_file)?;
        }
    }
    Ok(())
}

pub fn copy_keys(target_dir: std::path::PathBuf) -> Result<()> {
    let key_dirs = [
        include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/circom"),
        include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/circom/witnesscalc"),
        include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/halo2"),
        include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/noir"),
    ];
    key_dirs
        .iter()
        .try_for_each(|dir| copy_embedded_file(dir, &target_dir))?;
    Ok(())
}

pub fn check_bindings(project_dir: &Path, platform: Platform) -> Result<Option<PathBuf>> {
    let bindings_dir_name = platform.binding_dir();

    let bindings_dir = project_dir.join(bindings_dir_name);
    if bindings_dir.exists() && fs::read_dir(&bindings_dir)?.count() > 0 {
        return Ok(Some(bindings_dir));
    }
    style::print_yellow(format!(
        "{} are required to create the template.",
        bindings_dir_name
    ));
    println!();

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Run `mopro build --platforms {}` now to generate them?",
            platform.as_str()
        ))
        .default(true)
        .interact()?;

    if confirm {
        build_project(
            &None,
            &Some(vec![platform.as_str().to_string()]),
            &None,
            Some(false),
            false,
        )?;

        if bindings_dir.exists() && fs::read_dir(&bindings_dir)?.count() > 0 {
            return Ok(Some(bindings_dir));
        }
    }

    Ok(None)
}

pub fn download_and_extract_template(url: &str, dest: &Path, platform: &str) -> Result<()> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{msg} {spinner} {bytes} downloaded")
            .unwrap(),
    );
    spinner.set_message(format!("Downloading {platform} template..."));

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
