use clap::{Parser, Subcommand};
mod build;
mod init;
mod test;

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
        #[arg(long, default_value = "mopro-config.toml")]
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
        #[arg(long, default_value = "mopro-config.toml")]
        config: String,
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
        } => init::init_project(adapter, platform, project_name),
        Commands::Build {
            config,
            adapter,
            platform,
        } => build::build_project(config, adapter, platform),
        Commands::Update { adapter, platform } => {
            println!("Updating project for platform {}: {}", platform, adapter);
            // Implement update logic here
            println!("Not yet implemented")
        }
        Commands::Test {
            config,
            adapter,
            platform,
            test_case,
        } => test::test_project(config, adapter, platform, test_case),
    }
}
