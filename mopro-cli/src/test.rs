use std::env;
use std::process::{exit, Command};

pub fn test_project(config: &str, adapter: &str, platform: &str, test_case: &Option<String>) {
    println!(
        "Testing project on platform {} with adapter {} and config {}",
        platform, adapter, config
    );

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let config_file_path = current_dir.join(config);
    let config_file_path_str = config_file_path
        .to_str()
        .expect("Failed to convert config path to string");

    // Set the BUILD_CONFIG_PATH environment variable for the cargo test command
    let mut command = Command::new("cargo");
    command.arg("test");
    command.env("BUILD_CONFIG_PATH", config_file_path_str);

    if let Some(case) = test_case {
        command.arg(case);
    }

    let status = command.status().expect("Failed to execute cargo test");

    if !status.success() {
        eprintln!("Tests failed.");
        exit(1);
    }

    println!("Tests completed successfully.");
}
