# Rust Setup for Android/iOS Bindings

This tutorial guides you through building iOS and Android bindings from a Rust project. It’s divided into three sections:

-   [**🌎 General setup**](#-general-setup) – Use `mopro-ffi` to generate native bindings from your Rust project.
-   [**🔐 Integrating a ZK prover**](#-integrating-a-zk-prover) – Add support for Circom, Halo2, and/or Noir.
-   [**🦄 customize the bindings**](#-customize-the-bindings) using any Rust crate, making it easy to expose your logic to mobile platforms.

If you want to quickly get started with Circom `.wasm` and `.zkey` files, refer to the

-   [**📋 Generating Circom Bindings**](#-generate-circom-bindings) section.

:::info
Make sure you've installed the [prerequisites](/docs/prerequisites).
:::

---

## 🌎 General setup

### 0. Initialize a new project

If you already have a Rust project, skip ahead to [1. Add dependencies](#1-add-dependencies).

If you don’t have an existing Rust project, start by initializing a new Rust crate with creating a new directory (e.g., `mopro-rust-project`) and change into it:

```sh
mkdir mopro-rust-project
```

```sh
cd mopro-rust-project
```

Initialize a new Rust crate by running:

```sh
cargo init --lib
```

### 1. Add dependencies

Include the crate in your `Cargo.toml`

```toml title="Cargo.toml"
[features]
default = ["uniffi"]
uniffi = ["mopro-ffi/uniffi"]
flutter = ["mopro-ffi/flutter"]

[dependencies]
mopro-ffi = "0.3"
thiserror = "2.0.12"

[build-dependencies]
mopro-ffi = "0.3"
```

### 2. Setup the lib

Define the `crate-type` for the [UniFFI](https://mozilla.github.io/uniffi-rs/latest/) build process configuration.

```toml title="Cargo.toml"
[lib]
crate-type = ["lib", "cdylib", "staticlib"]
```

### 3. Use `mopro-ffi` macro

The `mopro-ffi` macro exports several FFI configurations. To enable it, activate the `mopro-ffi` macro in `src/lib.rs`.

```rust title="src/lib.rs"
mopro_ffi::app!();
```

### 4. Define the binaries

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

You can also apply this to Flutter and React Native.

```rust title="src/bin/react_native.rs"
fn main() {
    mopro_ffi::app_config::react_native::build();
}
```

```rust title="src/bin/flutter.rs"
fn main() {
    mopro_ffi::app_config::flutter::build();
}
```

### 5. What's next

You have two options moving forward:

-   [**Integrate a supported prover**](#-integrating-a-zk-prover) – Choose one of the built-in options: Circom, Halo2, or Noir (see the next section).
-   [**Customize your bindings**](#-customize-the-bindings) – If you're using a different prover or want to expose your own business logic, skip ahead to the Customize Bindings section.

---

## 🔐 Integrating a ZK prover

Be sure to complete the [general setup](#-general-setup) before continuing.

This tutorial will cover:

1. [Circom prover](#setup-circom-based-rust-project)
2. [Halo2 prover](#setup-halo2-based-rust-project)
3. [Noir prover](#setup-noir-based-rust-project)

:::info
If you’re starting from scratch, the setup process is very similar to using the mopro CLI. Please refer to the [Getting Started](/docs/getting-started) guide for detailed instructions.
:::

## Setup Circom-Based rust project

📢 Ensure you've completed the [General Setup](#-general-setup) before proceeding.

### 1. Add witness generator

Each witness generator must be built within a project. You need to supply the required data for it to generate a circuit-specific execution function.

Here, we used [rust-witness](https://github.com/chancehudson/rust-witness) as an example.

:::info
To learn more about `witnesscalc` and `circom-witnesscalc` for Mopro, please check out [circom-prover](https://github.com/zkmopro/mopro/blob/main/circom-prover/README.md#advanced-usage).
:::

Include the `rust-witness` in your Cargo.toml and add `circom-prover` in `Cargo.toml`:

```toml title="Cargo.toml"
[dependencies]
circom-prover = "0.1"
rust-witness  = "0.1"
num-bigint    = "0.4.0"

[build-dependencies]
rust-witness = "0.1"
```

In `build.rs`, add the following code to compile the witness generator wasm sources (`.wasm`) into a native library and link to it:

```rust title="build.rs"
fn main() {
    rust_witness::transpile::transpile_wasm("../path to directory containing your wasm sources");
    // e.g. rust_witness::transpile::transpile_wasm("./test-vectors".to_string());
    // The directory should contain the following files:
    // - <circuit name>.wasm
}
```

:::info
Learn more about `.wasm` files in [Circom documentation](https://docs.circom.io/getting-started/compiling-circuits/). <br/>
Here are the example WASM and Zkey files to be downloaded.

-   http://ci-keys.zkmopro.org/multiplier2.wasm
-   http://ci-keys.zkmopro.org/multiplier2_final.zkey

:::

### 2. Add the Helper Template

Use the Circom helper template by adding a `src/circom.rs` file based on this example: [circom.rs](https://github.com/zkmopro/mopro/blob/23265fe5b14154d023278742ffe6289eb629014a/cli/src/template/init/src/circom.rs).
This template exposes the Circom prover interface through FFIs and makes it available to mobile native packages.

And add an error handler `src/error.rs` for the mobile native bindings based on this example: [error.rs](https://github.com/zkmopro/mopro/blob/23265fe5b14154d023278742ffe6289eb629014a/cli/src/template/init/src/error.rs).

Enable the `circom` and `error` module in `src/lib.rs`

```rust title="src/lib.rs"
mod error;
pub use error::MoproError;
mod circom;
pub use circom::{
    generate_circom_proof, verify_circom_proof, CircomProof, CircomProofResult, ProofLib, G1, G2,
};
```

Then, use the witness function to associate it with a `.zkey` file. This allows the proof generation function to know which witness function to use when generating the proof.

```rust title="src/lib.rs"
// Activate rust-witness function
mod witness {
    rust_witness::witness!(multiplier2);
}

// Set the witness functions to a zkey
set_circom_circuits! {
    (
        "multiplier2_final.zkey",
        circom_prover::witness::WitnessFn::RustWitness(witness::multiplier2_witness)
    ),
}
```

:::info
To ensure the circuit is correctly integrated with Rust, consider writing unit tests to verify the bindings.

```rust title="src/lib.rs"
#[cfg(test)]
mod circom_tests {
    use super::*;

    #[test]
    fn test_multiplier2() {
        let zkey_path = "./test-vectors/circom/multiplier2_final.zkey".to_string();
        let circuit_inputs = "{\"a\": 2, \"b\": 3}".to_string();
        let result = generate_circom_proof(zkey_path.clone(), circuit_inputs, ProofLib::Arkworks);
        assert!(result.is_ok());
        let proof = result.unwrap();
        let valid = verify_circom_proof(zkey_path, proof, ProofLib::Arkworks);
        assert!(valid.is_ok());
        assert!(valid.unwrap());
    }
}
```

:::

### 3. Generate bindings for iOS and Android

Now you're ready to build your static library! You should be able to run either the Mopro CLI or the binaries.

**1. Execute the process through Mopro CLI (recommended 👍🏻)**

To install the Mopro CLI, please refer to the [Getting Started](/docs/getting-started) guide.

Execute the building through

```sh
mopro build
```

Then, you can select the target mode and architectures with greater flexibility.

**2. Execute the process using the defined binaries.**

For example:

```sh
cargo run --bin ios # Debug mode for iOS
cargo run --bin android # Debug mode for Android
CONFIGURATION=release cargo run --bin ios # Release mode for iOS
CONFIGURATION=release cargo run --bin android # Release mode for Android
ANDROID_ARCHS=x86_64-linux-android cargo run --bin android # Build for Android x86_64-linux-android architecture
IOS_ARCHS=aarch64-apple-ios,aarch64-apple-ios-sim cargo run --bin ios # Build for iOS aarch64-apple-ios and aarch64-apple-ios-sim architecture
```

to build the corresponding static library.

:::info
Running your project in `release` mode significantly enhances performance compared to `debug` mode. This is because the Rust compiler applies optimizations that improve runtime speed and reduce binary size, making your application more efficient.
:::

### 4. What's next

Once the bindings are successfully built, you will see the `MoproiOSBindings`, `MoproAndroidBindings`, `MoproReactNativeBindings` and/or `mopro_flutter_bindings` folders.

Next, you have two options:

-   Use the `mopro create` command from the mopro CLI to generate ready-to-use templates for your desired framework (e.g., Swift, Kotlin, React Native, or Flutter).
-   If you already have a mobile app or prefer to manually integrate the bindings, follow the [iOS Setup](ios-setup.md), [Android Setup](android-setup.md), [React Native Setup](react-native-setup.md) and/or [FLutter Setup](flutter-setup.md) sections.
-   If you find that some functionality is still missing on mobile, you can refer to the [Customize the Bindings](#-customize-the-bindings) section to learn how to expose additional functions using any Rust crate.
-   After making changes, be sure to run:
    ```sh
    mopro build
    mopro update
    ```
    This ensures the bindings are regenerated and reflect your latest updates.

:::warning
If you haven’t integrated other adapters and encounter an error like:

```sh
Cannot find 'generateHalo2Proof' in scope
```

you can fix it by adding the [stubs.rs](https://github.com/zkmopro/mopro/blob/23265fe5b14154d023278742ffe6289eb629014a/cli/src/template/init/src/stubs.rs) file in `src/stubs.rs` and activating the stubs in `src/lib.rs`:

```rust title="src/lib.rs"
#[macro_use]
mod stubs;

// Activate stubs for adapters that are not integrated
halo2_stub!();
noir_stub!();
```

This ensures that missing adapter functions are stubbed, preventing compilation errors while keeping your project runnable.

:::

---

## Setup Halo2-Based rust project

📢 Ensure you've completed the [General Setup](#-general-setup) before proceeding.

### 1. Add Halo2 circuits

Import the Halo2 prover as a Rust crate using:

```toml title="Cargo.toml"
[dependencies]
plonk-fibonacci = { package = "plonk-fibonacci", git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git" }
anyhow = "1.0.99"
```

:::info
See how to define a Halo2 prover crate and generate SRS here: [plonkish-fibonacci-sample](https://github.com/sifnoc/plonkish-fibonacci-sample)
:::

Next, copy your SRS and key files into the project folder. For this tutorial, we'll assume you place them in `test-vectors/halo2`.

:::info
Download example SRS and key files :

-   [plonk_fibonacci_srs.bin](https://github.com/zkmopro/mopro/blob/dfb9b286c63f6b418fe27465796c818996558bf7/test-vectors/halo2/plonk_fibonacci_srs.bin)
-   [plonk_fibonacci_pk.bin](https://github.com/zkmopro/mopro/blob/dfb9b286c63f6b418fe27465796c818996558bf7/test-vectors/halo2/plonk_fibonacci_pk.bin)
-   [plonk_fibonacci_vk.bin](https://github.com/zkmopro/mopro/blob/dfb9b286c63f6b418fe27465796c818996558bf7/test-vectors/halo2/plonk_fibonacci_vk.bin)

Now, add three key files, for example, in `test-vectors/halo2` folder.

:::

### 2. Add the Helper Template

Use the Halo2 helper template by adding a `src/halo2.rs` file based on this example: [halo2.rs](https://github.com/zkmopro/mopro/blob/23265fe5b14154d023278742ffe6289eb629014a/cli/src/template/init/src/halo2.rs).
This template exposes the Halo2 prover interface through FFIs and makes it available to mobile native packages.

And add an error handler `src/error.rs` for the mobile native bindings based on this example: [error.rs](https://github.com/zkmopro/mopro/blob/23265fe5b14154d023278742ffe6289eb629014a/cli/src/template/init/src/error.rs).

Enable the `halo2` and `error` module in `src/lib.rs`

```rust title="src/lib.rs"
mod error;
pub use error::MoproError;
mod halo2;
pub use halo2::{generate_halo2_proof, verify_halo2_proof, Halo2ProofResult};

```

Then, associate each key file with its corresponding execution functions. This ensures the proof generation function knows which function to use when generating a proof.

```rust title="src/lib.rs"
set_halo2_circuits! {
    ("plonk_fibonacci_pk.bin", plonk_fibonacci::prove, "plonk_fibonacci_vk.bin", plonk_fibonacci::verify),
}
```

:::info
To ensure the circuit is correctly integrated with Rust, consider writing unit tests to verify the bindings.

```rust title="src/lib.rs"
#[cfg(test)]
mod halo2_tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_plonk_fibonacci() {
        let srs_path = "./test-vectors/halo2/plonk_fibonacci_srs.bin".to_string();
        let pk_path = "./test-vectors/halo2/plonk_fibonacci_pk.bin".to_string();
        let vk_path = "./test-vectors/halo2/plonk_fibonacci_vk.bin".to_string();
        let mut circuit_inputs = HashMap::new();
        circuit_inputs.insert("out".to_string(), vec!["55".to_string()]);
        let result = generate_halo2_proof(srs_path.clone(), pk_path.clone(), circuit_inputs);
        assert!(result.is_ok());
        let halo2_proof_result = result.unwrap();
        let valid = verify_halo2_proof(
            srs_path,
            vk_path,
            halo2_proof_result.proof,
            halo2_proof_result.inputs,
        );
        assert!(valid.is_ok());
        assert!(valid.unwrap());
    }
}
```

:::

### 3. Generate bindings for iOS and Android

Similar to Circom-based Rust project setup [3. Generate bindings for iOS and Android](#3-generate-bindings-for-ios-and-android)

### 4. What's next

Similar to Circom-based Rust project setup [4. What's next](#4-whats-next)

---

## Setup Noir-Based rust project

📢 Ensure you've completed the [General Setup](#-general-setup) before proceeding.

### 0. Prepare Noir circuits

Follow the [Noir documentation](https://noir-lang.org/docs/dev) to build your circuit.
You’ll need to generate a `.json` file from the compiled circuit for use in this project.

Downloading the SRS (Structured Reference String) is optional but recommended, as it can significantly improve proving performance.
See [Downloading SRS](https://github.com/zkmopro/noir-rs?tab=readme-ov-file#downloading-srs-structured-reference-string) for more details.

### 1. Enable `noir` feature

```toml title="Cargo.toml"
[dependencies]
mopro-ffi = { git = "https://github.com/zkmopro/mopro", features = ["noir"] } # enable noir feature
```

:::warning
Noir and its dependencies are not yet published as crates on crates.io. Therefore, we can only import them directly from GitHub.
:::

You’re now ready to generate a Noir proof using mopro-ffi.

:::info
Download example SRS and circuit files :

-   [noir_multiplier2.json](https://github.com/zkmopro/mopro/blob/3fb330e31f6111ca0cc0aa0a408a491c239ccb93/test-vectors/noir/noir_multiplier2.json)
-   [noir_multiplier2.srs](https://github.com/zkmopro/mopro/blob/3fb330e31f6111ca0cc0aa0a408a491c239ccb93/test-vectors/noir/noir_multiplier2.srs)

:::

:::info
To verify everything is working correctly, you can write a Rust unit test like the one below to ensure the proof is computed successfully.

```rust title="src/lib.rs"
#[cfg(test)]
mod noir_tests {
    use super::*;

    #[test]
    fn test_noir_multiplier2() {
        let srs_path = "./test-vectors/noir/noir_multiplier2.srs".to_string();
        let circuit_path = "./test-vectors/noir/noir_multiplier2.json".to_string();
        let circuit_inputs = vec!["3".to_string(), "5".to_string()];
        let result = generate_noir_proof(
            circuit_path.clone(),
            Some(srs_path.clone()),
            circuit_inputs.clone(),
        );
        assert!(result.is_ok());
        let proof = result.unwrap();
        let valid = verify_noir_proof(circuit_path.clone(), proof);
        assert!(valid.is_ok());
        assert!(valid.unwrap());
    }
}
```

:::

### 2. Generate bindings for iOS and Android

Similar to Circom-based Rust project setup [3. Generate bindings for iOS and Android](#3-generate-bindings-for-ios-and-android)

### 3. What's next

Similar to Circom-based Rust project setup [4. What's next](#4-whats-next)

---

## 🦄 Customize the bindings

📢 Ensure you've completed the [General Setup](#-general-setup) before proceeding.

If your ZK prover isn’t included, or you already have your own Rust crate,
you can use the `#[uniffi::export]` procedural macro to define your own functions and generate mobile-native bindings from them.

### 1. Define exported functions

Export Rust functions using a procedural macro, as shown below:

```rust title="src/lib.rs"
mopro_ffi::app!(); // Enable the mopro-ffi macro to generate UniFFI scaffolding.

#[uniffi::export]
pub fn hello_world() -> String {
    "Hello, World!".to_string()
}
```

:::info
For more examples and detailed references, check the [UniFFI documentation](https://mozilla.github.io/uniffi-rs/latest).
:::

You may also bring in other Rust crates to extend functionality.

For instance, to use `semaphore-rs`, you can add it like this:

```toml title="Cargo.toml"
[dependencies]
semaphore-rs = { git = "https://github.com/semaphore-protocol/semaphore-rs", features = ["serde"] }
```

```rust title="src/lib.rs"
mopro_ffi::app!(); // Enable the mopro-ffi macro to generate UniFFI scaffolding.

use semaphore_rs::group::Group;
use semaphore_rs::identity::Identity;
use semaphore_rs::proof::GroupOrMerkleProof;
use semaphore_rs::proof::Proof;
use semaphore_rs::proof::SemaphoreProof;

#[uniffi::export]
fn semaphore_prove(
    id_secret: String,
    leaves: Vec<Vec<u8>>,
    message: String,
    scope: String,
    tree_depth: u16,
) -> String {
    let identity = Identity::new(id_secret.as_bytes());
    let group_members: Vec<[u8; 32]> = leaves
        .iter()
        .map(|leaf| {
            leaf.as_slice()
                .try_into()
                .expect("Leaf must be exactly 32 bytes")
        })
        .collect::<Vec<[u8; 32]>>();
    let group = Group::new(&group_members).unwrap();
    let proof = Proof::generate_proof(
        identity,
        GroupOrMerkleProof::Group(group),
        message.to_string(),
        scope.to_string(),
        tree_depth,
    )
    .unwrap();
    let proof_json = proof.export().unwrap();
    return proof_json;
}

#[uniffi::export]
fn semaphore_verify(proof: String) -> bool {
    let proof = SemaphoreProof::import(&proof).unwrap();
    let valid = Proof::verify_proof(proof);
    valid
}
```

<details>
    <summary>Example usage</summary>
    ```rust
    #[cfg(test)]
    mod uniffi_tests {
        use super::*;
        use semaphore_rs::utils::to_element;

        #[test]
        fn test_mopro_uniffi_hello_world() {
            let secret1 = "secret1";
            let secret2 = "secret2";
            let identity1 = Identity::new(secret1.as_bytes());
            let identity2 = Identity::new(secret2.as_bytes());
            let leaves = vec![
                to_element(*identity1.commitment()).to_vec(),
                to_element(*identity2.commitment()).to_vec(),
            ];
            let message = "message";
            let scope = "scope";
            let tree_depth = 10;
            let proof = semaphore_prove(
                secret1.to_string(),
                leaves,
                message.to_string(),
                scope.to_string(),
                tree_depth,
            );
            assert!(semaphore_verify(proof));
        }
    }
    ```

</details>

### 2. Generate bindings for iOS and Android

Similar to Circom-based Rust project setup [3. Generate bindings for iOS and Android](#3-generate-bindings-for-ios-and-android)

Once the bindings are generated, you'll see your exported functions (e.g., `semaphoreProve`, `semaphoreVerify`) included in the generated code—for example, in `MoproiOSBindings/mopro.swift` for iOS and `MoproAndroidBindings/uniffi/mopro/mopro.kt` for Android.

You can then use these functions directly within your iOS and/or Android applications as part of the generated bindings.

## 📋 Generate Circom bindings

<!-- TODO: update mopro-cli package -->

Install the latest CLI from GitHub:

```sh
git clone https://github.com/zkmopro/mopro
cd mopro/cli
cargo install --path .
```

Use `mopro bindgen` to specify your circuit path and target architectures (e.g., iOS or Android):

-   Specify configuration directly in the CLI command

```sh
mopro bindgen
```

-   Specify circuit directory

```sh
mopro bindgen --circuit-dir ./test-vectors/circom
```

-   Specify circuit directory, build mode, and architectures

```sh
mopro bindgen \
  --circuit-dir ./test-vectors/circom \
  --mode release \
  --platforms ios \
  --architectures aarch64-apple-ios-sim
```

-   View all available options:

```sh
mopro bindgen --help
```

:::warning
Currently, only the [`rust-witness`](https://github.com/chancehudson/rust-witness) witness generator is supported.
:::

### 3. What's next

Similar to Circom-based Rust project setup [4. What's next](#4-whats-next)
