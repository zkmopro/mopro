use camino::Utf8Path;
use uniffi_bindgen::bindings::{KotlinBindingGenerator, SwiftBindingGenerator};
use uniffi_bindgen::library_mode::generate_bindings;

use crate::app_config::utils::build_release;

mod android;
mod ios;
mod utils;

use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Android,
    Ios,
}

#[derive(Debug, thiserror::Error)]
pub enum MoproBuildError {
    #[error("Failed to build the release library: {0}")]
    LibraryBuildError(String),
    #[error("Failed to generate bindings: {0}")]
    GenerateBindingsError(String),
}

pub fn build(target: Target) -> Result<(), MoproBuildError> {
    // Set up the directories for the bindings
    let cwd = std::env::current_dir().unwrap();
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(cwd.to_str().unwrap().to_string());

    // Library name is the name of the crate with all `-` replaced with `_`
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let library_name = crate_name.replace("-", "_");

    let build_dir = format!("{}/build", manifest_dir);

    let bindings_dir = format!("{}/out", build_dir);
    let library_path = format!("{}/debug/lib{}.dylib", build_dir, library_name);

    // Build the crate as a release library for the bindgen
    build_cdylib(&build_dir).map_err(|e| MoproBuildError::LibraryBuildError(e.to_string()))?;

    // Generate the bindings for IOS
    match target {
        Target::Ios => {
            generate_bindings(
                Utf8Path::new(&library_path),
                None,
                &SwiftBindingGenerator,
                None,
                Utf8Path::new(&bindings_dir),
                true,
            )
            .map_err(|e| MoproBuildError::GenerateBindingsError(e.to_string()))?;

            ios::build(&library_name, &manifest_dir, &bindings_dir); // TODO - add error handling
        }
        Target::Android => {
            generate_bindings(
                Utf8Path::new(&library_path),
                None,
                &KotlinBindingGenerator,
                None,
                Utf8Path::new(&bindings_dir),
                true,
            )
            .map_err(|e| MoproBuildError::GenerateBindingsError(e.to_string()))?;

            android::build(); // TODO - rewrite this to make it work
        }
    };

    Ok(())
}
