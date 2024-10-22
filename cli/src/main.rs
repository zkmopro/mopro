use clap::{Parser, Subcommand};

mod build;
mod create;
mod init;
mod style;

/// CLI for creating a mopro project.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init {
        #[arg(
            long,
            help = "Specify the adapter to use (e.g., 'circom', 'halo2' or 'circom,halo2')."
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
    Create {
        #[arg(long, help = "Specify the platform")]
        mode: Option<String>,
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
            Err(e) => style::print_read_bold(format!("Failed to initialize project {:?}", e)),
        },
        Commands::Build { mode, platforms } => match build::build_project(mode, platforms) {
            Ok(_) => {}
            Err(e) => style::print_read_bold(format!("Failed to build project {:?}", e)),
        },
        Commands::Create { mode } => match create::create_project(mode) {
            Ok(_) => {}
            Err(e) => style::print_read_bold(format!("Failed to build project {:?}", e)),
        },
    }
}
