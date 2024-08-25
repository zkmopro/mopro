use dialoguer::{theme::ColorfulTheme, Input};
use include_dir::{include_dir, Dir};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn init_project(adapter: &str, platforms: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .with_initial_text("mopro-example-app".to_string())
        .interact_text()?;

    println!(
        "Initializing project for platforms {:?}: {} with name {}",
        platforms, adapter, project_name
    );

    let current_dir = env::current_dir()?;

    let project_dir = current_dir.join(&project_name);
    fs::create_dir(&project_dir)?;

    // Change directory to the project directory
    env::set_current_dir(&project_dir)?;
    const TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/init");

    copy_embedded_dir(&TEMPLATE_DIR, &project_dir).expect("Failed to copy embedded directory");

    // Print out the instructions
    println!("Project '{}' initialized successfully.", &project_name);
    println!("\x1b[1mcd {}\x1b[0m", &project_name);
    println!("\x1b[1mcargo run --bin ios\x1b[0m");
    println!("\x1b[1mcargo run --bin android\x1b[0m");
    Ok(())
}

fn copy_embedded_dir(dir: &Dir, output_dir: &Path) -> std::io::Result<()> {
    for file in dir.entries() {
        let relative_path = file.path();
        let output_path = output_dir.join(relative_path);

        // Create directories as needed
        if let Some(parent) = output_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!(
                    "\x1b[1;31mFailed to create directory {:?}: {}\x1b[0m",
                    parent, e
                );
                return Err(e);
            }
        }

        // Write the file to the output directory
        match file.as_file() {
            Some(file) => {
                if let Err(e) = fs::write(&output_path, file.contents()) {
                    eprintln!(
                        "\x1b[1;31mFailed to write file {:?}: {}\x1b[0m",
                        output_path, e
                    );
                    return Err(e);
                }
            }
            None => {
                if let Err(e) = copy_embedded_dir(file.as_dir().unwrap(), &output_dir) {
                    eprintln!("Failed to write file {:?}: {}", output_path, e);
                    return Err(e);
                };
            }
        }
    }
    Ok(())
}
