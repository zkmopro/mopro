use crate::config::read_config;
use crate::constants::{
    Platform, JNILIBS_DIR, MOPRO_KOTLIN_FILE, MOPRO_SWIFT_FILE, XCFRAMEWORK_NAME,
};
use crate::print::print_update_success_message;
use crate::style::{print_gray_items, print_green_bold};
use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn update_bindings() -> Result<()> {
    let project_dir = std::env::current_dir()?; // TODO - this is a strong assumption that we update within the project directory

    let config = read_config(&project_dir.join("Config.toml")).ok();
    let ios_bindings_dir = project_dir.join(Platform::Ios.binding_dir(&config));
    let android_bindings_dir = project_dir.join(Platform::Android.binding_dir(&config));
    let web_bindings_dir = project_dir.join(Platform::Web.binding_dir(&config));

    // Update iOS bindings
    if ios_bindings_dir.exists() {
        print_green_bold("🔄 Updating iOS bindings...".to_string());
        let updated_xcframework_paths =
            update_folder(&ios_bindings_dir.join(XCFRAMEWORK_NAME), XCFRAMEWORK_NAME)?; // TODO - handle custom name of XCFramework
        print_gray_items(updated_xcframework_paths);
        let updated_swift_paths =
            update_file(&ios_bindings_dir.join(MOPRO_SWIFT_FILE), MOPRO_SWIFT_FILE)?;
        print_gray_items(updated_swift_paths);
    }
    // Update Android bindings
    if android_bindings_dir.exists() {
        print_green_bold("🔄 Updating Android bindings...".to_string());
        let updated_jnilib_paths =
            update_folder(&android_bindings_dir.join(JNILIBS_DIR), JNILIBS_DIR)?;
        print_gray_items(updated_jnilib_paths);
        let updated_kotlin_paths = update_file(
            &android_bindings_dir
                .join("uniffi")
                .join("mopro")
                .join(MOPRO_KOTLIN_FILE),
            MOPRO_KOTLIN_FILE,
        )?;
        print_gray_items(updated_kotlin_paths);
    }
    // Update Wasm bindings
    if web_bindings_dir.exists() {
        print_green_bold("🔄 Updating Web bindings...".to_string());
        let updated_web_paths =
            update_folder(&web_bindings_dir, &Platform::Web.binding_dir(&config))?;
        print_gray_items(updated_web_paths);
    }

    print_update_success_message();

    Ok(())
}

fn update_folder(source_dir: &Path, target_dir_name: &str) -> Result<Vec<String>> {
    let mut updated_paths = Vec::new();
    if !source_dir.exists() {
        return Err(anyhow::anyhow!(
            "Source directory does not exist: {}, Please make sure 'mopro build' has been run successfully",
            source_dir.display()
        ));
    }
    let canonical_source = source_dir.canonicalize()?;

    for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
        if entry.file_type().is_dir() && entry.file_name() == target_dir_name {
            let path = entry.path();
            let canonical_target = path.canonicalize()?;

            // Skip if source and target are the same directory
            if canonical_source == canonical_target {
                continue;
            }

            // Remove old directory
            fs::remove_dir_all(path)?;

            // Copy the new directory into the parent of the old one
            fs_extra::dir::copy(
                source_dir,
                path.parent().unwrap(),
                &fs_extra::dir::CopyOptions::new()
                    .overwrite(true)
                    .copy_inside(true),
            )?;

            updated_paths.push(path.display().to_string());
        }
    }

    Ok(updated_paths)
}

fn update_file(source_file: &Path, target_file_name: &str) -> Result<Vec<String>> {
    let mut updated_file_paths = Vec::new();
    if !source_file.exists() {
        return Err(anyhow::anyhow!(
            "Source file does not exist: {}, Please make sure 'mopro build' has been run successfully",
            source_file.display()
        ));
    }
    let canonical_source = source_file.canonicalize()?;

    for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() && entry.file_name() == target_file_name {
            let path = entry.path();
            let canonical_target = path.canonicalize()?;

            // Skip if source and target are the same file
            if canonical_source == canonical_target {
                continue;
            }

            // Replace the file
            fs::copy(source_file, path)?;

            updated_file_paths.push(path.display().to_string());
        }
    }

    Ok(updated_file_paths)
}
