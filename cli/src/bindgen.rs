use std::{env, ffi::OsStr, fs, path::Path};

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use mopro_ffi::app_config::constants::{ANDROID_BINDINGS_DIR, IOS_BINDINGS_DIR};
use walkdir::WalkDir;

use crate::{
    build::build_project,
    init::{adapter::Adapter, init_project},
};

pub fn bindgen(
    arg_mode: &Option<String>,
    arg_platforms: &Option<Vec<String>>,
    arg_architectures: &Option<Vec<String>>,
    circuit_dir: &Option<String>,
) -> Result<()> {
    // Currently only support circom
    let adapter = Adapter::Circom;

    // Find the circuit name

    let mut specified_circuit_dir = String::new();
    if let Some(circuit_dir) = circuit_dir {
        specified_circuit_dir = circuit_dir.to_string();
    } else {
        specified_circuit_dir = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Circuit folder path")
            .with_initial_text("./circuit".to_string())
            .interact_text()?;
    }

    // Convert relative path to absolute path
    let current_dir = env::current_dir()?;
    let absolute_circuit_dir = if Path::new(&specified_circuit_dir).is_absolute() {
        Path::new(&specified_circuit_dir).to_path_buf()
    } else {
        current_dir.join(&specified_circuit_dir)
    };

    // Verify the directory exists
    if !absolute_circuit_dir.exists() {
        return Err(anyhow::anyhow!(
            "Circuit directory does not exist: {:?}",
            absolute_circuit_dir
        ));
    }

    if !absolute_circuit_dir.is_dir() {
        return Err(anyhow::anyhow!(
            "Circuit path is not a directory: {:?}",
            absolute_circuit_dir
        ));
    }

    let mut project_name = String::new();

    for entry in WalkDir::new(&absolute_circuit_dir) {
        let e = entry.unwrap();
        let path = e.path();
        if path.is_dir() {
            continue;
        }
        let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");
        // Iterate over all wasm files and generate c source, then compile each source to
        // a static library that can be called from rust
        if ext != "wasm" {
            continue;
        }
        // make source files with the same name as the wasm binary file
        let circuit_name = path.file_stem().unwrap();
        let circuit_name_compressed = circuit_name
            .to_str()
            .unwrap()
            .replace("_", "")
            .replace("-", "");
        project_name = circuit_name_compressed;
        // TODO: find zkey and wasm mapping
    }

    init_project(
        &Some(adapter.as_str().to_string()),
        &Some(project_name.to_string()),
    )?;

    let project_dir = env::current_dir()?;
    let test_vectors_dir = project_dir.join("test-vectors").join("circom");
    fs::create_dir_all(&test_vectors_dir)?;

    // Copy the entire directory recursively
    // Remove the destination directory if it exists to avoid conflicts
    if test_vectors_dir.exists() {
        fs::remove_dir_all(&test_vectors_dir)?;
    }
    // Copy the entire directory
    fs_extra::dir::copy(
        &absolute_circuit_dir,
        test_vectors_dir.parent().unwrap(),
        &fs_extra::dir::CopyOptions::new(),
    )?;

    // TODO: Update rust_witness functions

    // Run the build command
    build_project(arg_mode, arg_platforms, arg_architectures)?;

    // Copy the bindings folder to the project root
    let ios_bindings_dir = project_dir.join(IOS_BINDINGS_DIR);
    if ios_bindings_dir.exists() {
        let output_ios_bindings_dir = current_dir.join(IOS_BINDINGS_DIR);
        fs::create_dir_all(&output_ios_bindings_dir)?;
        fs_extra::dir::copy(
            &ios_bindings_dir,
            &output_ios_bindings_dir,
            &fs_extra::dir::CopyOptions::new(),
        )?;
    }

    let android_bindings_dir = project_dir.join(ANDROID_BINDINGS_DIR);
    if android_bindings_dir.exists() {
        let output_android_bindings_dir = current_dir.join(ANDROID_BINDINGS_DIR);
        fs::create_dir_all(&output_android_bindings_dir)?;
        fs_extra::dir::copy(
            &android_bindings_dir,
            &output_android_bindings_dir,
            &fs_extra::dir::CopyOptions::new(),
        )?;
    }

    fs::remove_dir_all(&project_dir)?;

    Ok(())
}
