use crate::init::adapter::Adapter;
use crate::init::proving_system::ProvingSystem;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Gnark;

impl ProvingSystem for Gnark {
    const TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/src/template/gnark");

    const ADAPTER: Adapter = Adapter::Gnark;

    const DEPENDENCIES: &'static str = r#"
rust-gnark = "0.0.1"
    "#;
}
