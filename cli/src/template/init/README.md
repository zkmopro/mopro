# Mopro example app

This is the example app of mopro. You can use the following commands to build native bindings for your iOS and/or Android app.

**📚 To learn more about mopro, visit: https://zkmopro.org**

## Getting Started

To set up and build bindings, follow these steps.

### 1. Install the Mopro CLI Tool

-   Get published CLI

```sh
cargo install mopro-cli
```

-   Or get the latest CLI on GitHub

```sh
git clone https://github.com/zkmopro/mopro
cd mopro/cli
cargo install --path .
```

### 2. Initialize adapter

Navigate to the Mopro example app directory and initialize setup by running:

```sh
mopro init
```

### 3. Generate Native Bindings

Build bindings for your project by executing:

```sh
mopro build
```

### 4. Create Platform-Specific Templates

To generate templates tailored to your target platform, use:

```sh
mopro create
```

### 5. Open the project

Follow the instructions to open the development tools

For iOS:

```sh
open ios/MoproApp.xcodeproj
```

For Android:

```sh
open android -a Android\ Studio
```

For Web:

```sh
cd web && yarn && yarn start
```

For React Native:
Follow the README in the `react-native` directory. Or [zkmopro/react-native-app/README.md](https://github.com/zkmopro/react-native-app/blob/main/README.md)

For Flutter:
Follow the README in the `flutter` directory. Or [zkmopro/flutter-app/README.md](https://github.com/zkmopro/flutter-app/blob/main/README.md)

### 6. Update bindings

After creating templates, you may still need to update the bindings.

`mopro build` will prompt you to run `mopro update` to refresh the bindings in each template. You can also run it automatically:

```sh
mopro build --auto-update
```

Or manually:

```sh
mopro update
```

## Customize Bindings

### UniFFI

For mobile native apps (iOS and Android), you can use `#[uniffi::export]` to define custom functions that will be included in the generated bindings. For example:

```rust
#[uniffi::export]
fn mopro_hello_world() -> String {
    "Hello, World!".to_string()
}
```

After defining your custom functions, run the standard Mopro commands (`mopro build`, `mopro create`, or `mopro update`) to regenerate and update the bindings for each target platform.

### `wasm_bindgen`

For web (WASM) apps, you can use `#[wasm_bindgen]` in [`src/lib.rs`](src/lib.rs) to expose custom functions to JavaScript. For example:

```rust
#[cfg_attr(
    all(feature = "wasm", target_arch = "wasm32"),
    wasm_bindgen(js_name = "moproWasmHelloWorld")
)]
pub fn mopro_wasm_hello_world() -> String {
    "Hello, World!".to_string()
}
```

After running `mopro build`, be sure to run `mopro update` to refresh the bindings in each template. This command automatically finds the appropriate bindings folders and updates them accordingly.

## Test

Run tests before building bindings

```sh
curl -L https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar -o jna-5.13.0.jar
export CLASSPATH=jna-5.13.0.jar
cargo test
```

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
