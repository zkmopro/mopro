use std::env;
use std::process::{exit, Command};

pub fn update_project(config: &str, adapter: &str, platforms: &Vec<String>) {
    for platform in platforms.iter() {
        // Skip the update process for the 'core' platform
        if platform == "core" {
            println!("Skipping update for platform 'core'.");
            continue;
        }

        println!("Updating project for platform {}: {}", platform, adapter);

        let mopro_root = match env::var("MOPRO_ROOT") {
            Ok(root) => root,
            Err(_) => {
                eprintln!("Error: MOPRO_ROOT environment variable is not set.");
                eprintln!("Please set MOPRO_ROOT to point to the local checkout of mopro.");
                eprintln!(
                    "For example: export MOPRO_ROOT=/Users/user/repos/github.com/oskarth/mopro"
                );
                eprintln!("Git repository: https://github.com/oskarth/mopro");
                exit(1);
            }
        };

        // Determine the script based on the platform
        let script_name = "update_bindings_project.sh";
        let script_path = format!("{}/scripts/{}", mopro_root, script_name);

        let status = Command::new("sh")
            .arg(script_path)
            .arg(&config)
            .status()
            .expect("Failed to execute update script");

        if !status.success() {
            eprintln!("Update failed for platform {}.", platform);
            exit(1);
        }

        println!("Update completed successfully for platform {}.", platform);
    }
}
