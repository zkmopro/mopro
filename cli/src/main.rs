use clap::{Parser, Subcommand};

mod build;
// mod deps;
// mod export;
mod init;
// mod prepare;
// mod test;
// mod update;
// mod utils;

/// CLI for multi-platform project management
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // /// Install required dependencies
    // Deps {},
    /// Initializes a new project
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
    // // TODO: Update this when it does something useful over just `build`
    // // Updates bindings for the specified platforms
    // // Update {
    // //     #[arg(long, default_value = "mopro-config.toml")]
    // //     config: String,
    // //     #[arg(long, default_value = "circom")]
    // //     adapter: String,
    // //     #[arg(long, num_args = 1.., default_value = "core")]
    // //     platforms: Vec<String>,
    // // },
    // /// Runs tests for the specified platform and test cases
    // Test {
    //     #[arg(long, default_value = "mopro-config.toml")]
    //     config: String,
    //     #[arg(long, default_value = "circom")]
    //     adapter: String,
    //     #[arg(long, num_args = 1.., default_value = "core")]
    //     platforms: Vec<String>,
    //     #[arg(long)]
    //     test_case: Option<String>,
    // },
    // /// Exports platform bindings to some other directory
    // ExportBindings {
    //     #[arg(long, num_args = 1.., default_value = "ios")]
    //     platforms: Vec<String>,
    //     #[arg(short, long)]
    //     destination: PathBuf,
    // },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // Commands::Deps {} => deps::install_deps(),
        Commands::Init {
            adapter,
            project_name,
        } => match init::init_project(adapter, project_name) {
            Ok(_) => {}
            Err(e) => println!("\x1b[1;31mFailed to initialize project {:?}\x1b[0m", e),
        },
        Commands::Build { mode, platforms } => match build::build_project(mode, platforms) {
            Ok(_) => {}
            Err(e) => println!("\x1b[1;31mFailed to build project {:?}\x1b[0m", e),
        },
        // // Commands::Update {
        // //     config,
        // //     adapter,
        // //     platforms,
        // // } => update::update_project(config, adapter, platforms),
        // Commands::Test {
        //     config,
        //     adapter,
        //     platforms,
        //     test_case,
        // } => test::test_project(config, adapter, platforms, test_case),
        // Commands::ExportBindings {
        //     platforms,
        //     destination,
        // } => export::export_bindings(platforms, destination),
    }
}
