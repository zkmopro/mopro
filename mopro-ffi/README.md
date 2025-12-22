# Mopro FFI

**What is MoPro?**

MoPro stands for **Mobile Prover** — a framework designed to simplify the development of client-side zero-knowledge (ZK) proof systems on mobile platforms.

**MoPro FFI** is a core component of this framework. It generates platform-specific bindings from Rust code, originally using [`UniFFI`](https://github.com/mozilla/uniffi-rs) to support Swift (iOS) and Kotlin (Android). MoPro FFI abstracts away the complexity of UniFFI setup, as well as integration with Xcode and Android Studio. It provides ready-to-use frameworks and packages that you can simply drag and drop into your Xcode or Android Studio project.

Now, **MoPro FFI** also support additional FFI tools such as [`wasm-bindgen`](https://github.com/RReverser/wasm-bindgen-rayon), [`flutter_rust_bridge`](https://github.com/fzyzcjy/flutter_rust_bridge), and [`uniffi-bindgen-react-native`](https://github.com/jhugman/uniffi-bindgen-react-native) — eliminating the need for developers to configure each tool manually.

To get the most out of MoPro FFI, we recommend using the companion tool [`mopro-cli`](https://crates.io/crates/mopro-cli). It offers:

-   Built-in support for various ZK proof systems (e.g., Circom, Halo2, and Noir)
-   Project templates for platforms like Xcode, Android Studio, React Native, Flutter, and the Web

With MoPro CLI, you can scaffold and build a privacy-preserving mobile app in minutes.

👉 Visit [zkmopro.org](https://zkmopro.org) to learn more about using MoPro and MoPro FFI.

## Usage

We recommend using `mopro-cli` to generate project templates for a smoother experience.
However, you can also use the `mopro-ffi` package directly to generate bindings by following the setup instructions below. You can also refer to [Rust Setup](https://zkmopro.org/docs/setup/rust-setup) for more details.

Install `mopro-ffi` through `cargo add mopro-ffi` or add `mopro-ffi` in `Cargo.toml`

```toml
[dependencies]
mopro-ffi = "0.3"
```

Define the `crate-type` for the UniFFI build process configuration.

```toml
[lib]
crate-type = ["lib", "cdylib", "staticlib"]
```

### iOS

Create a file `src/bin/ios.rs` in your rust project

```rust
fn main() {
    mopro_ffi::app_config::ios::build();
}
```

Execute the process using the defined binaries. For example

```sh
cargo run --bin ios # Debug mode for iOS
CONFIGURATION=release cargo run --bin ios # Release mode for iOS
IOS_ARCHS=aarch64-apple-ios,aarch64-apple-ios-sim cargo run --bin ios # Build for iOS aarch64-apple-ios and aarch64-apple-ios-sim architecture
```

or

```sh
mopro build --platforms ios # with mopro CLI
```

It will generate bindings for the function you defined with `#[uniffi::export]`. Please checkout [UniFFI | Procedural Macros: Attributes and Derives](https://mozilla.github.io/uniffi-rs/latest/proc_macro/index.html) for more details.

### Android

Create a file `src/bin/android.rs` in your rust project

```rust
fn main() {
    mopro_ffi::app_config::android::build();
}
```

Execute the process using the defined binaries. For example

```sh
cargo run --bin android # Debug mode for Android
CONFIGURATION=release cargo run --bin android # Release mode for Android
ANDROID_ARCHS=x86_64-linux-android cargo run --bin android # Build for Android x86_64-linux-android architecture
```

or

```sh
mopro build --platforms android # with mopro CLI
```

It will generate bindings for the function you defined with `#[uniffi::export]`. Please checkout [UniFFI | Procedural Macros: Attributes and Derives](https://mozilla.github.io/uniffi-rs/latest/proc_macro/index.html) for more details.

### Flutter

Create a file `src/bin/flutter.rs` in your rust project

```rust
fn main() {
    mopro_ffi::app_config::flutter::build();
}
```

Execute the process using the defined binaries. For example

```sh
cargo run --bin flutter --no-default-features --features flutter # Debug mode for Flutter
CONFIGURATION=release cargo run --bin flutter --no-default-features --features flutter # Release mode for Flutter
```

or

```sh
mopro build --platforms flutter # with mopro CLI
```

It will generate bindings for the function you defined with `pub fn` in `src/lib.rs`.

### React Native

Create a file `src/bin/react_native.rs` in your rust project

```rust
fn main() {
    mopro_ffi::app_config::react_native::build();
}
```

Execute the process using the defined binaries. For example

```sh
cargo run --bin react_native # Debug mode for react native
CONFIGURATION=release cargo run --bin react_native # Release mode for react native
```

or

```sh
mopro build --platforms react-native # with mopro CLI
```

It will generate bindings for the function you defined with `#[uniffi::export]`. Please checkout [UniFFI | Procedural Macros: Attributes and Derives](https://mozilla.github.io/uniffi-rs/latest/proc_macro/index.html) for more details.

### WASM

Create a file `src/bin/web.rs` in your rust project

```rust
fn main() {
    mopro_ffi::app_config::web::build();
}
```

Execute the process using the defined binaries. For example

```sh
cargo run --bin web --no-default-features --features wasm # Debug mode for wasm
CONFIGURATION=release cargo run --bin web --no-default-features --features wasm # Release mode for wasm
```

or

```sh
mopro build --platforms web # with mopro CLI
```

It will generate bindings for the function you defined with `#[wasm_bindgen]`. Please checkout [The `wasm-bindgen` Guide](https://wasm-bindgen.github.io/wasm-bindgen/examples/hello-world.html) for more details.

### Integration

-   To integrate the generated bindings into your mobile development project, please refer to the appropriate platform-specific setup guides:
    -   [iOS Setup](https://zkmopro.org/docs/setup/ios-setup)
    -   [Android Setup](https://zkmopro.org/docs/setup/android-setup)
    -   [React Native Setup](https://zkmopro.org/docs/setup/react-native-setup)
    -   [Flutter Setup](https://zkmopro.org/docs/setup/flutter-setup)
    -   [Web(Wasm) Setup](https://zkmopro.org/docs/setup/web-wasm-setup)

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
-   Mopro Documentation: https://zkmopro.org

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
