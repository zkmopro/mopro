use color_eyre::eyre::Result;
use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};
use toml;

#[derive(Deserialize)]
struct Config {
    circuit: CircuitConfig,
    dylib: Option<DylibConfig>,
}

#[derive(Deserialize)]
struct CircuitConfig {
    dir: String,
    name: String,
}

#[derive(Deserialize)]
struct DylibConfig {
    use_dylib: bool,
    name: String,
}

fn build_dylib(wasm_path: String, dylib_name: String) -> Result<()> {
    use std::path::Path;
    use std::str::FromStr;

    use color_eyre::eyre::eyre;
    use enumset::enum_set;
    use enumset::EnumSet;

    use wasmer::Cranelift;
    use wasmer::Dylib;
    use wasmer::Target;
    use wasmer::{Module, Store, Triple};

    let out_dir = env::var("OUT_DIR")?;
    let project_dir = env::var("CARGO_MANIFEST_DIR")?;
    let build_mode = env::var("PROFILE")?;
    let target_arch = env::var("TARGET")?;

    let out_dir = Path::new(&out_dir).to_path_buf();
    let wasm_file = Path::new(&wasm_path).to_path_buf();
    let dylib_file = out_dir.join(&dylib_name);
    let final_dir = PathBuf::from(&project_dir)
        .join("target")
        .join(&target_arch)
        .join(build_mode);

    // if dylib_file.exists() {
    //     return Ok(());
    // }

    // Create a WASM engine for the target that can compile
    let triple = Triple::from_str(&target_arch).map_err(|e| eyre!(e))?;
    let cpu_features = enum_set!();
    let target = Target::new(triple, cpu_features);
    let engine = Dylib::new(Cranelift::default()).target(target).engine();
    println!("cargo:warning=Building dylib for {}", target_arch);

    // Compile the WASM module
    let store = Store::new(&engine);
    let module = Module::from_file(&store, &wasm_file).unwrap();
    module.serialize_to_file(&dylib_file).unwrap();
    assert!(dylib_file.exists());

    // Copy dylib to a more predictable path
    fs::create_dir_all(&final_dir)?;
    let final_path = final_dir.join(dylib_name);
    fs::copy(&dylib_file, &final_path)?;
    println!("cargo:warning=Dylib location: {}", final_path.display());

    Ok(())
}

fn build_circuit(config: &Config) -> Result<()> {
    let project_dir = env::var("CARGO_MANIFEST_DIR")?;
    let circuit_dir = &config.circuit.dir;
    let circuit_name = &config.circuit.name;

    let zkey_path = PathBuf::from(&project_dir).join(format!(
        "{}/target/{}_final.zkey",
        circuit_dir, circuit_name
    ));
    let wasm_path = PathBuf::from(&project_dir).join(format!(
        "{}/target/{}_js/{}.wasm",
        circuit_dir, circuit_name, circuit_name
    ));
    let arkzkey_path = PathBuf::from(&project_dir).join(format!(
        "{}/target/{}_final.arkzkey",
        circuit_dir, circuit_name
    ));

    // TODO: Improve this to be more user-friendly
    assert!(
        zkey_path.exists(),
        "Make sure the zkey file exists. Did you forget to run a trusted setup? Adjust prepare.sh if necessary."
    );
    assert!(
        wasm_path.exists(),
        "Make sure the wasm file exists. Did you forget to compile the circuit to wasm? Adjust prepare.sh if necessary."
    );
    assert!(arkzkey_path.exists(), "Make sure the arkzkey file exists. Did you forget to generate the arkzkey? Adjust prepare.sh if necessary.");

    println!("cargo:warning=zkey_file: {}", zkey_path.display());
    println!("cargo:warning=wasm_file: {}", wasm_path.display());
    println!("cargo:warning=arkzkey_file: {}", arkzkey_path.display());

    // Set BUILD_RS_* environment variables
    println!("cargo:rustc-env=BUILD_RS_ZKEY_FILE={}", zkey_path.display());
    println!("cargo:rustc-env=BUILD_RS_WASM_FILE={}", wasm_path.display());
    println!(
        "cargo:rustc-env=BUILD_RS_ARKZKEY_FILE={}",
        arkzkey_path.display()
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_str = match env::var("BUILD_CONFIG_PATH") {
        Ok(config_path) => {
            println!("cargo:warning=config: {}", config_path);
            fs::read_to_string(config_path)?
        }
        Err(_) => {
            println!("cargo:warning=BUILD_CONFIG_PATH not set. Using default configuration.");
            // Default configuration
            let default_config = r#"
                    [circuit]
                    dir = "examples/circom/keccak256"
                    name = "keccak256_256_test"
    
                    #[dylib]
                    use_dlib = false
                    #name = "keccak256.dylib"
                "#;
            default_config.to_string()
        }
    };

    let config: Config = toml::from_str(&config_str)?;

    // Build circuit
    build_circuit(&config)?;

    // Build dylib if enabled
    if let Some(dylib_config) = &config.dylib {
        if dylib_config.use_dylib {
            println!("cargo:warning=Building dylib: {}", dylib_config.name);
            build_dylib(
                config.circuit.dir.clone()
                    + "/target/"
                    + &config.circuit.name
                    + "_js/"
                    + &config.circuit.name
                    + ".wasm",
                dylib_config.name.clone(),
            )?;
        } else {
            println!("cargo:warning=Dylib usage is disabled in the config.");
        }
    }

    Ok(())
}
