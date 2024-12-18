pub use mopro_wasm::app_config;

#[cfg(target_family = "wasm")]
use mopro_wasm::halo2::{gemini, hyperplonk, plonk};
