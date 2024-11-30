# mopro-wasm

The mopro-wasm module enables the compilation of WASM code using `wasm-pack`. This module supports multithreading in WASM through the use of `wasm-bindgen-rayon`.

## Development


### Prerequisites

1. Install Rust and wasm-pack:

    Ensure you have Rust installed, and install wasm-pack for building and testing WASM modules:

    ```bash
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    ```

2. Install chrome and chromedriver:
    
    Both chrome and chromedriver must be installed, and their versions must match.

    Please refer to [Get started with ChromeDriver](https://developer.chrome.com/docs/chromedriver/get-started)

### Building

To compile the WASM files, use the following commands based on the desired feature:

Compile plonk Circuit to WASM:
```bash
wasm-pack build --target web -- --features plonk
```
Compile hyperplonk Circuit to WASM:
```bash
wasm-pack build --target web -- --features hyperplonk
```

Compile gemini Circuit to WASM:
```bash
wasm-pack build --target web -- --features gemini
```