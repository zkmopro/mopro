use crate::config::{read_config, write_config, Config};
use crate::create::write_toml;
use crate::print::print_init_instructions;
use crate::utils::contains_adapter;
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
        replace_features(
            cargo_toml_path,
            adapter_sel.adapters.iter().map(|a| a.as_str()).collect(),
        )?;
        adapter_sel.dep_template(cargo_toml_path);
    }

    if let Some(build_rs_path) = project_dir.join("build.rs").to_str() {
        adapter_sel.build_template(build_rs_path);
    }

    if let Some(lib_rs_path) = project_dir.join("src").join("lib.rs").to_str() {
        adapter_sel.lib_template(lib_rs_path);
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
                if contains_adapter(path_str, Adapter::Circom)
                    && !adapter_sel.contains(Adapter::Circom)
                {
                    return Ok(());
                }
            }
            if let Some(path_str) = parent.to_str() {
                if contains_adapter(path_str, Adapter::Halo2)
                    && !adapter_sel.contains(Adapter::Halo2)
                {
                    return Ok(());
                }
            }
            if let Some(path_str) = parent.to_str() {
                if contains_adapter(path_str, Adapter::Noir) && !adapter_sel.contains(Adapter::Noir)
                {
                    return Ok(());
                }
            }
            fs::create_dir_all(parent)?;
        }

        // Write the file to the output directory
        match file.as_file() {
            Some(file) => {
                if let Some(path_str) = relative_path.to_str() {
                    if contains_adapter(path_str, Adapter::Circom)
                        && !adapter_sel.contains(Adapter::Circom)
                    {
                        return Ok(());
                    }
                }
                if let Some(path_str) = relative_path.to_str() {
                    if contains_adapter(path_str, Adapter::Halo2)
                        && !adapter_sel.contains(Adapter::Halo2)
                    {
                        return Ok(());
                    }
                }
                if let Some(path_str) = relative_path.to_str() {
                    if contains_adapter(path_str, Adapter::Noir)
                        && !adapter_sel.contains(Adapter::Noir)
                    {
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
    let target = "MOPRO_TEMPLATE_PROJECT_NAME";
    replace_string_in_file(file_path, target, project_name)
}

fn replace_features(file_path: &str, adapters: Vec<&str>) -> Result<()> {
    let target = "\"<FEATURES>\"";

    let features: Vec<String> = adapters
        .iter()
        .map(|adapter| format!("\"mopro-ffi/{adapter}\""))
        .collect();

    replace_string_in_file(file_path, target, &features.join(", "))
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
