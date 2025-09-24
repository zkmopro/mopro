use crate::init::adapter::Adapter;
use crate::init::proving_system::ProvingSystem;
use include_dir::include_dir;
use include_dir::Dir;

pub struct Halo2;

impl ProvingSystem for Halo2 {
    const TEMPLATE_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/src/template/halo2");

    const ADAPTER: Adapter = Adapter::Halo2;

    const DEPENDENCIES: &'static str = r#"
plonk-fibonacci   = { package = "plonk-fibonacci",   git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git" }
hyperplonk-fibonacci = { package = "hyperplonk-fibonacci", git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git" }
gemini-fibonacci  = { package = "gemini-fibonacci",  git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git" }
    "#;
}
