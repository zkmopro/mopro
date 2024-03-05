use std::env;
use std::process::{exit, Command};

pub fn build_project(config: &str, adapter: &str, platforms: &Vec<String>) {
    for platform in platforms.iter() {
        println!(
            "Building project for platform {}: {} with config {}",
            platform, adapter, config
        );

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
        let script_name = if platform == "ios" {
            "build_ios_project.sh"
        } else {
            "build_minimal_project.sh"
        };

        let script_path = format!("{}/scripts/{}", mopro_root, script_name);

        let status = Command::new("sh")
            .arg(script_path)
            .arg(config)
            .status()
            .expect("Failed to execute build script");

        if !status.success() {
            eprintln!("Cargo build failed.");
            exit(1);
        }

        println!("Build completed successfully for platform {}.", platform);
    }
}
