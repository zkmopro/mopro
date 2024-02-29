use clap::{Parser, Subcommand};
use std::process::Command;
mod init;
use std::env;
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
        #[arg(long)]
        adapter: String,
        #[arg(long)]
        platform: String,
        #[arg(long)]
        test_case: String,
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
                "Initializing project: {}, {}, {}",
                adapter, platform, project_name
            );
            // Implement initialization logic here
            // If adapter is 'circom', platform is 'desktop', then do something
            if adapter == "circom" && platform == "desktop" {
                println!("Initializing circom project for desktop");
                init::create_project_structure(project_name);
                println!("Project {} created successfully", project_name);
            } else {
                println!("Not yet implemented")
            }
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
                "Testing project on platform {} with test {}: {}",
                platform, test_case, adapter
            );
            // Implement test logic here
            println!("Not yet implemented")
        }
    }
}
