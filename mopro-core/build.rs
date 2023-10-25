use color_eyre::eyre::Result;

#[cfg(feature = "dylib")]
fn build_dylib() -> Result<()> {
    use color_eyre::eyre::eyre;
    use enumset::enum_set;
    use enumset::EnumSet;
    use std::path::Path;
    use std::{env, str::FromStr};

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
    println!("cargo:warning=mopro-core: OUT_DIR={}", out_dir.display());
    let dylib_file = out_dir.join("keccak256.dylib");
    println!(
        "cargo:rustc-env=CIRCUIT_WASM_DYLIB={}",
        dylib_file.display()
    );

    // if dylib_file.exists() {
    //     return Ok(());
    // }

    // Create a WASM engine for the target that can compile
    let target_arch = env::var("TARGET")?;
    let triple = Triple::from_str(&target_arch).map_err(|e| eyre!(e))?;
    let cpu_features = enum_set!();
    let target = Target::new(triple, cpu_features);
    let engine = Dylib::new(Cranelift::default()).target(target).engine();
    println!("cargo:warning=mopro-core: TARGET={}", target_arch);

    // Compile the WASM module
    let store = Store::new(&engine);
    let module = Module::from_file(&store, &wasm_file).unwrap();
    module.serialize_to_file(&dylib_file).unwrap();
    assert!(dylib_file.exists());
    println!("cargo:warning=mopro-core: Dylib {}", dylib_file.display());

    Ok(())
}

fn main() -> Result<()> {
    #[cfg(feature = "dylib")]
    build_dylib()?;
    Ok(())
}
