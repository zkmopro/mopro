use std::process::{exit, Command};

pub fn test_project(adapter: &str, platform: &str, test_case: &Option<String>) {
    println!(
        "Testing project on platform {} with adapter {}",
        platform, adapter
    );

    let mut command = Command::new("cargo");
    command.arg("test");

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
