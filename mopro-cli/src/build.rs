use crate::utils::get_mopro_root;
use std::process::{exit, Command};

pub fn build_circuit(config: &str) {
    println!("Building circuit with config {}", config);

    let mopro_root = get_mopro_root();
    let script_name = "build_circuit_project.sh";
    let script_path = format!("{}/scripts/{}", mopro_root, script_name);

    let status = Command::new("sh")
        .arg(script_path)
        .arg(config)
        .status()
        .expect("Failed to execute build script");

    if !status.success() {
        eprintln!("Unable to build circuit.");
        exit(1);
    }

    println!("Circuit build completed successfully.");
}

pub fn build_project(config: &str, adapter: &str, platforms: &Vec<String>) {
    for platform in platforms.iter() {
        println!(
            "Building project for platform {}: {} with config {}",
            platform, adapter, config
        );

        let mopro_root = get_mopro_root();

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
