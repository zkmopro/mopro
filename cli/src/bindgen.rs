use crate::init::replace_string_in_file;
use crate::style;
use std::{collections::HashMap, env, ffi::OsStr, fs, path::Path};

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use include_dir::include_dir;
use include_dir::Dir;
use mopro_ffi::app_config::constants::{ANDROID_BINDINGS_DIR, IOS_BINDINGS_DIR};
use walkdir::WalkDir;

use crate::{
    build::build_project,
    init::{adapter::Adapter, init_project},
};

#[derive(PartialEq)]
enum WitnessAdapter {
    RustWitness,
    WitnessCalc,
}

pub fn bindgen(
    arg_mode: &Option<String>,
    arg_platforms: &Option<Vec<String>>,
    arg_architectures: &Option<Vec<String>>,
    circuit_dir: &Option<String>,
    adapter: &Option<String>,
    output_dir: &Option<String>,
) -> Result<()> {
    let witness_adapter = match adapter.as_deref() {
        Some("witnesscalc") => WitnessAdapter::WitnessCalc,
        Some("rust-witness") | None => WitnessAdapter::RustWitness,
        Some(other) => return Err(anyhow::anyhow!(format!("Unsupported adapter: {other}"))),
    };

    // Currently only support circom proving system
    let proving_adapter = Adapter::Circom;

    let specified_circuit_dir;
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
    let output_base_dir = if let Some(out) = output_dir {
        let path = Path::new(out);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            current_dir.join(path)
        }
    } else {
        current_dir.clone()
    };
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
    let mut circuit_map: HashMap<String, String> = HashMap::new();
    for entry in WalkDir::new(&absolute_circuit_dir) {
        let e = entry.unwrap();
        let path = e.path();
        if path.is_dir() {
            continue;
        }
        let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");
        match (&witness_adapter, ext) {
            (WitnessAdapter::RustWitness, "wasm") | (WitnessAdapter::WitnessCalc, "dat") => {
                let circuit_name = path.file_stem().unwrap();
                let circuit_name_compressed = circuit_name
                    .to_str()
                    .unwrap()
                    .replace("_", "")
                    .replace("-", "");
                project_name = circuit_name_compressed;
                circuit_map.insert(circuit_name.to_str().unwrap().to_string(), "".to_string());
            }
            _ => {}
        }
    }

    // find zkey and wasm mapping
    for entry in WalkDir::new(&absolute_circuit_dir) {
        let e = entry.unwrap();
        let path = e.path();
        if path.is_dir() {
            continue;
        }
        let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");
        if ext == "zkey" {
            let circuit_name = path.file_stem().unwrap();
            for (key, value) in &mut circuit_map {
                if circuit_name.to_str().unwrap().contains(key) {
                    *value = path.to_str().unwrap().to_string();
                }
            }
        }
    }

    init_project(
        &Some(proving_adapter.as_str().to_string()),
        &Some(project_name.to_string()),
        true,
    )?;

    let project_dir = env::current_dir()?;
    let test_vectors_dir = project_dir.join("test-vectors").join("circom");
    fs::create_dir_all(&test_vectors_dir)?;

    // Copy the entire directory recursively
    // Remove the destination directory if it exists to avoid conflicts
    if test_vectors_dir.exists() {
        fs::remove_dir_all(&test_vectors_dir)?;
    }
    // Create the destination directory
    fs::create_dir_all(&test_vectors_dir)?;

    // Copy the contents of the circuit directory into test-vectors/circom
    // This ensures files end up in the correct location regardless of source dir name
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.content_only = true;
    fs_extra::dir::copy(&absolute_circuit_dir, &test_vectors_dir, &copy_options)?;

    let lib_rs_path = project_dir.join("src").join("lib.rs");
    let template_dir: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/circom");
    let circom_lib_rs = match template_dir.get_file("lib.rs") {
        Some(file) => file.contents(),
        None => return Err(anyhow::anyhow!("lib.rs not found in template")),
    };
    let replacement = generate_witness_functions(&circuit_map, &witness_adapter);
    replace_string_in_file(
        lib_rs_path.to_str().unwrap(),
        &String::from_utf8_lossy(circom_lib_rs),
        &replacement,
    )?;

    if witness_adapter == WitnessAdapter::WitnessCalc {
        let cargo_toml_path = project_dir.join("Cargo.toml");
        let cargo_toml_str = cargo_toml_path.to_str().unwrap();
        replace_string_in_file(
            cargo_toml_str,
            "mopro-ffi = { version = \"0.3.2-alpha.0\", features = [\"uniffi\"] }",
            "mopro-ffi = { version = \"0.3.2-alpha.0\", features = [\"uniffi\", \"witnesscalc\"] }",
        )?;
        replace_string_in_file(cargo_toml_str, "rust-witness = \"0.1\"\n", "")?;
        replace_string_in_file(
            cargo_toml_str,
            "[dependencies]\n",
            "[dependencies]\nwitnesscalc-adapter = \"0.1\"\n",
        )?;
        replace_string_in_file(
            cargo_toml_str,
            "[build-dependencies]\n",
            "[build-dependencies]\nwitnesscalc-adapter = \"0.1\"\n",
        )?;

        let build_rs_path = project_dir.join("build.rs");
        replace_string_in_file(
            build_rs_path.to_str().unwrap(),
            "rust_witness::transpile::transpile_wasm(\"./test-vectors/circom\".to_string());",
            "witnesscalc_adapter::build_and_link(\"./test-vectors/circom\");",
        )?;
    }
    // Run the build command
    build_project(arg_mode, arg_platforms, arg_architectures, None, true)?;

    // Copy the bindings folder to the output directory
    fs::create_dir_all(&output_base_dir)?;
    let ios_bindings_dir = project_dir.join(IOS_BINDINGS_DIR);
    if ios_bindings_dir.exists() {
        let output_ios_bindings_dir = current_dir.join(IOS_BINDINGS_DIR);
        fs::create_dir_all(&output_ios_bindings_dir)?;
        fs_extra::dir::copy(
            &ios_bindings_dir,
            &output_base_dir,
            &fs_extra::dir::CopyOptions::new(),
        )?;
    }

    let android_bindings_dir = project_dir.join(ANDROID_BINDINGS_DIR);
    if android_bindings_dir.exists() {
        fs_extra::dir::copy(
            &android_bindings_dir,
            &output_base_dir,
            &fs_extra::dir::CopyOptions::new(),
        )?;
    }

    fs::remove_dir_all(&project_dir)?;
    style::print_green_bold(format!(
        "✨ Bindings Generated Successfully at {} ✨",
        output_base_dir.display()
    ));

    Ok(())
}

fn generate_witness_functions(
    circuit_map: &HashMap<String, String>,
    adapter: &WitnessAdapter,
) -> String {
    let mut content = String::new();
    for key in circuit_map.keys() {
        let circuit_key = key.replace("_", "").replace("-", "");
        match adapter {
            WitnessAdapter::RustWitness => {
                content.push_str(&format!("rust_witness::witness!({circuit_key});\n"));
            }
            WitnessAdapter::WitnessCalc => {
                content.push_str(&format!("witnesscalc_adapter::witness!({circuit_key});\n"));
            }
        }
    }

    content.push('\n');
    content.push_str("mopro_ffi::set_circom_circuits! {\n");

    for (key, value) in circuit_map {
        let circuit_key = key.replace("_", "").replace("-", "");
        match adapter {
            WitnessAdapter::RustWitness => {
                content.push_str(&format!(
                    "({value:?}, mopro_ffi::witness::WitnessFn::RustWitness({circuit_key}_witness)),\n",
                ));
            }
            WitnessAdapter::WitnessCalc => {
                content.push_str(&format!(
                    "({value:?}, mopro_ffi::witness::WitnessFn::WitnessCalc({circuit_key}_witness)),\n",
                ));
            }
        }
    }

    content.push('}');
    content
}
