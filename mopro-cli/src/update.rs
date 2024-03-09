use crate::utils::get_mopro_root;
use std::process::{exit, Command};

// TODO: Update this script, not currently super useful
pub fn _update_project(config: &str, adapter: &str, platforms: &Vec<String>) {
    for platform in platforms.iter() {
        // Skip the update process for the 'core' platform
        if platform == "core" {
            println!("Skipping update for platform 'core'.");
            continue;
        }

        println!("Updating project for platform {}: {}", platform, adapter);

        let mopro_root = get_mopro_root();

        // Determine the script based on the platform
        let script_name = "update.sh";
        let script_path = format!("{}/scripts/cli/{}", mopro_root, script_name);

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
