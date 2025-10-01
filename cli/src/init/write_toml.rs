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
default = []

[dependencies]
mopro-wasm = { git = "https://github.com/zkmopro/mopro.git", branch = "v0.2.x" }
mopro-ffi = { version = "0.3.1", features = ["uniffi"] }
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
mopro-ffi = { version = "0.3.1", features = ["uniffi-tests"] }

# CIRCOM_DEV_DEPENDENCIES
# HALO2_DEV_DEPENDENCIES
# NOIR_DEV_DEPENDENCIES

    "# // TODO - make build dependencies also configurable
}
