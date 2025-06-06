use clap::Parser;
use clap::Subcommand;

mod build;
mod config;
mod constants;
mod create;
mod init;
mod print;
mod select;
mod style;
mod utils;

/// CLI for creating a mopro project.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize the project for specified adapters
    Init {
        #[arg(
            long,
            help = "Specify the adapter to use (e.g., 'circom', 'halo2', 'noir', or 'circom,halo2')."
        )]
        adapter: Option<String>,
        #[arg(long)]
        project_name: Option<String>,
    },
    /// Builds the project for specified platforms
    Build {
        #[arg(long, help = "Specify the build mode (e.g., 'release' or 'debug').")]
        mode: Option<String>,
        #[arg(long, num_args = 1.., help = "Specify the platforms to build for (e.g., 'ios', 'android').")]
        platforms: Option<Vec<String>>,
    },
    /// Create templates for the specified platform
    Create {
        #[arg(long, help = "Specify the platform")]
        framework: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            adapter,
            project_name,
        } => match init::init_project(adapter, project_name) {
            Ok(_) => {}
            Err(e) => style::print_red_bold(format!("Failed to initialize project: {:?}", e)),
        },
        Commands::Build { mode, platforms } => match build::build_project(mode, platforms) {
            Ok(_) => {}
            Err(e) => style::print_red_bold(format!("Failed to build project: {:?}", e)),
        },
        Commands::Create { framework } => match create::create_project(framework) {
            Ok(_) => {}
            Err(e) => style::print_red_bold(format!("Failed to create template: {:?}", e)),
        },
    }
}
