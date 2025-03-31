# Mopro FFI

`mopro-ffi` is a tool designed to assist programmable cryptography application or rust application developers in efficiently creating bindings for client-side targets.

Key features include:

-   **Function Serialization and Export:** Enables serialization and export of functions within each proving system. To generate FFI bindings for different targets, inputs and outputs must conform to the specific types defined in [uniffi](https://mozilla.github.io/uniffi-rs/0.29/types/builtin_types.html).
    -   Supported proving systems: `circom`, `halo2`.
-   **Executable Binaries:** Provides pre-built binaries, allowing developers to generate bindings for various targets effortlessly.
    -   Supported targets: `swift`, `kotlin`.
-   **Customize Exported Functions:** Supports the ability to customize the exported functions. Users can define the functions with [procedural macros](https://mozilla.github.io/uniffi-rs/0.29/proc_macro/index.html).

## Usage

-   Please check the [Manual Setup for Android/iOS Bindings](https://zkmopro.org/docs/setup/rust-setup) for integrating `mopro-ffi` into your project.

## Usage for general Rust application

-   Integrate the `mopro-ffi` like the above tutorial.
-   Add `#[uniffi::export]` attribute to add the functions you want to export. Check out the built-in types in: [UniFFI: Built-in types](https://mozilla.github.io/uniffi-rs/0.29/types/builtin_types.html)

    -   E.g.
        export Rust function like
        ```rust
        #[uniffi::export]
        pub fn hello_world() -> String {
          "Hello World!".to_string()
        }
        ```

-   Run `cargo run --bin ios` or `cargo run --bin android` again.

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
-   Mopro Documentation: https://zkmopro.org

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
