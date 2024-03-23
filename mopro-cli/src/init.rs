use crate::utils::get_mopro_root;
use fs_extra::dir::{self, CopyOptions};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

pub fn init_project(adapter: &str, platforms: &Vec<String>, project_name: &str) {
    println!(
        "Initializing project for platforms {:?}: {} with name {}",
        platforms, adapter, project_name
    );

    let mopro_root = get_mopro_root();

    let source_path = PathBuf::from(mopro_root)
        .join("templates")
        .join("mopro-example-app");
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let destination_path = current_dir.join(project_name);

    // Create the project directory
    if destination_path.exists() {
        eprintln!(
            "A directory with the name '{}' already exists.",
            project_name
        );
        process::exit(1);
    } else {
        fs::create_dir(&destination_path).expect("Failed to create project directory");
    }

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    // Determine which directories to copy based on the enabled platforms
    let mut entries_to_copy = Vec::new();

    // Always copy core platform
    entries_to_copy.push(source_path.join("core"));
    entries_to_copy.push(source_path.join("mopro-config.toml"));
    entries_to_copy.push(source_path.join("README.md"));
    if platforms.contains(&"ios".to_string()) {
        entries_to_copy.push(source_path.join("ios"));
    }
    if platforms.contains(&"android".to_string()) {
        entries_to_copy.push(source_path.join("android"));
    }
    if platforms.contains(&"web".to_string()) {
        entries_to_copy.push(source_path.join("web"));
    }

    // Perform the copy operation for each entry
    for entry in entries_to_copy {
        if entry.is_dir() {
            // Copy directory
            dir::copy(&entry, &destination_path, &options).expect("Failed to copy directory");
        } else {
            // Copy file
            let file_name = entry.file_name().unwrap();
            fs::copy(&entry, destination_path.join(file_name)).expect("Failed to copy file");
        }
    }

    println!("Project '{}' initialized successfully.", project_name);
}
