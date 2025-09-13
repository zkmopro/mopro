use crate::init::adapter::Adapter;
use crate::init::proving_system::ProvingSystem;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Circom;

impl ProvingSystem for Circom {
    const TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/src/template/circom");

    const ADAPTER: Adapter = Adapter::Circom;

    const DEPENDENCIES: &'static str = r#"
circom-prover = { git = "https://github.com/zkmopro/mopro.git", features = ["rapidsnark"] }
rust-witness  = "0.1"
num-bigint    = "0.4.0"
    "#;
    const BUILD_DEPENDENCIES: &'static str = r#"
witnesscalc-adapter = "0.1"
rust-witness = "0.1"
    "#;
    const DEV_DEPENDENCIES: &'static str = r#"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.94"

circom-prover = { git = "https://github.com/zkmopro/mopro.git", features = ["rapidsnark", "witnesscalc"] }
witnesscalc-adapter = "0.1"
    "#;

    const BUILD_TEMPLATE: &'static str = r#"
    rust_witness::transpile::transpile_wasm("./test-vectors/circom".to_string());
    witnesscalc_adapter::build_and_link("./test-vectors/circom/witnesscalc");

    // For running the uniffi tests on macOS with `witnesscalc`,
    // we need to set the rpath to find the
    // `witnesscalc` generated dynamic libraries at runtime.
    #[cfg(target_os = "macos")]
    {
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let wc_dir = out_dir.join("witnesscalc/package/lib");

        if wc_dir.exists() {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", wc_dir.display());
        } else {
            panic!(
                "Expected witnesscalc lib path does not exist: {}",
                wc_dir.display()
            );
        }
    }
    "#;
}
