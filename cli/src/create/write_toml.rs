pub fn mopro_wasm_lib_toml() -> &'static str {
    r#"
[package]
name = "mopro-wasm-lib"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["rlib", "cdylib"]

[dependencies]
mopro-wasm = { git = "https://github.com/zkmopro/mopro",features = [
    "gemini",
    "hyperplonk",
    "plonk",
]}

[target.wasm32-unknown-unknown.dependencies]
console_error_panic_hook = "0.1.7"
getrandom = { version = "0.2.15", features = ["js"] }
serde-wasm-bindgen = "0.6.5"
wasm-bindgen = { version = "0.2.95", features = ["serde-serialize"] }
wasm-bindgen-console-logger = "0.1.1"
wasm-bindgen-futures = "0.4.47"
wasm-bindgen-rayon = { version = "1.2.2", features = ["no-bundler"] }
wasm-bindgen-test = "0.3.42"
"#
}
