use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

use crate::constants::Framework;
use crate::create::{Android, Create, Flutter, Ios, ReactNative, Web};

mod build;
mod config;
mod constants;
mod create;
mod init;
mod print;
mod select;
mod style;
mod update;
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
        #[arg(long, help = "Show instruction message for init")]
        show: bool,
    },
    /// Builds the project for specified platforms
    Build {
        #[arg(long, help = "Specify the build mode (e.g., 'release' or 'debug').")]
        mode: Option<String>,
        #[arg(long, num_args = 1.., help = "Specify the platforms to build for (e.g., 'ios', 'android').")]
        platforms: Option<Vec<String>>,
        #[arg(long, num_args = 1.., help = "Specify the architectures to build for (e.g., 'aarch64-apple-ios', 'aarch64-apple-ios-sim', x86_64-apple-ios, x86_64-linux-android, i686-linux-android, armv7-linux-androideabi, aarch64-linux-android).")]
        architectures: Option<Vec<String>>,
        #[arg(long, help = "Show instruction message for build")]
        show: bool,
    },
    /// Create templates for the specified platform
    Create {
        #[arg(long, help = "Specify the framework")]
        framework: Option<String>,
        #[arg(
            long,
            value_name = "FRAMEWORK",
            help = "Show instruction message for create (e.g., 'ios', 'android', 'web', 'flutter', 'react-native')."
        )]
        show: Option<String>,
    },
    /// Update the bindings for the all platforms
    Update {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            adapter,
            project_name,
            show,
        } => {
            if *show {
                print::print_init_instructions("<PROJECT NAME>".to_string());
                return;
            }
            match init::init_project(adapter, project_name) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to initialize project: {:?}", e)),
            }
        }
        Commands::Build {
            mode,
            platforms,
            architectures,
            show,
        } => {
            if *show {
                print::print_build_success_message();
                return;
            }
            match build::build_project(mode, platforms, architectures) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to build project: {:?}", e)),
            }
        }
        Commands::Create { framework, show } => {
            if let Some(framework) = show {
                if framework.trim().is_empty() {
                    Cli::command()
                        .find_subcommand_mut("create")
                        .unwrap()
                        .print_help()
                        .unwrap();
                    println!();
                    return;
                }

                match Framework::parse_from_str(framework) {
                    Framework::Ios => <Ios as Create>::print_message(),
                    Framework::Android => <Android as Create>::print_message(),
                    Framework::Web => <Web as Create>::print_message(),
                    Framework::Flutter => <Flutter as Create>::print_message(),
                    Framework::ReactNative => <ReactNative as Create>::print_message(),
                }
                println!();
                return;
            }

            match create::create_project(framework) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to create template: {:?}", e)),
            }
        }
        Commands::Update {} => match update::update_bindings() {
            Ok(_) => {}
            Err(e) => style::print_red_bold(format!("Failed to update bindings: {:?}", e)),
        },
    }
}
