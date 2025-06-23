use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use mopro_ffi::app_config::constants::{
    ANDROID_JNILIBS_DIR, ANDROID_KT_FILE, ANDROID_PACKAGE_NAME, ANDROID_UNIFFI_DIR, IOS_SWIFT_FILE,
    IOS_XCFRAMEWORKS_DIR,
};

use crate::constants::Platform;
use crate::print::print_update_success_message;
use crate::style::{print_gray_items, print_green_bold};

pub fn update_bindings() -> Result<()> {
    let project_dir = std::env::current_dir()?;

    for platform in [Platform::Ios, Platform::Android, Platform::Web].iter() {
        let binding_dir_name = platform.binding_dir();
        let platform_bindings_dir = project_dir.join(binding_dir_name);

        if !platform_bindings_dir.exists() {
            continue;
        }

        print_green_bold(format!(
            "🔄 Updating {} bindings...",
            platform.binding_name()
        ));

        let mut updated_paths = vec![];

        match platform {
            Platform::Ios => {
                updated_paths.extend(update_folder(
                    &platform_bindings_dir.join(IOS_XCFRAMEWORKS_DIR),
                    IOS_XCFRAMEWORKS_DIR,
                    false,
                )?);
                updated_paths.extend(update_file(
                    &platform_bindings_dir.join(IOS_SWIFT_FILE),
                    IOS_SWIFT_FILE,
                )?);
            }
            Platform::Android => {
                let jnilib_path = platform_bindings_dir.join(ANDROID_JNILIBS_DIR);
                let kotlin_path = platform_bindings_dir
                    .join(ANDROID_UNIFFI_DIR)
                    .join(ANDROID_PACKAGE_NAME)
                    .join(ANDROID_KT_FILE);

                updated_paths.extend(update_file(&kotlin_path, ANDROID_KT_FILE)?);
                updated_paths.extend(update_folder(&jnilib_path, ANDROID_JNILIBS_DIR, true)?);
            }
            Platform::Web => updated_paths.extend(update_folder(
                &platform_bindings_dir,
                binding_dir_name,
                false,
            )?),
        };

        print_gray_items(updated_paths);
    }

    print_update_success_message();

    Ok(())
}

/// Recursively updates all directories in the current directory that match `target_dir_name`.
/// If `gracefully` is `true`, existing directories are preserved and only overlapping files are overwritten.
/// This is useful for shared folders across multiple bindings (e.g., Android JNI libs).
fn update_folder(
    source_dir: &Path,
    target_dir_name: &str,
    gracefully: bool,
) -> Result<Vec<String>> {
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

            if !gracefully {
                // Remove old directory
                fs::remove_dir_all(path)?;
            }

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
