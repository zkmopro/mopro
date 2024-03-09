use crate::utils::get_mopro_root;
use std::process::{exit, Command};

pub fn install_deps() {
    println!("Installing required dependencies");

    let mopro_root = get_mopro_root();
    let script_name = "install_deps.sh";
    let script_path = format!("{}/scripts/cli/{}", mopro_root, script_name);

    let status = Command::new("sh")
        .arg(script_path)
        .status()
        .expect("Failed to execute build script");

    if !status.success() {
        eprintln!("Unable install dependencies.");
        exit(1);
    }

    println!("Finished installing dependencies.")
}
