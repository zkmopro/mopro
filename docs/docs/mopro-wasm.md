# mopro-wasm

The mopro-wasm module enables the compilation of WASM code using `wasm-pack` for supported proving systems.

**Note:** Currently, only the WASM module for Halo2 can be generated in mopro-wasm. Support for additional proving systems may be added in the future.

This module supports multithreading in WASM through the use of `wasm-bindgen-rayon`.

## Development

### Prerequisites

1. Install Rust and wasm-pack:

    Ensure you have Rust installed, and install wasm-pack for building and testing WASM modules:

    ```bash
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    ```

2. Install chrome and chromedriver(for testing):
    
    Both chrome and chromedriver must be installed, and their versions must match.

    Please refer to [Get started with ChromeDriver](https://developer.chrome.com/docs/chromedriver/get-started)

### Building

To compile the WASM module for all supported backends-"plonk," "hyperplonk," and "gemini", run the following command:

```bash
wasm-pack build --target web -- --all-features
```

This command outputs files into the **pkg** directory, including the generated WASM file, JavaScript bindings, and metadata required for integration with web applications.

### Testing

Run the Fibonacci circuit tests for all the backends in the browser in headless mode.

```bash
wasm-pack test --chrome --headless -- --all-features
```