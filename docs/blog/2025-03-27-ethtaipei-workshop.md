---
slug: 2025-ethtaipei-workshop
title: 2025 ETHTaipei Workshop
authors:
    name: Vivian Jeng
    title: Developer on the Mopro Team
    url: https://github.com/vivianjeng
    image_url: https://github.com/vivianjeng.png
tags: [workshop]
---

## Overview

This tutorial guides developers through getting started with Mopro and building a native mobile app from scratch. It covers

-   Setting up an example [multiplier2](https://github.com/zkmopro/circuit-registry/blob/main/multiplier2/multiplier2.circom) Circom circuit

    -   Starting from [0. Prerequisites](#0-prerequisites)

-   Modifying it to use a different circuit, such as [keccak256](https://github.com/zkmopro/circuit-registry/blob/main/keccak256/keccak256_256_test.circom)

    -   Starting from [6. Prerequisites](#6-update-circuits)

-   Additionally, we'll integrate the [semaphore-rs](https://github.com/worldcoin/semaphore-rs) Rust crate to generate native bindings and run the implementation on both iOS and Android.

:::info
This is a workshop tutorial from [ETHTaipei](https://ethtaipei.org/) 2025 in April. If you'd like to follow along and build a native mobile app, please check out this commit: [eab28f](https://github.com/zkmopro/mopro/tree/eab28f8e318ff0afc053c2c004c58afe2f34fdb7).
:::

## 0. Prerequisites

-   XCode or Android Studio
    -   If you're using Android Studio, ensure that you follow the [Android configuration](https://zkmopro.org/docs/prerequisites/#android-configuration) steps and set the `ANDROID_HOME` environment variable.
-   Rust and CMake

:::info
Documentation: https://zkmopro.org/docs/prerequisites
:::

## 1. Download Mopro CLI tool

We offer a convenient command-line tool called `mopro` to streamline the development process. It functions similarly to tools like `npx create-react-app` or Foundry, enabling developers to get started quickly and efficiently.

```sh
git clone https://github.com/zkmopro/mopro
cd mopro/cli
cargo install --path .
cd ../..
```

## 2. Initialize a project with Mopro CLI

The `mopro init` command helps you create a Rust project designed to generate bindings for both iOS and Android.
This step is similar to running `npx create-react-app`, so select the directory where you want to create your new app.

```sh
mopro init
```

Start by selecting a name for your project (default: `mopro-example-app`).

Next, choose the proving system that best fits your needs—Mopro currently supports both `circom` and `halo2`. For this example, we’ll be using `circom`.

![mopro init](/img/mopro-init.jpg)

Next, navigate to your project directory by running:

```sh
cd mopro-example-app
```

## 3. Build Rust bindings with mopro CLI

`mopro build` command can help developers build binaries for mobile targets (e.g. iOS and Android devices).

```sh
mopro build
```

-   Choose `debug` for faster builds during development or `release` for optimized performance in production.
-   Select the platforms you want to build for: `ios`, `android`, `web`.
-   Select the architecture for each platform:

    -   **iOS**:

<table>
<thead>
<tr>
<th>Architecture</th>
<th>Description</th>
<th>Suggested</th>
</tr>
</thead>
<tbody>
<tr>
<td><code>aarch64-apple-ios</code></td>
<td>For physical iOS devices</td>
<td>✅</td>
</tr>
<tr>
<td><code>aarch64-apple-ios-sim</code></td>
<td>For M-series Mac simulators</td>
<td>✅</td>
</tr>
<tr>
<td><code>x86_64-apple-ios</code></td>
<td>For Intel-based Mac simulators</td>
<td>-</td>
</tr>
</tbody>
</table>
    -   **Android**:

<table>
<thead>
<tr>
<th>Architecture</th>
<th>Description</th>
<th>Suggested</th>
</tr>
</thead>
<tbody>
<tr>
<td><code>x86_64-linux-android</code></td>
<td>For 64-bit Intel architecture (x86_64)</td>
<td>✅</td>
</tr>
<tr>
<td><code>i686-linux-android</code></td>
<td>For 32-bit Intel architecture (x86)</td>
<td>-</td>
</tr>
<tr>
<td><code>armv7-linux-androideabi</code></td>
<td>For 32-bit ARM architecture (ARMv7-A)</td>
<td>-</td>
</tr>
<tr>
<td><code>aarch64-linux-android</code></td>
<td>For 64-bit ARM architecture (ARMv8-A)</td>
<td>✅</td>
</tr>
</tbody>
</table>

![mopro build](/img/mopro-build.jpg)

:::warning
The build process may take a few minutes to complete.
:::

Next, you will see the following instructions displayed:

![mopro-build-finish](/img/mopro-build-finish.jpg)

## 4. Create templates for mobile development

`mopro create` command generates templates for various platforms and integrates bindings into the specified directories.

```sh
mopro create
```

Currently supported platforms:

-   iOS (Xcode project)
-   Android (Android Studio project)
-   React Native
-   Flutter
-   Web

After running the `mopro create` command, a new folder will be created in the current directory, such as:

-   `ios`
-   `android`
-   `react-native`
-   `flutter`
-   `web` (currently does not support Circom prover)

You will then see the following instructions to open the project:

![mopro-create](/img/mopro-create.jpg)

If you want to create multiple templates, simply run `mopro create` again and select a different framework each time.

![mopro-create-android](/img/mopro-create-android.jpg)

## 5. Run the app on a device/simulator

### iOS

Open the Xcode project by running the following command:

```sh
open ios/MoproApp.xcodeproj
```

Select the target device and run the project by pressing `cmd` + `R`.

Alternatively, you can watch this video to see how to run the app.

<p align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/6TydXwYMQCU?si=9s3B8OVa_eRLzchU&amp;start=100" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</p>

### Android

Open the Android Studio project by running the following command:

```sh
open android -a Android\ Studio
```

Run the project by pressing `^` + `R` or `ctrl` + `R`.

Alternatively, you can watch this video to see how to run the app.

<p align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/r6WolEEHuMw?si=Lrvkxt03fnqwaYlK&amp;start=196" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</p>

## 6. Update circuits

This section explains how to update circuits with alternative witness generators and corresponding zkey files. We use the [Keccak256 circuit](https://github.com/zkmopro/circuit-registry/blob/main/keccak256/keccak256_256_test.circom) as a reference example here.

1.  Add wasm and zkey file in the `test-vectors/circom` folder
    -   wasm: https://ci-keys.zkmopro.org/keccak256_256_test.wasm
    -   zkey: https://ci-keys.zkmopro.org/keccak256_256_test_final.zkey
2.  In `src/lib.rs` file, update the circuit's witness generator function definition.

    ```diff
    -  rust_witness::witness!(multiplier2);
    +  rust_witness::witness!(keccak256256test);

    mopro_ffi::set_circom_circuits! {
    -    ("multiplier2_final.zkey", WitnessFn::RustWitness(multiplier2_witness))
    +    ("keccak256_256_test_final.zkey", WitnessFn::RustWitness(keccak256256test_witness))
    }
    ```

    :::warning
    The name should match the lowercase version of the WASM file, with all special characters removed.<br/>
    e.g.<br/>
    `multiplier2` -> `multiplier2`<br/>
    `keccak_256_256_main` -> `keccak256256main`<br/>
    `aadhaar-verifier` -> `aadhaarverifier`
    :::

3.  Similar to [Step 3](#3-build-rust-bindings-with-mopro-cli), regenerate the bindings to reflect the updated circuit.
    ```sh
    mopro build
    ```
4.  Manually update the bindings in the app by replacing the existing ones.

    -   **iOS:**
        -   Replace `ios/MoproiOSBindings` with `MoproiOSBindings`.
    -   **Android:**

        -   Replace `android/app/src/main/jniLibs` with `MoproAndroidBindings/jniLibs`

        -   Replace `android/app/src/main/java/uniffi` with `MoproAndroidBindings/uniffi`

    :::info
    We aim to provide the `mopro update` CLI tool to assist with updating bindings.
    Contributions to this effort are welcome.https://github.com/zkmopro/mopro/issues/269
    :::

5.  Copy zkeys to assets

    -   **iOS:**<br/>
        Open Xcode, drag in the zkeys you plan to use for proving, then navigate to the project’s **Build Phases**. Under **Copy Bundle Resources**, add each zkey to ensure it's included in the app bundle.

        Alternatively, you can watch this video to see how to copy zkey in XCode.

            <p align="center">
            <iframe width="560" height="315" src="https://www.youtube.com/embed/6TydXwYMQCU?si=QvOxlfbOOyX3GpcM&amp;start=53" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
            </p>

    -   **Android**<br/>
        Paste the zkey in the assets folder: `android/app/src/main/assets`.

        Alternatively, you can watch this video to see how to copy zkey in XCode.

            <p align="center">
            <iframe width="560" height="315" src="https://www.youtube.com/embed/r6WolEEHuMw?si=aOs7SPz4ajtE1hMP&amp;start=141" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
            </p>

6.  Update circuit input and zkey path

-   Update `zkeyPath` to `keccak256_256_test_final`

    -   **iOS:**<br/>

    ```diff
    - private let zkeyPath = Bundle.main.path(forResource: "multiplier2_final", ofType: "zkey")!
    + private let zkeyPath = Bundle.main.path(forResource: "keccak256_256_test_final", ofType: "zkey")!
    ```

    -   **Android:**<br/>

    ```diff
    - val zkeyPath = getFilePathFromAssets("multiplier2_final.zkey")
    + val zkeyPath = getFilePathFromAssets("keccak256_256_test_final.zkey")
    ```

-   Update circuit inputs: https://ci-keys.zkmopro.org/keccak256.json

    -   **iOS:**<br/>

    ```diff
    - let input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"
    + let input_str: String = "{\"in\":[\"0\",\"0\",\"1\",\"0\",\"1\",\"1\",\"1\",\"0\",\"1\",\"0\",\"1\",\"0\",\"0\",\"1\",\"1\",\"0\",\"1\",\"1\",\"0\",\"0\",\"1\",\"1\",\"1\",\"0\",\"0\",\"0\",\"1\",\"0\",\"1\",\"1\",\"1\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\"]}"
    ```

    -   **Android:**<br/>

    ```diff
    - val input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"
    + val input_str: String = "{\"in\":[\"0\",\"0\",\"1\",\"0\",\"1\",\"1\",\"1\",\"0\",\"1\",\"0\",\"1\",\"0\",\"0\",\"1\",\"1\",\"0\",\"1\",\"1\",\"0\",\"0\",\"1\",\"1\",\"1\",\"0\",\"0\",\"0\",\"1\",\"0\",\"1\",\"1\",\"1\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\",\"0\"]}"
    ```

7. Then Run the app again like in [step 5](#5-run-the-app-on-a-devicesimulator).

## 7. Update Rust exported functions

Currently, only `generateCircomProof` and `verifyCircomProof` are available with the bindings, but the bindings can be extended to support nearly all Rust functions.

Here is an example demonstrating how to use the Semaphore crate.

1. Update `Cargo.toml`

    Import semaphore crate from: https://github.com/worldcoin/semaphore-rs

    ```toml title="Cargo.toml"
    semaphore-rs = { git = "https://github.com/worldcoin/semaphore-rs", features = ["depth_16"]}
    ```

2. Define a function to generate semaphore proof

    [Here](https://github.com/chengggkk/Zuma/blob/master/src/lib.rs) is an example to define semaphore `prove` and `verify` in `src/lib.rs`

    Alternatively, we can create a demo function called `semaphore()` to run the code from the [README](https://github.com/worldcoin/semaphore-rs?tab=readme-ov-file#example).

    ```rust title="src/lib.rs"
    use semaphore_rs::{get_supported_depths, hash_to_field, Field, identity::Identity,
                poseidon_tree::LazyPoseidonTree, protocol::*};
    use num_bigint::BigInt;

    pub fn semaphore() {
        // generate identity
        let mut secret = *b"secret";
        let id = Identity::from_secret(&mut secret, None);

        // Get the first available tree depth. This is controlled by the crate features.
        let depth = get_supported_depths()[0];

        // generate merkle tree
        let leaf = Field::from(0);
        let mut tree = LazyPoseidonTree::new(depth, leaf).derived();
        tree = tree.update(0, &id.commitment());

        let merkle_proof = tree.proof(0);
        let root = tree.root();

        // change signal and external_nullifier here
        let signal_hash = hash_to_field(b"xxx");
        let external_nullifier_hash = hash_to_field(b"appId");

        let nullifier_hash = generate_nullifier_hash(&id, external_nullifier_hash);

        let proof = generate_proof(&id, &merkle_proof, external_nullifier_hash, signal_hash).unwrap();
        let success = verify_proof(root, nullifier_hash, signal_hash, external_nullifier_hash, &proof, depth).unwrap();

        assert!(success);
    }
    ```

    :::warning
    You can also try returning a value; otherwise, nothing will happen after execution.
    e.g.

    ```rust
    pub fn semaphore() -> bool {
        // ...
        return success
    }
    ```

    :::

3. Export the function through UniFFI procedural macros

    You can simply use the UniFFI proc-macros (e.g. `#[uniffi::export]`) to define the function interfaces.

    :::info
    For more details, refer to the [UniFFI documentation](https://mozilla.github.io/uniffi-rs/latest/proc_macro/index.html).
    :::

    ```diff
    + #[uniffi::export]
    pub fn semaphore() {
        // generate identity
        let mut secret = *b"secret";
        ...
    ```

4. Run `mopro build` again and manually update the bindings for iOS and Android as explained in [Step 6](#6-update-circuits).

5. You can now call the `semaphore()` function you just defined on both iOS and Android. 🎉

## 8. Conclusion

-   By following the tutorial, you will learn to create a native mobile ZK app with:
    -   A simple circuit
    -   Custom circuits
    -   Custom functions and structs
-   Just like with the Semaphore case, this approach can be extended to any Rust crate, as long as you define the input and output types according to the [UniFFI documentation](https://mozilla.github.io/uniffi-rs/latest/proc_macro/index.html).
-   Alternative platforms, such as wasm for web, and frameworks like React Native and Flutter are also supported. Please check out:

    -   https://github.com/zkmopro/react-native-app
    -   https://github.com/zkmopro/flutter-app

    Or simply run `mopro build` for these frameworks.

-   There are still many challenges to address, and contributions are highly encouraged. Feel free to explore the issues list.
    -   https://github.com/zkmopro/mopro/issues
    -   https://github.com/zkmopro/gpu-acceleration/issues
-   We encourage developers to build mobile apps, as it helps us enhance the developer experience and gain a deeper understanding of the challenges involved.
