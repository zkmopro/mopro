use crate::config::{read_config, write_config, Config};
use crate::init::adapter::ADAPTERS;
use crate::print::print_init_instructions;
use adapter::{Adapter, AdapterSelector};
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use include_dir::{include_dir, Dir};
use std::collections::HashSet;
use std::{env, fs, io::Write, path::Path};

pub mod adapter;
mod circom;
mod halo2;
mod noir;
mod proving_system;
mod write_toml;

pub fn init_project(
    arg_adapter: &Option<String>,
    arg_project_name: &Option<String>,
    quiet: bool,
) -> Result<()> {
    let project_name: String = match arg_project_name.as_deref() {
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project name")
            .with_initial_text("mopro-example-app".to_string())
            .interact_text()?,
        Some(name) => name.to_string(),
    };

    let adapter_sel = match arg_adapter.as_deref() {
        None => AdapterSelector::select(),
        Some(a) => {
            let mut selection = vec![];
            Adapter::all_strings()
                .iter()
                .enumerate()
                .for_each(|(i, adapter)| {
                    if a.contains(adapter) {
                        selection.push(i);
                    }
                });
            AdapterSelector::construct(selection)
        }
    };

    let current_dir = env::current_dir()?;
    let project_dir = current_dir.join(&project_name);
    fs::create_dir(&project_dir)?;

    // Change directory to the project directory
    env::set_current_dir(&project_dir)?;
    fs::write(project_dir.join("Cargo.toml"), write_toml::init_toml())?;
    const TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/init");

    copy_embedded_dir(&TEMPLATE_DIR, &project_dir, &adapter_sel)?;

    if let Some(cargo_toml_path) = project_dir.join("Cargo.toml").to_str() {
        replace_project_name(cargo_toml_path, &project_name)?;
        adapter_sel.dep_template(cargo_toml_path)?;
        adapter_sel.build_dep_template(cargo_toml_path)?;
        adapter_sel.dev_dep_template(cargo_toml_path)?;
    }

    if let Some(build_rs_path) = project_dir.join("build.rs").to_str() {
        adapter_sel.build_template(build_rs_path)?;
    }

    if let Some(lib_rs_path) = project_dir.join("src").join("lib.rs").to_str() {
        adapter_sel.lib_template(lib_rs_path)?;
    }

    if let Some(test_bindings_dir_path) = project_dir.join("tests").join("bindings").to_str() {
        replace_test_bindings_lib_import(test_bindings_dir_path, project_name.as_str())?;
    }

    // Store selection
    let config_path = project_dir.join("Config.toml");

    // Check if the config file exists, if not create a default one
    if !config_path.exists() {
        let default_config = Config::default();
        write_config(&config_path, &default_config)?;
    }
    // Read & Write config for selected adapter
    let mut config = read_config(&config_path)?;
    for adapter in adapter_sel.adapters {
        config
            .target_adapters
            .get_or_insert_with(HashSet::new)
            .insert(adapter.as_str().to_string());
    }
    write_config(&config_path, &config)?;

    // Print out the instructions
    if !quiet {
        // Print out the instructions
        print_init_instructions(project_name);
    }

    Ok(())
}

/// Replace the placeholder import lines in the test bindings files with the actual import statements
/// based on the project name and file type (Kotlin or Swift).
fn replace_test_bindings_lib_import(test_bindings_dir: &str, project_name: &str) -> Result<()> {
    let project_lib_name = project_name.replace('-', "_");

    // We expect a structure of `tests/bindings/<adapter>/*.(kts|swift)`
    for bundle in fs::read_dir(test_bindings_dir)? {
        let bundle = bundle?;
        let bundle_path = bundle.path();

        if !bundle_path.is_dir() {
            continue;
        }

        for file in fs::read_dir(&bundle_path)? {
            let file = file?;
            let file_path = file.path();

            if !file_path.is_file() {
                continue;
            }

            let target = "// GENERATED LIB IMPORT PLACEHOLDER";
            let replacement = match file_path.extension().and_then(|ext| ext.to_str()) {
                Some("kts") => Some(format!("import uniffi.{}.*", project_lib_name)),
                Some("swift") => Some(format!("import {}", project_lib_name)),
                _ => None,
            };

            if let Some(line) = replacement {
                replace_string_in_file(
                    file_path
                        .to_str()
                        .ok_or(anyhow::anyhow!("Invalid test binding file path"))?,
                    target,
                    &line,
                )?;
            }
        }
    }

    Ok(())
}

pub fn copy_embedded_dir(dir: &Dir, output_root: &Path, sel: &AdapterSelector) -> Result<()> {
    for entry in dir.entries() {
        let rel = entry.path();

        // Skip entire subtree/file if it's gated by an adapter not in `sel`
        if should_skip(rel, sel) {
            continue;
        }

        let out = output_root.join(rel);

        match entry.as_file() {
            Some(f) => {
                if let Some(parent) = out.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&out, f.contents())?;
            }
            None => {
                copy_embedded_dir(entry.as_dir().unwrap(), output_root, sel)?;
            }
        }
    }
    Ok(())
}

/// Skip if any adapter name is contained in the file/dir name but not selected
fn should_skip(path: &Path, sel: &AdapterSelector) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|name| {
            ADAPTERS
                .iter()
                .any(|a| name.contains(a.as_str()) && !sel.contains(*a))
        })
}

fn replace_project_name(file_path: &str, project_name: &str) -> Result<()> {
    let target = "MOPRO_TEMPLATE_PROJECT_NAME";
    replace_string_in_file(file_path, target, project_name)
}

pub fn replace_string_in_file(file_path: &str, target: &str, replacement: &str) -> Result<()> {
    // Read the entire content of the file
    let content = fs::read_to_string(file_path)?;

    // Replace the target string with the replacement string
    let modified_content = content.replace(target, replacement);

    // Open the file in write mode, which truncates the file content
    let mut file = fs::File::create(file_path)?;

    // Write the modified content back to the file
    file.write_all(modified_content.as_bytes())?;

    Ok(())
}
