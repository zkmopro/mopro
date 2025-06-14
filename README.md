<p align="center">
<img src="img/logo_title.svg">
</p>

<p align="center">
    <a href="https://github.com/zkmopro" target="_blank">
        <img src="https://img.shields.io/badge/project-Mopro-blue.svg?style=flat-square">
    </a>
    <a href="/LICENSE">
        <img alt="Github license" src="https://img.shields.io/github/license/zkmopro/mopro.svg?style=flat-square">
    </a>
    <a href="https://github.com/zkmopro/mopro/actions?query=workflow%3Aproduction">
        <img alt="GitHub Workflow test" src="https://img.shields.io/github/actions/workflow/status/zkmopro/mopro/build-and-test.yml?branch=main&label=test&style=flat-square&logo=github">
    </a>
    <img alt="Repository top language" src="https://img.shields.io/github/languages/top/zkmopro/mopro?style=flat-square">
    <a href="http://commitizen.github.io/cz-cli/">
        <img alt="Commitizen friendly" src="https://img.shields.io/badge/commitizen-friendly-586D76?style=flat-square">
    </a>
    <a href="https://twitter.com/zkmopro">
        <img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro">
    </a>
    <a href="https://t.me/zkmopro">
        <img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram">
    </a>
</p>

## Mopro: ZK Toolkit for Mobile

Mopro (Mobile Prover) is a toolkit for ZK app development on mobile. Mopro makes client-side proving on mobile simple.

📖 To learn more about mopro, please refer to the documentation at [zkmopro](https://zkmopro.org/docs/intro).

## Repository Structure

This repository contains the following components:

<table>
    <th>Package</th>
    <th>Version</th>
    <th>Description</th>
    <tbody>
        <tr>
            <td>
                <a href="https://github.com/zkmopro/mopro/tree/main/mopro-ffi">
                    mopro-ffi
                </a>
            </td>
            <td>
                <a href="https://crates.io/crates/mopro-ffi">
                    <img src="https://img.shields.io/crates/v/mopro-ffi?label=mopro-ffi&style=flat-square">
                </a>
            </td>
            <td>
                Uses <a href="https://github.com/mozilla/uniffi-rs">UniFFI</a> to generate bindings for ZK provers (currently: Circom, Halo2, and Noir). It includes build scripts that eliminate the need for manual setup when integrating with iOS and Android.
            </td>
        </tr>
        <tr>
            <td>
                <a href="https://github.com/zkmopro/mopro/tree/main/mopro-wasm">
                    mopro-wasm
                </a>
            </td>
            <td> WIP
            </td>
            <td>
                Uses <a href="https://github.com/rustwasm/wasm-bindgen">wasm-bindgen</a> to generate bindings for web environments, with rayon support for parallel performance. It includes a build script to streamline ZK integration for WASM projects.
            </td>
        </tr>
        <tr>
            <td>
                <a href="https://github.com/zkmopro/mopro/tree/main/cli">
                    mopro-cli
                </a>
            </td>
            <td> WIP
            </td>
            <td>
                A command-line tool that makes it easy to scaffold ZK projects using selected proving systems and target platforms. Currently supports: Swift (Xcode), Kotlin (Android Studio), React Native, Flutter and Web.
            </td>
        </tr>
        <tr>
            <td>
                <a href="https://github.com/zkmopro/mopro/tree/main/circom-prover">
                    circom-prover
                </a>
            </td>
            <td>
                <a href="https://crates.io/crates/circom-prover">
                    <img src="https://img.shields.io/crates/v/circom-prover?label=circom-prover&style=flat-square">
                </a>
            </td>
            <td>
                A Rust-based Groth16 prover for Circom. It supports multiple witness generators (<a href="https://github.com/chancehudson/rust-witness">rust-witness</a>, <a href="https://github.com/zkmopro/witnesscalc_adapter">witnesscalc</a>, and <a href="https://github.com/iden3/circom-witnesscalc">circom-witnesscalc</a>) and provers (<a href="https://github.com/arkworks-rs/circom-compat">arkworks</a>, <a href="https://github.com/zkmopro/rust-rapidsnark">rapidsnark</a>). It is designed to work across devices including desktop, iOS, and Android.
            </td>
        </tr>
        <tr>
            <td>
                <a href="https://github.com/zkmopro/mopro/tree/main/test-e2e">
                    test-e2e
                </a>
            </td>
            <td>
            </td>
            <td>
                End-to-end test examples for verifying integrations between mopro-ffi and mopro-wasm.
            </td>
        </tr>
        <tr>
            <td>
                <a href="https://github.com/zkmopro/mopro/tree/main/docs">
                    docs
                </a>
            </td>
            <td>
            </td>
            <td>
                The source for <a href="https://zkmopro.org">zkmopro.org</a>, containing up-to-date documentation.
            </td>
        </tr>
    </tbody>
</table>

## 🎯 Mopro Kanban board

All tasks related to the Mopro implementation are public. You can track their progress, statuses, and additional details in the [Mopro Kanban](https://github.com/orgs/zkmopro/projects/1/views/3).

## 📱 Getting started

To get started with building a mobile app using Mopro, check out the [Getting Started](https://zkmopro.org/docs/getting-started/) guide and ensure you’ve installed all required [prerequisites](https://zkmopro.org/docs/prerequisites).

## 🛠 Install

Clone this repository:

```sh
git clone https://github.com/zkmopro/mopro.git
```

## 📜 Usage

### Code quality and formatting

Run [Rustfmt](https://github.com/rust-lang/rustfmt) to automatically format the code.

```sh
cargo fmt --all
```

Run [rust-clippy](https://github.com/rust-lang/rust-clippy) to catch common mistakes and improve your Rust code.

```sh
cargo clippy --all-targets --all-features
```

### `circom-prover`

To test all witness generators and proof generators:

```sh
cd circom-prover
cargo test --all-features
```

To run specific witness generator or proof generator

```sh
cd circom-prover
cargo test --features witnesscalc --features rapidsnark
```

> [!IMPORTANT]  
> To learn more about `circom-prover`, please visit [circom-prover](./circom-prover/README.md)

### `mopro-ffi`

-   To test for circom adapter
    ```sh
    cd mopro-ffi
    cargo test --features circom
    ```
-   To test for halo2 adapter

    ```sh
    cd mopro-ffi
    cargo test --features halo2
    ```

-   To test for noir adapter
    ```sh
    cd mopro-ffi
    cargo test --features noir --release
    ```

> [!IMPORTANT]  
> To learn more about `mopro-ffi`, please visit [mopro-ffi](./mopro-ffi/README.md)

### `mopro-wasm`

To test the wasm bindings with `wasm-pack test`

```sh
wasm-pack test --chrome --headless -- --all-features
```

> [!IMPORTANT]  
> To learn more about `mopro-wasm`, please visit [mopro-wasm](./mopro-wasm/README.md)

### `mopro-cli`

To install the CLI

```sh
cd cli
cargo install --path .
```

> [!IMPORTANT]  
> To learn more about `mopro-cli`, please visit [cli](./cli/README.md)

### `test-e2e`

#### iOS

-   Update bindings for iOS e2e app

    ```sh
    cargo run --bin ios
    ```

    or

    ```sh
    mopro build # with mopro CLI
    ```

    and choose `iOS`.

-   To test for iOS e2e app

    ```sh
    cd test-e2e
    open ios/mopro-test.xcodeproj
    ```

    Then choose an iOS simulator to run on.

    Or you can use xcodebuild and choose an iOS simulator to run the test.

    ```sh
    xcodebuild -project ./test-e2e/ios/mopro-test.xcodeproj \
        -scheme mopro-test \
        -destination 'platform=iOS Simulator,name=iPhone 15' \
        test CODE_SIGN_IDENTITY="" \
        CODE_SIGNING_REQUIRED=NO \
        -maximum-parallel-testing-workers 1
    ```

#### Android

-   Update bindings for Android e2e app

    ```sh
    cargo run --bin android
    ```

    or

    ```sh
    mopro build # with mopro CLI
    ```

    and choose `iOS`.

-   To test for Android e2e app

    ```sh
    cd test-e2e
    open android -a Android\ Studio
    ```

    Similar to the iOS app, choose an Android emulatro to run on.

    Or you can use

    ```sh
    cd test-e2e/android
    ./gradlew connectedAndroidTest
    ```

    to run on a connected emulator/device.

### docs

-   Install dependencies
    ```sh
    yarn
    ```
-   Build for the website
    ```sh
    yarn build
    ```
-   Start a server
    ```sh
    yarn start
    ```

## Performance

### Circom

Both native witness generation and proof generation are generally faster than `snarkjs` in the browser, with potential speed improvements of up to 20 times.
Check the details here: [performance](https://zkmopro.org/docs/performance).

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.

> [!IMPORTANT]
> We do not accept minor grammatical fixes (e.g., correcting typos, rewording sentences) unless they significantly improve clarity in technical documentation. These contributions, while appreciated, are not a priority for merging. If there is a grammatical error feel free to message the team.
