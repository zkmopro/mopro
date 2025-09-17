use crate::init::adapter::Adapter;
use crate::init::proving_system::ProvingSystem;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Noir;

impl ProvingSystem for Noir {
    const TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/src/template/noir");

    const ADAPTER: Adapter = Adapter::Noir;

    const DEPENDENCIES: &'static str = r#"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.94"

noir_rs = { package = "noir", git = "https://github.com/zkmopro/noir-rs", features = [
    "barretenberg",
    "android-compat",
], branch = "v1.0.0-beta.3-2" }
    "#;
}
