<p align="center">
<img src="img/logo_title.svg">
</p>

<p align="center">
    <a href="https://github.com/zkmopro" target="_blank">
        <img src="https://img.shields.io/badge/project-Mopro-blue.svg?style=flat-square">
    </a>
    <a href="/LICENSE-APACHE">
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
                <a href="https://github.com/zkmopro/mopro/tree/main/cli">
                    mopro-cli
                </a>
            </td>
            <td> 
                <a href="https://crates.io/crates/mopro-cli">
                    <img src="https://img.shields.io/crates/v/mopro-cli?label=mopro-cli&style=flat-square">
                </a>
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
                <a href="https://github.com/zkmopro/mopro/tree/main/tests">
                    tests
                </a>
            </td>
            <td>
            </td>
            <td>
                End-to-end test examples for verifying integrations for mopro-ffi.
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

To run with specific witness generator and/or proof generator

```sh
cd circom-prover
cargo test --features witnesscalc --features rapidsnark
```

> [!IMPORTANT]  
> To learn more about `circom-prover`, please visit [circom-prover](./circom-prover/README.md)

### `mopro-ffi`

To test the wasm bindings with `wasm-pack test`

```sh
wasm-pack test --chrome --headless -- --all-features
```

> [!IMPORTANT]  
> To learn more about `mopro-ffi`, please visit [mopro-ffi](./mopro-ffi/README.md)

### `mopro-cli`

To install the CLI

```sh
cd cli
cargo install --path .
```

> [!IMPORTANT]  
> To learn more about `mopro-cli`, please visit [cli](./cli/README.md)

### `tests`

#### iOS

-   Update bindings for iOS

    ```sh
    cargo run --bin ios
    ```

    or

    ```sh
    mopro build # with mopro CLI
    ```

    and choose `iOS`.

#### Android

-   Update bindings for Android

    ```sh
    cargo run --bin android
    ```

    or

    ```sh
    mopro build # with mopro CLI
    ```

    and choose `Android`.

#### Flutter

-   Update bindings for Flutter

    ```sh
    cargo run --bin flutter --no-default-features --features flutter
    ```

    or

    ```sh
    mopro build # with mopro CLI
    ```

    and choose `flutter`.

#### React Native

-   Update bindings for React Native

    ```sh
    cargo run --bin react_native
    ```

    or

    ```sh
    mopro build # with mopro CLI
    ```

    and choose `react-native`.

#### Web

-   Update bindings for wasm

    ```sh
    cargo run --bin web --no-default-features --features wasm
    ```

    or

    ```sh
    mopro build # with mopro CLI
    ```

    and choose `web`.

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

Both native circom witness generation and proof generation are generally faster than `snarkjs` in the browser, with potential speed improvements of up to 20 times.
Check the details for circom, halo2, and noir provers here: [performance](https://zkmopro.org/docs/performance).

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.

> [!IMPORTANT]
> We do not accept minor grammatical fixes (e.g., correcting typos, rewording sentences) unless they significantly improve clarity in technical documentation. These contributions, while appreciated, are not a priority for merging. If there is a grammatical error feel free to message the team.
