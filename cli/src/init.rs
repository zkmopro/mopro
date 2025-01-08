use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::constants::Adapter;
use crate::constants::ADAPTERS;
use crate::print::print_init_instructions;
use crate::utils::contains_circom;
use crate::utils::contains_halo2;
use crate::utils::AdapterSelector;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use include_dir::include_dir;
use include_dir::Dir;

use crate::config::read_config;
use crate::config::write_config;
use crate::config::Config;
use crate::print::print_init_instructions;
use crate::style;
use crate::style::create_custom_theme;

mod circom;
use circom::Circom;
mod halo2;
use halo2::Halo2;

trait ProvingSystem {
    fn dep_template(file_path: &str) -> Result<()>;
    fn lib_template(file_path: &str) -> Result<()>;
    fn build_template(file_path: &str) -> Result<()>;
}

pub fn init_project(
    arg_adapter: &Option<String>,
    arg_project_name: &Option<String>,
) -> anyhow::Result<()> {
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
            for (i, adapter) in ADAPTERS.iter().enumerate() {
                if a.contains(adapter) {
                    selection.push(i);
                }
            }
            AdapterSelector::construct(selection)
        }
    };

    let current_dir = env::current_dir()?;
    let project_dir = current_dir.join(&project_name);
    fs::create_dir(&project_dir)?;

    // Change directory to the project directory
    env::set_current_dir(&project_dir)?;
    const TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/init");

    let selections = adapter_sel.selections();
    copy_embedded_dir(&TEMPLATE_DIR, &project_dir, &adapter_sel)?;

    if let Some(cargo_toml_path) = project_dir.join("Cargo.toml").to_str() {
        replace_project_name(cargo_toml_path, &project_name)?;
        replace_features(
            cargo_toml_path,
            selections.iter().map(|&i| ADAPTERS[i]).collect(),
        )?;
        if adapter_sel.contains(Adapter::Circom) {
            Circom::dep_template(cargo_toml_path)?;
        }
        if adapter_sel.contains(Adapter::Halo2) {
            Halo2::dep_template(cargo_toml_path)?;
        }
    }

    if let Some(build_rs_path) = project_dir.join("build.rs").to_str() {
        if adapter_sel.contains(Adapter::Circom) {
            Circom::build_template(build_rs_path)?;
        }
    }

    if let Some(lib_rs_path) = project_dir.join("src").join("lib.rs").to_str() {
        if adapter_sel.contains(Adapter::Circom) {
            Circom::lib_template(lib_rs_path)?;
        }
        if adapter_sel.contains(Adapter::Halo2) {
            Halo2::lib_template(lib_rs_path)?;
        }
    }

    // Store selection
    let config_path = project_dir.join("Config.toml");

    // Check if the config file exists, if not create a default one
    if !config_path.exists() {
        let default_config = Config {
            target_adapters: HashSet::new(),
            target_platforms: HashSet::new(),
        };
        write_config(&config_path, &default_config)?;
    }
    // Read & Write config for selected adapter
    let mut config = read_config(&config_path)?;
    for adapter_idx in selection {
        config
            .target_adapters
            .insert(adapters[adapter_idx].to_owned());
    }
    write_config(&config_path, &config)?;

    // Print out the instructions
    print_init_instructions(project_name);

    Ok(())
}

pub fn copy_embedded_dir(
    dir: &Dir,
    output_dir: &Path,
    adapter_sel: &AdapterSelector,
) -> Result<()> {
    for file in dir.entries() {
        let relative_path = file.path();
        let output_path = output_dir.join(relative_path);

        // Create directories as needed
        if let Some(parent) = output_path.parent() {
            if let Some(path_str) = parent.to_str() {
                if contains_circom(path_str) && !adapter_sel.contains(Adapter::Circom) {
                    return Ok(());
                }
            }
            if let Some(path_str) = parent.to_str() {
                if contains_halo2(path_str) && !adapter_sel.contains(Adapter::Halo2) {
                    return Ok(());
                }
            }
            fs::create_dir_all(parent)?;
        }

        // Write the file to the output directory
        match file.as_file() {
            Some(file) => {
                if let Some(path_str) = relative_path.to_str() {
                    if contains_circom(path_str) && !adapter_sel.contains(Adapter::Circom) {
                        return Ok(());
                    }
                }
                if let Some(path_str) = relative_path.to_str() {
                    if contains_halo2(path_str) && !adapter_sel.contains(Adapter::Halo2) {
                        return Ok(());
                    }
                }
                if let Err(e) = fs::write(&output_path, file.contents()) {
                    return Err(e.into());
                }
            }
            None => {
                copy_embedded_dir(file.as_dir().unwrap(), output_dir, adapter_sel)?;
            }
        }
    }
    Ok(())
}

fn replace_project_name(file_path: &str, project_name: &str) -> Result<()> {
    let target = "<PROJECT_NAME>";
    replace_string_in_file(file_path, target, project_name)
}

fn replace_features(file_path: &str, adapters: Vec<&str>) -> Result<()> {
    let target = "\"<FEATURES>\"";

    let features: Vec<String> = adapters
        .iter()
        .map(|adapter| format!("\"mopro-ffi/{}\"", adapter))
        .collect();

    replace_string_in_file(file_path, target, &features.join(", "))
}

fn replace_string_in_file(file_path: &str, target: &str, replacement: &str) -> Result<()> {
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
