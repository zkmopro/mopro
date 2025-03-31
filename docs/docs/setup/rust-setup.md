# Manual Setup for Android/iOS Bindings

This tutorial provides step-by-step instructions to manually build static libraries with Circom and Halo2 adapters for Android and iOS. It focuses on a hands-on approach for developers who prefer or require manual setup.

Make sure you've installed the [prerequisites](/docs/prerequisites).

## Setup Circom-Based rust project

You can integrate `mopro-ffi` into your own Rust crate, or alternatively, initialize a new project using

```sh
cargo init --lib
```

### 1. Add dependencies

Include the crate in your Cargo.toml:

```toml
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

```toml
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
To learn more about witnesscalc for Mopro, please check out [circom-prover](https://github.com/zkmopro/mopro/blob/main/circom-prover/README.md#advanced-usage).
:::

Include the `rust-witness` in your Cargo.toml

```toml
[dependencies]
rust-witness = "0.1"
num-bigint = "0.4"

[build-dependencies]
rust-witness = "0.1"
```

In `build.rs`, add the following code to compile the witness generator wasm sources (.wasm) into a native library and link to it:

```rust
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

### 3. Use `mopro-ffi` macro

The `mopro-ffi` macro exports the default Circom prover interfaces. To enable it, activate the mopro-ffi macro in `src/lib.rs`.

```rust
mopro_ffi::app!();
```

Bind the corresponding WASM and Zkey files together using mopro-ffi.

```rust
use circom_prover::witness::WitnessFn;

// Activate rust-witness function
rust_witness::witness!(multiplier2);

// Set the witness functions to a zkey
mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", WitnessFn::RustWitness(multiplier2_witness)),
}
```

### 4. Define the binaries

The binaries are used to generate bindings for both iOS and Android platforms.

We'll add a new file at `src/bin/ios.rs`:

```rust
fn main() {
    mopro_ffi::app_config::ios::build();
}
```

and another at `src/bin/android.rs`:

```rust
fn main() {
    mopro_ffi::app_config::android::build();
}
```

### 5. Generate bindings for iOS and Android

Now you're ready to build your static library! You should be able to run either the Mopro CLI or the binaries.

**1. Execute the process through Mopro CLI**

Create a `Config.toml` configuration file like:

```toml
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

Similar to the [Setup Circom-based Rust project](#setup-circom-based-rust-project), start by running the following commands in your terminal:

```sh
mkdir mopro-example-app
cd mopro-example-app
cargo init --lib
```

This will create a new Rust project in the current directory. Next, add the required dependencies to the project. Edit your `Cargo.toml` to match the following:

```toml
[package]
name = "mopro-example-app"
version = "0.1.0"
edition = "2021"

# We're going to build a static library named mopro_bindings
# This library name should not be changed
[lib]
crate-type = ["lib", "cdylib", "staticlib"]
name = "mopro_bindings"

# Adapters for different proof systems
[features]
default = ["mopro-ffi/halo2"]

[dependencies]
mopro-ffi = { git = "https://github.com/zkmopro/mopro.git", branch = "main" }
uniffi = { version = "0.28", features = ["cli"] }
num-bigint = "0.4.0"
plonk-fibonacci = { package = "plonk-fibonacci", git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git" }

[build-dependencies]
mopro-ffi = { git = "https://github.com/zkmopro/mopro.git", branch = "main" }
uniffi = { version = "0.28", features = ["build"] }
```

Next, copy your SRS and key files into the project folder. For this tutorial, we'll assume you place them in `test-vectors/halo2`.

:::info
Download example SRS and key files :

-   [plonk_fibonacci_srs.bin](https://github.com/zkmopro/mopro/blob/dfb9b286c63f6b418fe27465796c818996558bf7/test-vectors/halo2/plonk_fibonacci_srs.bin)
-   [plonk_fibonacci_pk.bin](https://github.com/zkmopro/mopro/blob/dfb9b286c63f6b418fe27465796c818996558bf7/test-vectors/halo2/plonk_fibonacci_pk.bin)
-   [plonk_fibonacci_vk.bin](https://github.com/zkmopro/mopro/blob/dfb9b286c63f6b418fe27465796c818996558bf7/test-vectors/halo2/plonk_fibonacci_vk.bin)

:::

Now, add three rust files, just as in the Circom-based Rust project setup.

Update the `./src/lib.rs` file to look like the following:

```rust
// Here we're calling a macro exported with Uniffi. This macro will
// write some functions and bind them to FFI type.
mopro_ffi::app!();

mopro_ffi::set_halo2_circuits! {
    ("plonk_fibonacci_pk.bin", plonk_fibonacci::prove, "plonk_fibonacci_vk.bin", plonk_fibonacci::verify),
}
```

Similar to the Circom-based Rust setup, add `ios.rs` and `android.rs` for binary execution.

Create the file `src/bin/ios.rs` as shown below:

```rust
fn main() {
    // A simple wrapper around a build command provided by mopro.
    // In the future this will likely be published in the mopro crate itself.
    mopro_ffi::app_config::ios::build();
}
```

Create another file `src/bin/android.rs` as shown below:

```rust
fn main() {
    // A simple wrapper around a build command provided by mopro.
    // In the future this will likely be published in the mopro crate itself.
    mopro_ffi::app_config::android::build();
}
```

Now you're ready to build your static library! You should be able to run either

```sh
cargo run --bin ios
```

or

```sh
cargo run --bin android
```
