# Rust Setup

This tutorial will show you how to build static library with Circom adapter for Android and iOS. Later pages show how to integrate into an existing iOS or Android app.

Make sure you've installed the [prerequisites](/docs/prerequisites).

## Demo video of this tutorial

<p align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/aIi-11hga4c?si=cdNN4Ee9QE0ZBXVq" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</p>

## Setup the rust project

Mopro works by providing a static library and an interface for your app to build proofs. Before you start this tutorial you should have a zkey and wasm file generated by circom.

To get started we'll make a new rust project that builds this library. Run the following commands in your terminal:

```sh
mkdir mopro-example
cd mopro-example
cargo init --lib
```

This will create a new rust project in the current directory. Now we'll add some dependencies to this project. Edit your `Cargo.toml` so that it looks like the following:

```toml
[package]
name = "mopro-example"
version = "0.1.0"
edition = "2021"

# We're going to build a static library named mopro_bindings
# This library name should not be changed
[lib]
crate-type = ["lib", "cdylib", "staticlib"]
name = "mopro_bindings"

# We're going to build support for circom proofs only for this example
[features]
default = ["mopro-ffi/circom"]

[dependencies]
mopro-ffi = "0.1.0"
rust-witness = "0.1.0"
uniffi = { version = "0.28", features = ["cli"] }
num-bigint = "0.4.0"

[build-dependencies]
mopro-ffi = "0.1.0"
rust-witness = "0.1.0"
uniffi = { version = "0.28", features = ["build"] }

# TODO: fix this
[patch.crates-io]
ark-circom = { git = "https://github.com/zkmopro/circom-compat.git", version = "0.1.0", branch = "wasm-delete" }
```

Now you should copy your wasm and zkey files somewhere in the project folder. For this tutorial we'll assume you placed them in `test-vectors/circom`.

:::info
Download example multiplier2 wasm and zkey here:

-   [multiplier2.wasm](https://github.com/zkmopro/mopro/raw/ae88356e680ac4d785183267d6147167fabe071c/test-vectors/circom/multiplier2.wasm)
-   [multiplier2_final.zkey](https://github.com/zkmopro/mopro/raw/ae88356e680ac4d785183267d6147167fabe071c/test-vectors/circom/multiplier2_final.zkey)

:::

Now we need to add 4 rust files. First we'll add `build.rs` in the main project folder. This file should contain the following:

```rust
fn main() {
    // We're going to transpile the wasm witness generators to C
    // Change this to where you put your zkeys and wasm files
    rust_witness::transpile::transpile_wasm("./test-vectors/circom".to_string());
    // This is writing the UDL file which defines the functions exposed
    // to your app. We have pre-generated this file for you
    // This file must be written to ./src
    std::fs::write("./src/mopro.udl", mopro_ffi::app_config::UDL).expect("Failed to write UDL");
    // Finally initialize uniffi and build the scaffolding into the
    // rust binary
    uniffi::generate_scaffolding("./src/mopro.udl").unwrap();
}
```

Second we'll change the file at `./src/lib.rs` to look like the following:

```rust
// Here we're generating the C witness generator functions
// for a circuit named `multiplier2`.
// Your circuit name will be the name of the wasm file all lowercase
// with spaces, dashes and underscores removed
//
// e.g.
// multiplier2 -> multiplier2
// keccak_256_256_main -> keccak256256main
// aadhaar-verifier -> aadhaarverifier
rust_witness::witness!(multiplier2);

// Here we're calling a macro exported by uniffi. This macro will
// write some functions and bind them to the uniffi UDL file. These
// functions will invoke the `get_circom_wtns_fn` generated below.
mopro_ffi::app!();

// This macro is used to define the `get_circom_wtns_fn` function
// which defines a mapping between zkey filename and witness generator.
// You can pass multiple comma seperated `(filename, witness_function)` pairs to it.
// You can read in the `circom` doc section how you can manually set this function.
// One way to create the witness generator function is to use the `rust_witness!` above.
mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", multiplier2_witness),
}
```

Finally we'll add a new file at `src/bin/ios.rs`:

```rust
fn main() {
    // A simple wrapper around a build command provided by mopro.
    // In the future this will likely be published in the mopro crate itself.
    mopro_ffi::app_config::ios::build();
}
```

and another at `src/bin/android.rs`:

```rust
fn main() {
    // A simple wrapper around a build command provided by mopro.
    // In the future this will likely be published in the mopro crate itself.
    mopro_ffi::app_config::android::build();
}
```

Now you're ready to build your static library! You should be able to run either

```sh
cargo run --bin ios # Debug mode
```

or

```sh
cargo run --bin android # Debug mode
```

to build the corresponding static library. Move on to [iOS setup](ios-setup) or [Android setup](android-setup) to begin integrating in an app.

:::info
To achieve optimal performance, it's recommended to build and run your application in release mode. You can do this by setting the `CONFIGURATION` environment variable to `release` before running your commands.

Example usage

```sh
CONFIGURATION=release cargo run --bin ios # Release mode
```

or

```sh
CONFIGURATION=release cargo run --bin android # Release mode
```

Running your project in release mode significantly enhances performance compared to debug mode. This is because the Rust compiler applies optimizations that improve runtime speed and reduce binary size, making your application more efficient.

:::