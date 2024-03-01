use fs_extra::dir::{self, CopyOptions};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::process::exit;

pub fn init_project(adapter: &str, platform: &str, project_name: &str) {
    println!(
        "Init project for platform {}: {} and name {}",
        platform, adapter, project_name
    );

    let mopro_root = match env::var("MOPRO_ROOT") {
        Ok(root) => root,
        Err(_) => {
            eprintln!("Error: MOPRO_ROOT environment variable is not set.");
            eprintln!("Please set MOPRO_ROOT to point to the local checkout of mopro.");
            eprintln!("For example: export MOPRO_ROOT=/Users/user/repos/github.com/oskarth/mopro");
            eprintln!("Git repository: https://github.com/oskarth/mopro");
            exit(1);
        }
    };

    let source_path = PathBuf::from(mopro_root).join("mopro-cli-example");
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

    // List of directories and files to copy
    let entries_to_copy = fs::read_dir(&source_path)
        .expect("Failed to read source directory")
        .filter_map(|entry| {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            // Exclude the `ptau` and `target` directories
            if file_name != "ptau" && file_name != "target" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>();

    // Perform the copy operation for each entry
    for entry in entries_to_copy {
        if entry.is_dir() {
            // Copy directory
            dir::copy(entry, &destination_path, &options).expect("Failed to copy directory");
        } else {
            // Copy file
            let file_name = entry.file_name().unwrap();
            fs::copy(&entry, destination_path.join(file_name)).expect("Failed to copy file");
        }
    }

    println!("Project '{}' initialized successfully.", project_name);
}
