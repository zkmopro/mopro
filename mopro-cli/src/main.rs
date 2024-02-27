use clap::{Parser, Subcommand};

mod init;

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
        adapter: String,
        #[arg(long)]
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
        Commands::Build { adapter, platform } => {
            println!("Building project for platform {}: {}", platform, adapter);
            // Implement build logic here
            println!("Not yet implemented")
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
