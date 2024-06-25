use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;

use serde::Deserialize;

use rust_witness::transpile::transpile_wasm;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Deserialize)]
struct Config {
    circuit: CircuitConfig,
    #[serde(skip)]
    expanded_circuit_dir: Option<String>,
}

#[derive(Deserialize)]
struct CircuitConfig {
    dir: String,
    name: String,
}

/// Resolve a potentially relative path against a base directory.
fn resolve_path(base_dir: &Path, relative_path: &str) -> String {
    let path = Path::new(relative_path);
    if path.is_absolute() {
        relative_path.to_owned()
    } else {
        base_dir.join(path).to_string_lossy().into_owned()
    }
}

fn read_config() -> Result<Config> {
    let config_str = match env::var("BUILD_CONFIG_PATH") {
        Ok(config_path) => {
            println!("cargo:rerun-if-changed={}", config_path);
            println!("cargo:warning=BUILD_CONFIG_PATH={}", config_path);
            let config_path = PathBuf::from(config_path);

            // Ensure the config path is absolute or resolve it based on current dir
            let config_path = if config_path.is_absolute() {
                config_path
            } else {
                env::current_dir()?.join(config_path)
            };

            // Read the configuration file
            fs::read_to_string(config_path)?
        }
        Err(_) => {
            println!("cargo:warning=BUILD_CONFIG_PATH not set. Using default configuration.");
            #[cfg(not(feature = "halo2"))]
            let default_config = r#"
                [circuit]
                adapter = "circom"
                dir = "examples/circom/keccak256"
                name = "keccak256_256_test"

                [dylib]
                use_dylib = false
                name = "keccak256.dylib"
            "#;

            #[cfg(feature = "halo2")] // TODO - change sample to fibonacci
            let default_config = r#"
                [circuit]
                adapter = "halo2"
                dir = "examples/halo2/fibonacci"
                name = "fibonacci"

                [dylib]
                use_dylib = false
                name = "keccak256.dylib"
            "#;
            default_config.to_string()
        }
    };

    let mut config: Config = toml::from_str(&config_str)?;

    // Resolve paths relative to the config file or default path
    let config_dir = PathBuf::from(env::var("BUILD_CONFIG_PATH").unwrap_or_else(|_| ".".into()));
    let config_dir = config_dir.parent().unwrap_or_else(|| Path::new("."));

    let resolved_circuit_dir = resolve_path(config_dir, &config.circuit.dir);

    config.expanded_circuit_dir = Some(resolved_circuit_dir.clone());

    Ok(config)
}

fn get_circuit_dir_path(config: &Config) -> PathBuf {
    // Check if the current package is mopro-core
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap_or_default();
    let base_dir = if pkg_name == "mopro-core" {
        // If mopro-core, use CARGO_MANIFEST_DIR as base directory
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set")
    } else {
        // Default to current directory
        ".".to_string()
    };

    // Use the expanded circuit directory if available, otherwise fallback to the original directory
    let circuit_dir = config
        .expanded_circuit_dir
        .as_ref()
        .unwrap_or(&config.circuit.dir);

    // Resolve the circuit dictory to an absolute path based on the conditionally set base_dir
    PathBuf::from(base_dir).join(circuit_dir)
}

#[cfg(feature = "halo2")]
mod halo2 {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use toml::Value;

    pub(crate) fn build_circuit(circuit_dir_path: &PathBuf, circuit_name: &String) -> Result<()> {
        // Resolve the circuit directory to an absolute path based on the conditionally set base_dir
        let circuit_key_path = circuit_dir_path.join("out");

        let srs_path = circuit_key_path.join(format!("{}_srs", circuit_name));
        let pk_path = circuit_key_path.join(format!("{}_pk", circuit_name));
        let vk_path = circuit_key_path.join(format!("{}_vk", circuit_name));

        if !srs_path.exists() || !pk_path.exists() || !vk_path.exists() {
            let missing_files = [&srs_path, &pk_path, &vk_path]
                .iter()
                .filter(|path| !path.exists())
                .map(|path| format!(" - {}", path.display()))
                .collect::<Vec<_>>()
                .join("\n");

            return Err(eyre!(format!(
        "Required files for building the Halo2 circuit are missing. Ensure you've run `mopro prepare` or generated the files yourself. \
        \nMissing files:\n{}",
            missing_files
    )));
        }

        println!("cargo:rustc-env=BUILD_SRS_FILE={}", srs_path.display());
        println!("cargo:rustc-env=BUILD_PK_FILE={}", pk_path.display());
        println!("cargo:rustc-env=BUILD_VK_FILE={}", vk_path.display());

        println!("cargo:warning=BUILD_SRS_FILE={}", srs_path.display());
        println!("cargo:warning=BUILD_PK_FILE={}", pk_path.display());
        println!("cargo:warning=BUILD_VK_FILE={}", vk_path.display());

        Ok(())
    }
}

fn main() -> color_eyre::eyre::Result<()> {
    println!("cargo:rerun-if-env-changed=BUILD_CONFIG_PATH");
    println!("cargo:warning=Preparing circuits...");

    let config = read_config()?;

    // Resolve the circuit dictory to an absolute path based on the conditionally set base_dir
    let circuit_dir_path = get_circuit_dir_path(&config);
    let circuit_name = &config.circuit.name;

    if cfg!(all(feature = "halo2", feature = "circom")) {
        println!("cargo:error=Both Halo2 and Circom features are enabled. Please enable only one of them.");
        return Err(eyre!(
            "Both Halo2 and Circom features were enabled. Please enable only one of them.\n \
             By default, `circom` is enabled. You need to turn off the default features to run Halo2."
        ));
    }

    #[cfg(feature = "halo2")]
    {
        // If Halo2 feature is enabled, build Halo2 Circuit
        println!("cargo:warning=Building Halo2 circuit...");

        halo2::build_circuit(&circuit_dir_path, circuit_name)?;
    }

    #[cfg(feature = "circom")]
    {
        // If Circom feature is enabled, build Circom Circuit
        println!("cargo:warning=Building Circom circuit...");

        // see here: https://github.com/zkmopro/rust-witness/?tab=readme-ov-file#rust-witness
        transpile_wasm(String::from("./examples"));
    }

    println!("cargo:warning=Successfully prepared circuits.");
    Ok(())
}
