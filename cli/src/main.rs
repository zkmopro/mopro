use clap::{Parser, Subcommand};

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
    }
}
