# MoPro CLI

**What is MoPro?**

MoPro stands for **Mobile Prover** — a framework designed to simplify the development of client-side zero-knowledge (ZK) proof systems on mobile platforms.

👉 Visit [zkmopro.org](https://zkmopro.org) to learn more about using MoPro and MoPro CLI.

**Mopro CLI** is a developer-friendly command-line tool that simplifies building native mobile apps with `mopro-ffi`. It streamlines the integration process and offers powerful customization options.

## Key Features

-   **Modular:** Choose one or more adapters from `mopro-ffi`—currently supporting Circom, Halo2, and Noir. You can also integrate your own Rust crate. See [Custom Adapters](https://zkmopro.org/docs/adapters/overview#custom-adapters) for details.
-   **Versatile:** Generate templates for various platforms including _Swift (Xcode)_, _Kotlin (Android Studio)_, _React Native_, _Flutter_, and _Web_.
-   **Automated:** Skip the tedious setup—`mopro` CLI automates [UniFFI](https://github.com/mozilla/uniffi-rs) bindings and configures Xcode and Android Studio for you.

## Usage

### Installation

```sh
cargo install mopro-cli
```

-   Install the latest change on GitHub

```sh
git clone https://github.com/zkmopro/mopro
cd mopro/cli
cargo install --path .
```

### Help

```sh
mopro --help
```

or

```sh
mopro init --help
```

to see instructions for each command.

### Initialization

```sh
mopro init
```

### Build bindings

```sh
mopro build
```

or 

```sh
mopro build --auto-update
```

### Create templates

```sh
mopro create
```

### Update bindings

```sh
mopro update
```

or

```sh
mopro update [--src PATH] [--dest PATH] [--no-prompt]
```

By default `mopro update` looks for bindings and mobile projects in the current
directory. Use `--src` to point to a bindings directory and `--dest` to target a
specific mobile project located elsewhere. Frequently used destinations can be
stored in `Config.toml` under an `update` section:

```toml
[update]
ios_dest = "../MyiOSApp"
android_dest = "../MyAndroidApp"
```

### Create bindings without Rust project

```sh
mopro bindgen
```

You can customize the bindings generation:

-   Choose a witness generator adapter (default `rust-witness`):

    ```sh
    mopro bindgen --adapter witnesscalc
    ```

-   Specify the output directory for generated bindings:

    ```sh
    mopro bindgen --output-dir ./output
    ```

## Development

After cloning the repository, you can install the CLI locally with your changes by running:

```sh
git clone https://github.com/zkmopro/mopro
cd mopro/cli
cargo install --path .
```

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
-   Mopro Documentation: https://zkmopro.org
-   Mopro Github: https://github.com/zkmopro/mopro

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
