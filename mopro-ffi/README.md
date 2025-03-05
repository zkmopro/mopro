# Mopro FFI

`mopro-ffi` is a tool designed to assist programmable cryptography application or rust application developers in efficiently creating bindings for client-side targets.

Key features include:

-   **Function Serialization and Export:** Enables serialization and export of functions within each proving system. To generate FFI bindings for different targets, inputs and outputs must conform to the specific types defined in [uniffi](https://mozilla.github.io/uniffi-rs/latest/udl/builtin_types.html).
    -   Supported proving systems: `circom`, `halo2`.
-   **Executable Binaries:** Provides pre-built binaries, allowing developers to generate bindings for various targets effortlessly.
    -   Supported targets: `swift`, `kotlin`.
-   **Customize Exported Functions:** Supports the ability to customize the exported functions. Users can define the functions in the `src/mopro.udl` file.

## Usage

-   Please check the [Manual Setup for Android/iOS Bindings](https://zkmopro.org/docs/setup/rust-setup) for integrating `mopro-ffi` into your project.

## Usage for general Rust application

-   Integrate the `mopro-ffi` like the above tutorial.
-   Update the `src/mopro.udl` file to add the functions you want to export. Check out how to define the functions in the UDL file: [UniFFI: The UDL file](https://mozilla.github.io/uniffi-rs/0.28/udl_file_spec.html)

    -   E.g.
        export Rust function like
        ```rust
        pub fn hello_world() -> String {
          "Hello World!".to_string()
        }
        ```
        and define the function in the UDL file like:
        ```udl
        namespace mopro {
          // ...
          string hello_world();
        }
        ```

-   Run `cargo run --bin ios` or `cargo run --bin android` again.

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
-   Mopro Documentation: https://zkmopro.org

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
