pub fn init_toml() -> &'static str {
    r#"
[package]
name = "MOPRO_TEMPLATE_PROJECT_NAME"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["lib", "cdylib", "staticlib"]

# Adapters for different proof systems
[features]
default = ["uniffi"]
uniffi = ["mopro-ffi/uniffi"]
flutter = ["mopro-ffi/flutter"]
wasm = ["mopro-ffi/wasm"]

[dependencies]
mopro-ffi = { git = "https://github.com/zkmopro/mopro.git", branch="mopro-wasm-refactor-2"}
thiserror = "2.0.12"
anyhow = "1.0.99"

# CIRCOM_DEPENDENCIES
# HALO2_DEPENDENCIES
# NOIR_DEPENDENCIES

[build-dependencies]
# CIRCOM_BUILD_DEPENDENCIES
# HALO2_BUILD_DEPENDENCIES
# NOIR_BUILD_DEPENDENCIES

[dev-dependencies]
mopro-ffi = { git = "https://github.com/zkmopro/mopro.git", branch="mopro-wasm-refactor-2", features = ["uniffi-tests"] }

# CIRCOM_DEV_DEPENDENCIES
# HALO2_DEV_DEPENDENCIES
# NOIR_DEV_DEPENDENCIES

[target.wasm32-unknown-unknown.dependencies]
mopro-ffi = { git = "https://github.com/zkmopro/mopro.git", branch="mopro-wasm-refactor-2", features = ["wasm"] }
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"

    "# // TODO - make build dependencies also configurable
}
