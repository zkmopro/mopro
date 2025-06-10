use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::constants::Platform;
use crate::print::print_update_success_message;
use crate::style::{print_gray_items, print_green_bold};

pub fn update_bindings() -> Result<()> {
    let project_dir = std::env::current_dir()?;
    let ios_bindings_dir = project_dir.join(Platform::Ios.binding_dir());
    let android_bindings_dir = project_dir.join(Platform::Android.binding_dir());
    let web_bindings_dir = project_dir.join(Platform::Web.binding_dir());

    // Update iOS bindings
    if ios_bindings_dir.exists() {
        print_green_bold("🔄 Updating iOS bindings...".to_string());
        let updated_xcframework_paths = update_folder(
            &ios_bindings_dir.join("MoproBindings.xcframework"),
            "MoproBindings.xcframework",
        )?;
        print_gray_items(updated_xcframework_paths);
        let updated_swift_paths =
            update_file(&ios_bindings_dir.join("mopro.swift"), "mopro.swift")?;
        print_gray_items(updated_swift_paths);
    }
    // Update Android bindings
    if android_bindings_dir.exists() {
        print_green_bold("🔄 Updating Android bindings...".to_string());
        let updated_jnilib_paths = update_folder(&android_bindings_dir.join("jniLibs"), "jniLibs")?;
        print_gray_items(updated_jnilib_paths);
        let updated_kotlin_paths = update_file(
            &android_bindings_dir
                .join("uniffi")
                .join("mopro")
                .join("mopro.kt"),
            "mopro.kt",
        )?;
        print_gray_items(updated_kotlin_paths);
    }
// Update Wasm bindings
    if web_bindings_dir.exists() {
        print_green_bold("🔄 Updating Web bindings...".to_string());
        let updated_web_paths = update_folder(&web_bindings_dir, "MoproWasmBindings")?;
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
