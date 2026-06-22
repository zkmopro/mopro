// Build-pipeline driver for the mopro test project.
//
// Each backend only runs if the corresponding arch env var is set.
// The library being compiled lives in test-app/ (which carries the mopro-ffi
// dependency); this crate only orchestrates the build pipeline.
//
// iOS:          IOS_ARCHS=aarch64-apple-ios,aarch64-apple-ios-sim  cargo build
// Android:      ANDROID_ARCHS=aarch64-linux-android                cargo build
// Wasm:         WEB_ARCHS=wasm32-unknown-unknown                   cargo build
// Flutter:      FLUTTER_ARCHS=aarch64-apple-ios,aarch64-linux-android  cargo build
// React Native: REACT_NATIVE_ARCHS=aarch64-apple-ios              cargo build

fn main() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let app_dir = manifest_dir.parent().unwrap().join("test-app");

    if std::env::var("IOS_ARCHS").is_ok() {
        mopro_uniffi_backend::ios::build_at(&app_dir);
    }
    if std::env::var("ANDROID_ARCHS").is_ok() {
        mopro_uniffi_backend::android::build_at(&app_dir);
    }
    if std::env::var("WEB_ARCHS").is_ok() {
        mopro_wasm_backend::build_at(&app_dir);
    }
    if std::env::var("FLUTTER_ARCHS").is_ok() {
        mopro_flutter_backend::build_at(&app_dir);
    }
    if std::env::var("REACT_NATIVE_ARCHS").is_ok() {
        mopro_react_native_backend::build_at(&app_dir);
    }
}
