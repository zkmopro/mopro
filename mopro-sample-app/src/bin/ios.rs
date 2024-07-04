fn main() {
    // cargo build --release && cargo run --bin ios generate --library target/release/libmopro_bindings.dylib --language swift --out-dir target/out
    uniffi::uniffi_bindgen_main();
    mopro_ffi::app_config::ios::build();
}
