# Rust Setup for Android/iOS Bindings

This tutorial provides step-by-step instructions to manually build static libraries with Circom and Halo2 adapters for Android and iOS. It focuses on a hands-on approach for developers who prefer or require manual setup.

Make sure you've installed the [prerequisites](/docs/prerequisites).

This tutorial will cover:

1. [Circom prover](#setup-circom-based-rust-project)

2. [Halo2 prover](#setup-halo2-based-rust-project)

3. [Universal Rust crate integration](#setup-any-rust-project)

## Setup Circom-Based rust project

You can integrate `mopro-ffi` into your own Rust crate, or alternatively, initialize a new project using

```sh
cargo init --lib
```

### 1. Add dependencies

Include the crate in your Cargo.toml:

```toml title="Cargo.toml"
[dependencies]
mopro-ffi = { version = "0.2", features = ["circom"] }
uniffi = "0.29"
circom-prover = "0.1"
thiserror = "2.0.12"

[build-dependencies]
mopro-ffi = "0.2"
uniffi = { version = "0.29", features = ["build"] }
```

### 2. Setup the lib

Define the name and type for the UniFFI build process configuration.

```toml title="Cargo.toml"
[lib]
name = "mopro_bindings"
crate-type = ["lib", "cdylib", "staticlib"]
```

:::warning
The name of the lib could be fixed in the future. See: [#387](https://github.com/zkmopro/mopro/issues/387)
:::

### 3. Add witness generator

Each witness generator must be built within a project. You need to supply the required data for it to generate a circuit-specific execution function.

Here, we used [rust-witness](https://github.com/chancehudson/rust-witness) as an example.

:::info
To learn more about `witnesscalc` for Mopro, please check out [circom-prover](https://github.com/zkmopro/mopro/blob/main/circom-prover/README.md#advanced-usage).
:::

Include the `rust-witness` in your Cargo.toml

```toml title="Cargo.toml"
[dependencies]
rust-witness = "0.1"
num-bigint = "0.4"

[build-dependencies]
rust-witness = "0.1"
```

In `build.rs`, add the following code to compile the witness generator wasm sources (.wasm) into a native library and link to it:

```rust title="build.rs"
fn main() {
    rust_witness::transpile::transpile_wasm("../path to directory containing your wasm sources");
    // e.g. rust_witness::transpile::transpile_wasm("./test-vectors".to_string());
    // The directory should contain the following files:
    // - <circuit name>.wasm
}
```

:::info
Here are the example WASM and Zkey files to be downloaded.

-   http://ci-keys.zkmopro.org/multiplier2.wasm
-   http://ci-keys.zkmopro.org/multiplier2_final.zkey

:::

### 4. Use `mopro-ffi` macro

The `mopro-ffi` macro exports the default Circom prover interfaces. To enable it, activate the mopro-ffi macro in `src/lib.rs`.

```rust title="src/lib.rs"
mopro_ffi::app!();
```

Bind the corresponding WASM and Zkey files together using mopro-ffi.

```rust title="src/lib.rs"
use circom_prover::witness::WitnessFn;

// Activate rust-witness function
rust_witness::witness!(multiplier2);

// Set the witness functions to a zkey
mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", WitnessFn::RustWitness(multiplier2_witness)),
}
```

### 5. Define the binaries

The binaries are used to generate bindings for both iOS and Android platforms.

We'll add a new file at `src/bin/ios.rs`:

```rust title="src/bin/ios.rs"
fn main() {
    mopro_ffi::app_config::ios::build();
}
```

and another at `src/bin/android.rs`:

```rust title="src/bin/android.rs"
fn main() {
    mopro_ffi::app_config::android::build();
}
```

### 6. Generate bindings for iOS and Android

Now you're ready to build your static library! You should be able to run either the Mopro CLI or the binaries.

**1. Execute the process through Mopro CLI**

Create a `Config.toml` configuration file like:

```toml title="Config.toml"
target_adapters = [
    "circom",
]
target_platforms = [
    "android",
    "ios",
]
```

To install the Mopro CLI, please refer to the [Getting Started](/docs/getting-started) guide.

Execute the building through

```sh
mopro build
```

Then, you can select the target mode and architectures with greater flexibility.

**2. Execute the process using the defined binaries.**

```sh
cargo run --bin ios # Debug mode for iOS
```

```sh
cargo run --bin android # Debug mode for Android
```

```sh
CONFIGURATION=release cargo run --bin ios # Release mode for iOS
```

```sh
CONFIGURATION=release cargo run --bin android # Release mode for Android
```

to build the corresponding static library. Move on to [iOS setup](ios-setup) or [Android setup](android-setup) to begin integrating in an app.

:::info
Running your project in release mode significantly enhances performance compared to debug mode. This is because the Rust compiler applies optimizations that improve runtime speed and reduce binary size, making your application more efficient.
:::

## Setup Halo2-Based rust project

Similar to the [Setup Circom-based Rust project](#setup-circom-based-rust-project), you can integrate `mopro-ffi` into your own Rust crate, or alternatively, initialize a new project using

```sh
cargo init --lib
```

### 1. Add dependencies

Include the crate in your Cargo.toml:

```toml title="Cargo.toml"
[dependencies]
mopro-ffi = { version = "0.2", features = ["halo2"] }
uniffi = "0.29"
thiserror = "2.0.12"

[build-dependencies]
mopro-ffi = "0.2"
uniffi = { version = "0.29", features = ["build"] }
```

### 2. Setup the lib

Similar to [Setup the lib](#2-setup-the-lib), define the name and type for the UniFFI build process configuration.

```toml title="Cargo.toml"
[lib]
name = "mopro_bindings"
crate-type = ["lib", "cdylib", "staticlib"]
```

:::warning
The name of the lib could be fixed in the future. See: [#387](https://github.com/zkmopro/mopro/issues/387)
:::

### 3. Add Halo2 circuits

Import the Halo2 prover as a Rust crate using:

```toml title="Cargo.toml"
[dependencies]
plonk-fibonacci = { package = "plonk-fibonacci", git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git" }
```

:::info
See how to define a Halo2 prover crate here: [plonkish-fibonacci-sample](https://github.com/sifnoc/plonkish-fibonacci-sample)
:::

Next, copy your SRS and key files into the project folder. For this tutorial, we'll assume you place them in `test-vectors/halo2`.

:::info
Download example SRS and key files :

-   [plonk_fibonacci_srs.bin](https://github.com/zkmopro/mopro/blob/dfb9b286c63f6b418fe27465796c818996558bf7/test-vectors/halo2/plonk_fibonacci_srs.bin)
-   [plonk_fibonacci_pk.bin](https://github.com/zkmopro/mopro/blob/dfb9b286c63f6b418fe27465796c818996558bf7/test-vectors/halo2/plonk_fibonacci_pk.bin)
-   [plonk_fibonacci_vk.bin](https://github.com/zkmopro/mopro/blob/dfb9b286c63f6b418fe27465796c818996558bf7/test-vectors/halo2/plonk_fibonacci_vk.bin)

:::

### 4. Use `mopro-ffi` macro

Now, add three rust files, just as in the Circom-based Rust project setup [4. Use `mopro-ffi` macro](#4-use-mopro-ffi-macro).

Update the `src/lib.rs` file to look like the following:

```rust title="src/lib.rs"
mopro_ffi::app!();

mopro_ffi::set_halo2_circuits! {
    ("plonk_fibonacci_pk.bin", plonk_fibonacci::prove, "plonk_fibonacci_vk.bin", plonk_fibonacci::verify),
}
```

### 5. Define the binaries

Similar to Circom-based Rust project setup [5. Define the binaries](#5-define-the-binaries)

### 6. Generate bindings for iOS and Android

Similar to Circom-based Rust project setup [6. Generate bindings for iOS and Android](#6-generate-bindings-for-ios-and-android)

However, creating a `Config.toml` like

```toml title="Config.toml"
target_adapters = [
    "halo2",
]
target_platforms = [
    "android",
    "ios",
]
```

## Setup any rust project

In addition to supporting Circom and Halo2 circuits, `mopro-ffi` allows integration with any Rust crate, enabling developers to define custom functions for iOS and Android. This ensures that Rust developers can leverage their existing expertise while seamlessly building comprehensive packages for mobile development.

### 1. Add dependencies

Include the crate in your Cargo.toml:

```toml title="Cargo.toml"
[dependencies]
mopro-ffi = { version = "0.2" }
uniffi = "0.29"

[build-dependencies]
mopro-ffi = "0.2"
uniffi = { version = "0.29", features = ["build"] }
```

### 2. Setup the lib

Similar to [Setup the lib](#2-setup-the-lib), define the name and type for the UniFFI build process configuration.

```toml title="Cargo.toml"
[lib]
name = "mopro_bindings"
crate-type = ["lib", "cdylib", "staticlib"]
```

### 3. Define exported functions

Export Rust functions using a procedural macro, as shown below:

```rust title="src/lib.rs"
mopro_ffi::app!();

#[uniffi::export]
pub fn hello_world() -> String {
    "Hello, World!".to_string()
}
```

For more examples and detailed references, check the [UniFFI documentation](https://mozilla.github.io/uniffi-rs/0.29).

:::info
Enable the `mopro-ffi` macro to generate UniFFI scaffolding.
:::

### 4. Define the binaries

Similar to Circom-based Rust project setup [5. Define the binaries](#5-define-the-binaries)

### 5. Generate bindings for iOS and Android

Similar to Circom-based Rust project setup [6. Generate bindings for iOS and Android](#6-generate-bindings-for-ios-and-android)

However, creating a `Config.toml` like

```toml title="Config.toml"
target_adapters = []
target_platforms = [
    "android",
    "ios",
]
```