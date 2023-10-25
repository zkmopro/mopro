use color_eyre::eyre::Result;

#[cfg(feature = "dylib")]
fn build_dylib() -> Result<()> {
    use color_eyre::eyre::eyre;
    use enumset::enum_set;
    use enumset::EnumSet;
    use std::path::Path;
    use std::path::PathBuf;
    use std::{env, fs, str::FromStr};

    use wasmer::Cranelift;
    use wasmer::Target;
    use wasmer::{Module, Store, Triple};

    use wasmer::Dylib;

    let wasm_path = "./../mopro-core/examples/circom/keccak256/target/keccak256_256_test_js/keccak256_256_test.wasm";
    let wasm_file = Path::new(wasm_path).to_path_buf();

    // TODO: Improve this, we want it to be by architecture
    // We can copy assets and clean up install_paths after
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir).to_path_buf();
    let dylib_file = out_dir.join("keccak256.dylib");

    // if dylib_file.exists() {
    //     return Ok(());
    // }

    // Create a WASM engine for the target that can compile
    let target_arch = env::var("TARGET")?;
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
    let target_arch = env::var("TARGET")?;
    let project_dir = env::var("CARGO_MANIFEST_DIR")?;
    let final_dir = PathBuf::from(&project_dir)
        .join("target/debug")
        .join(target_arch);

    // Ensure the final directory exists
    fs::create_dir_all(&final_dir)?;

    let final_path = final_dir.join("keccak256.dylib");
    fs::copy(&dylib_file, &final_path)?;
    println!("cargo:warning=Dylib location: {}", final_path.display());

    Ok(())
}

fn main() -> Result<()> {
    #[cfg(feature = "dylib")]
    build_dylib()?;
    Ok(())
}
