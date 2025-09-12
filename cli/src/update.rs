use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::{read_config, write_config, Config, UpdateConfig};
use crate::constants::Platform;
use crate::print::print_update_success_message;
use crate::style::{print_gray_items, print_green_bold};
use mopro_ffi::app_config::constants::{
    ANDROID_JNILIBS_DIR, ANDROID_KT_FILE, ANDROID_PACKAGE_NAME, ANDROID_UNIFFI_DIR, IOS_SWIFT_FILE,
    IOS_XCFRAMEWORKS_DIR,
};

pub fn update_bindings(
    arg_src: &Option<String>,
    arg_dest: &Option<String>,
    no_prompt: bool,
) -> Result<()> {
    let src_dir = arg_src
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir()?);

    let config_path = src_dir.join("Config.toml");
    let mut config = if config_path.exists() {
        read_config(&config_path)?
    } else {
        Config::default()
    };

    if let Some(dest_str) = arg_dest {
        let dest_path = PathBuf::from(dest_str);
        let platform_paths = detect_platform_paths(&dest_path)?;
        for (platform, platform_dest) in platform_paths {
            verify_source(&src_dir, platform)?;
            update_platform(&src_dir, &platform_dest, platform)?;
            if !no_prompt {
                maybe_store_dest(&mut config, platform, &platform_dest)?;
            }
        }
    } else {
        let mut updated_any = false;
        for platform in [Platform::Ios, Platform::Android, Platform::Web] {
            let binding_dir_name = platform.binding_dir();
            let platform_bindings_dir = src_dir.join(binding_dir_name);
            if !platform_bindings_dir.exists() {
                continue;
            }

            let dest_root = config
                .update
                .as_ref()
                .and_then(|u| match platform {
                    Platform::Ios => u.ios_dest.as_ref(),
                    Platform::Android => u.android_dest.as_ref(),
                    Platform::Web => None,
                })
                .map(PathBuf::from);

            if let Some(dest) = dest_root {
                update_platform(&src_dir, &dest, platform)?;
                updated_any = true;
            } else {
                let did_update = update_platform(&src_dir, &src_dir, platform)?;
                updated_any = updated_any || did_update;
            }
        }

        if !updated_any && !no_prompt {
            println!("No target project found in the current directory.");
            let dest_input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the path to the project you want to update")
                .interact_text()?;
            let dest_path = PathBuf::from(dest_input);
            let platform_paths = detect_platform_paths(&dest_path)?;
            for (platform, platform_dest) in platform_paths {
                verify_source(&src_dir, platform)?;
                let proceed = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!(
                        "Detected {} project.  Proceed?",
                        platform.binding_name()
                    ))
                    .default(true)
                    .interact()?;
                if !proceed {
                    continue;
                }
                update_platform(&src_dir, &platform_dest, platform)?;
                let remember = if no_prompt {
                    false
                } else {
                    Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Remember this path for future updates?")
                        .default(false)
                        .interact()?
                };
                if remember {
                    config
                        .update
                        .get_or_insert(UpdateConfig::default())
                        .set_dest(platform, platform_dest.to_string_lossy().to_string());
                }
            }
        }
    }

    write_config(&config_path, &config)?;
    print_update_success_message();

    Ok(())
}

fn verify_source(src_dir: &Path, platform: Platform) -> Result<()> {
    let binding_dir = src_dir.join(platform.binding_dir());
    if !binding_dir.exists() {
        return Err(anyhow!(
            "Source directory does not contain {}",
            platform.binding_dir()
        ));
    }
    Ok(())
}

fn detect_platform_paths(dest: &Path) -> Result<Vec<(Platform, PathBuf)>> {
    if dest.join("pubspec.yaml").exists() {
        return Ok(vec![
            (Platform::Ios, dest.join("ios")),
            (Platform::Android, dest.join("android")),
        ]);
    }

    if dest.join("package.json").exists() {
        let pkg = fs::read_to_string(dest.join("package.json"))?;
        if pkg.contains("react-native") {
            return Ok(vec![
                (Platform::Ios, dest.join("ios")),
                (Platform::Android, dest.join("android")),
            ]);
        }
    }

    if is_xcodeproj(dest) {
        return Ok(vec![(Platform::Ios, dest.to_path_buf())]);
    }

    if dest.join("build.gradle").exists() || dest.join("build.gradle.kts").exists() {
        return Ok(vec![(Platform::Android, dest.to_path_buf())]);
    }

    Err(anyhow!(
        "Could not detect project type at {}",
        dest.display()
    ))
}

fn is_xcodeproj(path: &Path) -> bool {
    if path.extension().and_then(|s| s.to_str()) == Some("xcodeproj") {
        return true;
    }
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("xcodeproj") {
                return true;
            }
        }
    }
    false
}

fn update_platform(src_root: &Path, dest_root: &Path, platform: Platform) -> Result<bool> {
    let binding_dir_name = platform.binding_dir();
    let platform_bindings_dir = src_root.join(binding_dir_name);

    if !platform_bindings_dir.exists() {
        return Ok(false);
    }

    print_green_bold(format!(
        "🔄 Updating {} bindings...",
        platform.binding_name()
    ));

    let mut updated_paths = Vec::new();
    match platform {
        Platform::Ios => {
            updated_paths.extend(update_folder(
                &platform_bindings_dir.join(IOS_XCFRAMEWORKS_DIR),
                dest_root,
                IOS_XCFRAMEWORKS_DIR,
                false,
            )?);
            updated_paths.extend(update_file(
                &platform_bindings_dir.join(IOS_SWIFT_FILE),
                dest_root,
                IOS_SWIFT_FILE,
            )?);
        }
        Platform::Android => {
            let jnilib_path = platform_bindings_dir.join(ANDROID_JNILIBS_DIR);
            let kotlin_path = platform_bindings_dir
                .join(ANDROID_UNIFFI_DIR)
                .join(ANDROID_PACKAGE_NAME)
                .join(ANDROID_KT_FILE);

            updated_paths.extend(update_file(&kotlin_path, dest_root, ANDROID_KT_FILE)?);
            updated_paths.extend(update_folder(
                &jnilib_path,
                dest_root,
                ANDROID_JNILIBS_DIR,
                true,
            )?);
        }
        Platform::Web => {
            updated_paths.extend(update_folder(
                &platform_bindings_dir,
                dest_root,
                binding_dir_name,
                false,
            )?);
        }
    };

    print_gray_items(updated_paths.clone());
    Ok(!updated_paths.is_empty())
}

pub fn update_folder(
    source_dir: &Path,
    search_root: &Path,
    target_dir_name: &str,
    gracefully: bool,
) -> Result<Vec<String>> {
    let mut updated_paths = Vec::new();
    if !source_dir.exists() {
        return Err(anyhow!(
             "Source directory does not exist: {}, Please make sure 'mopro build' has been run successfully",
             source_dir.display()
         ));
    }
    let canonical_source = source_dir.canonicalize()?;

    for entry in WalkDir::new(search_root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_dir() && entry.file_name() == target_dir_name {
            let path = entry.path();
            let canonical_target = path.canonicalize()?;

            if canonical_source == canonical_target {
                continue;
            }

            if !gracefully {
                fs::remove_dir_all(path)?;
            }

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

pub fn update_file(
    source_file: &Path,
    search_root: &Path,
    target_file_name: &str,
) -> Result<Vec<String>> {
    let mut updated_file_paths = Vec::new();
    if !source_file.exists() {
        return Err(anyhow!(
             "Source file does not exist: {}, Please make sure 'mopro build' has been run successfully",
             source_file.display()
         ));
    }
    let canonical_source = source_file.canonicalize()?;
    for entry in WalkDir::new(search_root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() && entry.file_name() == target_file_name {
            let path = entry.path();
            let canonical_target = path.canonicalize()?;

            if canonical_source == canonical_target {
                continue;
            }

            fs::copy(source_file, path)?;

            updated_file_paths.push(path.display().to_string());
        }
    }

    Ok(updated_file_paths)
}

fn maybe_store_dest(config: &mut Config, platform: Platform, dest: &Path) -> Result<()> {
    let update_cfg = config.update.get_or_insert(UpdateConfig::default());
    let existing = match platform {
        Platform::Ios => &update_cfg.ios_dest,
        Platform::Android => &update_cfg.android_dest,
        Platform::Web => &None,
    };
    if existing.is_none() {
        let remember = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Remember this path for future updates?")
            .default(false)
            .interact()?;
        if remember {
            update_cfg.set_dest(platform, dest.to_string_lossy().to_string());
        }
    }
    Ok(())
}

trait UpdateConfigExt {
    fn set_dest(&mut self, platform: Platform, dest: String);
}

impl UpdateConfigExt for UpdateConfig {
    fn set_dest(&mut self, platform: Platform, dest: String) {
        match platform {
            Platform::Ios => self.ios_dest = Some(dest),
            Platform::Android => self.android_dest = Some(dest),
            Platform::Web => {}
        }
    }
}
