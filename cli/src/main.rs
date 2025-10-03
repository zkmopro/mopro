use clap::ArgAction;
use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

use crate::constants::Framework;
use crate::create::{Android, Create, Flutter, Ios, ReactNative, Web};

mod bindgen;
mod build;
mod config;
mod constants;
mod create;
mod init;
mod print;
mod select;
mod style;
mod update;

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
        #[arg(
            long,
            help = "Automatically run mopro update after build",
            conflicts_with = "no_auto_update"
        )]
        auto_update: bool,
        #[arg(
            long,
            help = "Skip running mopro update and disable the prompt",
            conflicts_with = "auto_update"
        )]
        no_auto_update: bool,
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
    Construct {
        #[arg(
            long,
            help = "Specify the adapter to use (e.g., 'circom', 'halo2', 'noir', or 'circom,halo2')."
        )]
        adapter: Option<String>,
        #[arg(long)]
        project_name: Option<String>,
        #[arg(long, help = "Show instruction message for create")]
        show_init: bool,
        #[arg(long, help = "Specify the build mode (e.g., 'release' or 'debug').")]
        mode: Option<String>,
        #[arg(long, num_args = 1.., help = "Specify the platforms to build for (e.g., 'ios', 'android').")]
        platforms: Option<Vec<String>>,
        #[arg(long, num_args = 1.., help = "Specify the architectures to build for (e.g., 'aarch64-apple-ios', 'aarch64-apple-ios-sim', x86_64-apple-ios, x86_64-linux-android, i686-linux-android, armv7-linux-androideabi, aarch64-linux-android).")]
        architectures: Option<Vec<String>>,
        #[arg(
            long,
            help = "Automatically run mopro update after build",
            conflicts_with = "no_auto_update"
        )]
        auto_update: bool,
        #[arg(
            long,
            help = "Skip running mopro update and disable the prompt",
            conflicts_with = "auto_update"
        )]
        no_auto_update: bool,
        #[arg(long, help = "Specify the framework")]
        framework: Option<String>,
        #[arg(
            long,
            value_name = "FRAMEWORK",
            help = "Show instruction message for create (e.g., 'ios', 'android', 'web', 'flutter', 'react-native')."
        )]
        show_create: Option<String>,
    },
    /// Update the bindings for all platforms
    Update {
        #[arg(
            long,
            help = "Optional path to the bindings directory (defaults to CWD)"
        )]
        src: Option<String>,
        #[arg(
            long,
            help = "Optional path to the mobile project (recurses if omitted)"
        )]
        dest: Option<String>,
        #[arg(long, action = ArgAction::SetTrue, help = "Suppress interactive prompts")]
        no_prompt: bool,
    },
    /// Generate the bindings for the specified platform (NOTE: it only supports circom with rust-witness and arkworks now)
    Bindgen {
        #[arg(long, help = "Specify the build mode (e.g., 'release' or 'debug').")]
        mode: Option<String>,
        #[arg(long, num_args = 1.., help = "Specify the platforms to build for (e.g., 'ios', 'android').")]
        platforms: Option<Vec<String>>,
        #[arg(long, num_args = 1.., help = "Specify the architectures to build for (e.g., 'aarch64-apple-ios', 'aarch64-apple-ios-sim', x86_64-apple-ios, x86_64-linux-android, i686-linux-android, armv7-linux-androideabi, aarch64-linux-android).")]
        architectures: Option<Vec<String>>,
        #[arg(
            long,
            help = "path to the circuit directory (it should contain `.wtns` and `.zkey` files)"
        )]
        circuit_dir: Option<String>,
        #[arg(
            long,
            help = "Specify the witness generator adapter (e.g., 'rust-witness' or 'witnesscalc')."
        )]
        adapter: Option<String>,
        #[arg(long, help = "Specify the output directory for bindings.")]
        output_dir: Option<String>,
        #[arg(long, help = "Show instruction message for build")]
        show: bool,
    },
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
            match init::init_project(adapter, project_name, false) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to initialize project: {e:?}")),
            }
        }
        Commands::Build {
            mode,
            platforms,
            architectures,
            auto_update,
            no_auto_update,
            show,
        } => {
            if *show {
                print::print_build_success_message();
                return;
            }
            let auto_update_flag = if *auto_update {
                Some(true)
            } else if *no_auto_update {
                Some(false)
            } else {
                None
            };
            match build::build_project(mode, platforms, architectures, auto_update_flag, false) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to build project: {e:?}")),
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

            // Handles platform-aware creation (React Native + Flutter)
            match create::create_project(framework) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to create template: {e:?}")),
            }
        }
        Commands::Construct {
            adapter,
            project_name,
            show_init,
            mode,
            platforms,
            architectures,
            auto_update,
            no_auto_update,
            framework,
            show_create,
        } => {
            if *show_init {
                print::print_init_instructions("<PROJECT_NAME>".to_string());
                return;
            }
            match init::init_project(adapter, project_name, false) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to initialize project: {e:?}")),
            }
            let auto_update_flag = if *auto_update {
                Some(true)
            } else if *no_auto_update {
                Some(false)
            } else {
                None
            };
            match build::build_project(mode, platforms, architectures, auto_update_flag, false) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to build project: {e:?}")),
            }
            if let Some(framework) = show_create {
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

            // Handles platform-aware creation (React Native + Flutter)
            match create::create_project(framework) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to create template: {e:?}")),
            }
        }

        Commands::Update {
            src,
            dest,
            no_prompt,
        } => match update::update_bindings(src, dest, *no_prompt) {
            Ok(_) => {}
            Err(e) => style::print_red_bold(format!("Failed to update bindings: {e:?}")),
        },

        Commands::Bindgen {
            mode,
            platforms,
            architectures,
            circuit_dir,
            show,
            adapter,
            output_dir,
        } => {
            if *show {
                // TODO: print bindgen instructions
                return;
            }
            match bindgen::bindgen(
                mode,
                platforms,
                architectures,
                circuit_dir,
                adapter,
                output_dir,
            ) {
                Ok(_) => {}
                Err(e) => style::print_red_bold(format!("Failed to generate bindings: {e:?}")),
            }
        }
    }
}
