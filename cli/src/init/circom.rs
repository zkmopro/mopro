use crate::init::adapter::Adapter;
use crate::init::proving_system::ProvingSystem;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Circom;

impl ProvingSystem for Circom {
    const TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/src/template/circom");

    const ADAPTER: Adapter = Adapter::Circom;

    const DEPENDENCIES: &'static str = r#"
circom-prover = { git = "https://github.com/zkmopro/mopro.git" }
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
    "#;

    const BUILD_TEMPLATE: &'static str = r#"
    rust_witness::transpile::transpile_wasm("./test-vectors/circom".to_string());
    "#;
}
