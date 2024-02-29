use clap::{Parser, Subcommand};
use std::process::Command;
mod init;
use fs_extra::dir::{self, CopyOptions};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

/// CLI for multi-platform project management
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initializes a new project
    Init {
        #[arg(long, default_value = "circom")]
        adapter: String,
        #[arg(long, default_value = "desktop")]
        platform: String,
        #[arg(default_value = "mopro-cli-example")]
        project_name: String,
    },
    /// Builds the project for specified platforms
    Build {
        #[arg(long)]
        config: String,
        #[arg(long, default_value = "circom")]
        adapter: String,
        #[arg(long, default_value = "desktop")]
        platform: String,
    },
    /// Updates bindings for the specified platforms
    Update {
        #[arg(long)]
        adapter: String,
        #[arg(long)]
        platform: String,
    },
    /// Runs tests for the specified platform and test cases
    Test {
        #[arg(long, default_value = "circom")]
        adapter: String,
        #[arg(long, default_value = "desktop")]
        platform: String,
        #[arg(long)]
        test_case: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            adapter,
            platform,
            project_name,
        } => {
            println!(
                "Init project for platform {}: {} and name {}",
                platform, adapter, project_name
            );

            let mopro_root =
                env::var("MOPRO_ROOT").expect("MOPRO_ROOT environment variable is not set");
            let source_path = PathBuf::from(mopro_root).join("mopro-cli-example");
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let destination_path = current_dir.join(project_name);

            // Create the project directory
            if destination_path.exists() {
                eprintln!(
                    "A directory with the name '{}' already exists.",
                    project_name
                );
                std::process::exit(1);
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
                    dir::copy(entry, &destination_path, &options)
                        .expect("Failed to copy directory");
                } else {
                    // Copy file
                    let file_name = entry.file_name().unwrap();
                    fs::copy(&entry, destination_path.join(file_name))
                        .expect("Failed to copy file");
                }
            }

            println!("Project initialized successfully.");
        }
        Commands::Build {
            config,
            adapter,
            platform,
        } => {
            println!(
                "Building project for platform {}: {} with config {}",
                platform, adapter, config
            );

            // Attempt to retrieve the MOPRO_ROOT environment variable
            match env::var("MOPRO_ROOT") {
                Ok(mopro_root) => {
                    // Construct the path to the build script using the MOPRO_ROOT environment variable
                    let script_path = format!("{}/scripts/build_minimal_project.sh", mopro_root);

                    // NOTE: When we feel more confident in this we can turn down the verbosity
                    // For example by capturing output and only printing it if the build fails
                    //
                    // Execute the shell script with the provided config file as an argument
                    let status = Command::new("sh")
                        .arg(script_path)
                        .arg(&config) // Pass the config file path as an argument to the script
                        .status()
                        //.output() // NOTE: This captures standard output
                        .expect("Failed to execute build script");

                    if status.success() {
                        println!("Cargo build completed successfully.");
                    } else {
                        eprintln!("Cargo build failed.");
                        process::exit(1);
                    }
                }
                Err(_) => {
                    eprintln!("Error: MOPRO_ROOT environment variable is not set.");
                    eprintln!("Please set MOPRO_ROOT to point to the local checkout of mopro.");
                    eprintln!(
                        "For example: export MOPRO_ROOT=/Users/user/repos/github.com/oskarth/mopro"
                    );
                    eprintln!("Git repository: https://github.com/oskarth/mopro");
                    process::exit(1);
                }
            }
        }
        Commands::Update { adapter, platform } => {
            println!("Updating project for platform {}: {}", platform, adapter);
            // Implement update logic here
            println!("Not yet implemented")
        }
        Commands::Test {
            adapter,
            platform,
            test_case,
        } => {
            println!(
                "Testing project on platform {} with adapter {}",
                platform, adapter
            );

            // Start building the command
            let mut command = Command::new("cargo");
            command.arg("test");

            // If a test case is provided, pass it to `cargo test`
            if let Some(case) = test_case {
                command.arg(case);
            }

            // Execute `cargo test`, allowing output to be printed directly to the terminal
            let status = command.status().expect("Failed to execute cargo test");

            if status.success() {
                println!("Tests completed successfully.");
            } else {
                eprintln!("Tests failed.");
                std::process::exit(1);
            }
        }
    }
}
