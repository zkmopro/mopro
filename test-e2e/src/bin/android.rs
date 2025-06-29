fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    mopro_ffi::app_config::android::build();
}
