use crate::print::print_init_instructions;
use crate::style;
use crate::style::create_custom_theme;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use dialoguer::MultiSelect;
use include_dir::include_dir;
use include_dir::Dir;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

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

    let adapters = vec!["circom", "halo2"];

    let theme = create_custom_theme();
    let selection = match arg_adapter.as_deref() {
        None => MultiSelect::with_theme(&theme)
            .with_prompt("Pick the adapters you want to use (multiple selection with space)")
            .items(&adapters)
            .interact()
            .unwrap(),
        Some(a) => {
            let mut selection = vec![];
            for (i, adapter) in adapters.iter().enumerate() {
                if a.contains(adapter) {
                    selection.push(i);
                }
            }
            selection
        }
    };

    if selection.is_empty() {
        style::print_yellow("No adapters selected. Use space to select an adapter".to_string());
        init_project(arg_adapter, &Some(project_name))?;
    } else {
        let current_dir = env::current_dir()?;

        let project_dir = current_dir.join(&project_name);
        fs::create_dir(&project_dir)?;

        // Change directory to the project directory
        env::set_current_dir(&project_dir)?;
        const TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/init");

        copy_embedded_dir(&TEMPLATE_DIR, &project_dir, selection.clone())?;

        if let Some(cargo_toml_path) = project_dir.join("Cargo.toml").to_str() {
            replace_project_name(cargo_toml_path, &project_name)?;
            replace_features(
                cargo_toml_path,
                selection.iter().map(|&i| adapters[i]).collect(),
            )?;
            if selection.contains(&0) {
                // circom is selected
                circom_dependencies_template(cargo_toml_path)?;
            }
            if selection.contains(&1) {
                // halo2 is selected
                halo2_dependencies_template(cargo_toml_path)?;
            }
        }

        if let Some(build_rs_path) = project_dir.join("build.rs").to_str() {
            if selection.contains(&0) {
                circom_build_template(build_rs_path)?;
            }
        }

        if let Some(lib_rs_path) = project_dir.join("src").join("lib.rs").to_str() {
            if selection.contains(&0) {
                // circom is selected
                circom_lib_template(lib_rs_path)?;
            }
            if selection.contains(&1) {
                // halo2 is selected
                halo2_lib_template(lib_rs_path)?;
            }
        }

        // Print out the instructions
        print_init_instructions(project_name);
    }
    Ok(())
}

pub fn copy_embedded_dir(dir: &Dir, output_dir: &Path, selection: Vec<usize>) -> anyhow::Result<()> {
    for file in dir.entries() {
        let relative_path = file.path();
        let output_path = output_dir.join(relative_path);

        // Create directories as needed
        if let Some(parent) = output_path.parent() {
            if let Some(path_str) = parent.to_str() {
                if path_str.contains("circom") && !selection.contains(&0) {
                    return Ok(());
                }
            }
            if let Some(path_str) = parent.to_str() {
                if path_str.contains("halo2") && !selection.contains(&1) {
                    return Ok(());
                }
            }
            fs::create_dir_all(parent)?;
        }

        // Write the file to the output directory
        match file.as_file() {
            Some(file) => {
                if let Some(path_str) = relative_path.to_str() {
                    if path_str.contains("circom") && !selection.contains(&0) {
                        return Ok(());
                    }
                }
                if let Some(path_str) = relative_path.to_str() {
                    if path_str.contains("halo2") && !selection.contains(&1) {
                        return Ok(());
                    }
                }
                if let Err(e) = fs::write(&output_path, file.contents()) {
                    return Err(e.into());
                }
            }
            None => {
                copy_embedded_dir(file.as_dir().unwrap(), output_dir, selection.clone())?;
            }
        }
    }
    Ok(())
}

fn replace_project_name(file_path: &str, project_name: &str) -> anyhow::Result<()> {
    let target = "<PROJECT_NAME>";
    replace_string_in_file(file_path, target, project_name)
}

fn replace_features(file_path: &str, adapters: Vec<&str>) -> anyhow::Result<()> {
    let target = "\"<FEATURES>\"";

    let features: Vec<String> = adapters
        .iter()
        .map(|adapter| format!("\"mopro-ffi/{}\"", adapter))
        .collect();

    replace_string_in_file(file_path, target, &features.join(", "))
}

fn circom_lib_template(file_path: &str) -> anyhow::Result<()> {
    let template_dir: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/circom");
    let circom_lib_rs = match template_dir.get_file("lib.rs") {
        Some(file) => file.contents(),
        None => return Err(anyhow::anyhow!("lib.rs not found in template")),
    };
    let target = "// CIRCOM_TEMPLATE";
    replace_string_in_file(file_path, target, &String::from_utf8_lossy(circom_lib_rs))
}

fn halo2_lib_template(file_path: &str) -> anyhow::Result<()> {
    let template_dir: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/halo2");
    let halo2_lib_rs = match template_dir.get_file("lib.rs") {
        Some(file) => file.contents(),
        None => return Err(anyhow::anyhow!("lib.rs not found in template")),
    };
    let target = "// HALO2_TEMPLATE";
    replace_string_in_file(file_path, target, &String::from_utf8_lossy(halo2_lib_rs))
}

fn circom_build_template(file_path: &str) -> anyhow::Result<()> {
    let replacement =
        "rust_witness::transpile::transpile_wasm(\"./test-vectors/circom\".to_string());";
    let target = "// CIRCOM_TEMPLATE";
    replace_string_in_file(file_path, target, replacement)
}

fn halo2_dependencies_template(file_path: &str) -> anyhow::Result<()> {
    let replacement =
        "plonk-fibonacci = { package = \"plonk-fibonacci\", git = \"https://github.com/sifnoc/plonkish-fibonacci-sample.git\" }";
    let target = "# HALO2_DEPENDENCIES";
    replace_string_in_file(file_path, target, replacement)
}

fn circom_dependencies_template(file_path: &str) -> anyhow::Result<()> {
    let replacement =
        "# TODO: fix this
[patch.crates-io]
ark-circom = { git = \"https://github.com/zkmopro/circom-compat.git\", version = \"0.1.0\", branch = \"wasm-delete\" }";
    let target = "# CIRCOM_DEPENDENCIES";
    replace_string_in_file(file_path, target, replacement)
}

fn replace_string_in_file(file_path: &str, target: &str, replacement: &str) -> anyhow::Result<()> {
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
