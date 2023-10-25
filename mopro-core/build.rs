use color_eyre::eyre::Result;

#[cfg(feature = "dylib")]
fn build_dylib(wasm_path: &str, dylib_name: &str) -> Result<()> {
    use std::path::Path;
    use std::path::PathBuf;
    use std::{env, fs, str::FromStr};

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
    let wasm_file = Path::new(wasm_path).to_path_buf();
    let dylib_file = out_dir.join(dylib_name);
    let final_dir = PathBuf::from(&project_dir)
        .join("target")
        .join(build_mode)
        .join(&target_arch);

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

fn main() -> Result<()> {
    #[cfg(feature = "dylib")]
    {
        let wasm_path = "./../mopro-core/examples/circom/keccak256/target/keccak256_256_test_js/keccak256_256_test.wasm";
        let dylib_name = "keccak256.dylib";
        build_dylib(wasm_path, dylib_name)?;
    }
    Ok(())
}
