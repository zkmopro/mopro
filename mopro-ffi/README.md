# mopro-ffi

mopro-ffi is a tool designed to assist programmable cryptography application developers in efficiently creating bindings for client-side targets.

Key features include:

-   **Function Serialization and Export:** Enables serialization and export of functions within each proving system. To generate FFI bindings for different targets, inputs and outputs must conform to the specific types defined in [uniffi-rs](https://mozilla.github.io/uniffi-rs/latest/udl/builtin_types.html).
    -   Supported proving systems: `circom`, `halo2`, [`Ashlang`](https://github.com/chancehudson/ashlang).
-   **Executable Binaries:** Provides pre-built binaries, allowing developers to generate bindings for various targets effortlessly.
    -   Supported targets: `swift`, `kotlin`.

## Getting started

-   Make sure you've installed the [prerequisites](https://zkmopro.org/docs/prerequisites).
-   Getting started with this [tutorial](https://zkmopro.org/docs/setup/rust-setup).

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
