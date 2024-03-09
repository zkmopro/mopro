use crate::utils::get_mopro_root;
use std::process::{exit, Command};

pub fn prepare_circuit(config: &str) {
    println!("Preparing circuit with config {}", config);

    let mopro_root = get_mopro_root();
    let script_name = "prepare.sh";
    let script_path = format!("{}/scripts/cli/{}", mopro_root, script_name);

    let status = Command::new("sh")
        .arg(script_path)
        .arg(config)
        .status()
        .expect("Failed to execute build script");

    if !status.success() {
        eprintln!("Unable to prepare circuit.");
        exit(1);
    }

    println!("Preparation of circuit completed successfully.");
}
