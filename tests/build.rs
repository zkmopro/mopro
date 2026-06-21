// Build-pipeline driver for the mopro test project.
//
// Each backend only runs if the corresponding arch env var is set.
// This lets `cargo build` succeed on any machine without toolchain requirements,
// while Xcode / CI / wasm-pack workflows opt in via env vars.
//
// iOS:          IOS_ARCHS=aarch64-apple-ios,aarch64-apple-ios-sim  cargo build
// Android:      ANDROID_ARCHS=aarch64-linux-android                cargo build
// Wasm:         WEB_ARCHS=wasm32-unknown-unknown                   cargo build
// Flutter:      FLUTTER_ARCHS=aarch64-apple-ios,aarch64-linux-android  cargo build
// React Native: REACT_NATIVE_ARCHS=aarch64-apple-ios              cargo build

fn main() {
    if std::env::var("IOS_ARCHS").is_ok() {
        mopro_uniffi_backend::ios::build();
    }
    if std::env::var("ANDROID_ARCHS").is_ok() {
        mopro_uniffi_backend::android::build();
    }
    if std::env::var("WEB_ARCHS").is_ok() {
        mopro_wasm_backend::build();
    }
    if std::env::var("FLUTTER_ARCHS").is_ok() {
        mopro_flutter_backend::build();
    }
    if std::env::var("REACT_NATIVE_ARCHS").is_ok() {
        mopro_react_native_backend::build();
    }
}
