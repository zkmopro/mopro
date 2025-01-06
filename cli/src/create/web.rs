use anyhow::Result;
use include_dir::include_dir;
use include_dir::Dir;
use std::{env, fs, path::PathBuf};

use super::Create;
use crate::create::utils::{check_web_bindings, copy_dir, copy_embedded_dir, copy_embedded_file};
use crate::style::print_bold;
use crate::style::print_green_bold;

pub struct Web;

impl Create for Web {
    const NAME: &'static str = "web";

    fn create(project_dir: PathBuf) -> Result<()> {
        let wasm_bindings_dir = check_web_bindings(&project_dir)?;
        let target_dir = project_dir.join(Self::NAME);
        fs::create_dir(&target_dir)?;

        env::set_current_dir(&target_dir)?;
        const WEB_TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template/web");
        copy_embedded_dir(&WEB_TEMPLATE_DIR, &target_dir)?;

        env::set_current_dir(&project_dir)?;

        let target_wasm_bindings_dir = target_dir.join("MoproWasmBindings");
        fs::create_dir(target_wasm_bindings_dir.clone())?;
        copy_dir(&wasm_bindings_dir, &target_wasm_bindings_dir)?;

        let asset_dir = target_dir.join("assets");
        const HALO2_KEYS_DIR: Dir =
            include_dir!("$CARGO_MANIFEST_DIR/src/template/init/test-vectors/halo2");
        copy_embedded_file(&HALO2_KEYS_DIR, &asset_dir)?;

        fs::remove_dir_all(&wasm_bindings_dir)?;

        Self::print_message();
        Ok(())
    }

    fn print_message() {
        print_green_bold("Template created successfully!".to_string());
        println!();
        print_green_bold("Next steps:".to_string());
        println!();
        print_green_bold(
            "  You can now use the following command to start web server:".to_string(),
        );
        println!();
        print_bold(r"    cd web && yarn && yarn start".to_string());
        println!();
        print_green_bold("This will start a simple web server for your browser.".to_string());
    }
}
