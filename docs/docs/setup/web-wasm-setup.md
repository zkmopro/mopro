# Web(Wasm) setup

This tutorial will show you how to build static library for web browser.

Before proceeding, ensure that **Rust**, **Wasm-Pack** and **Chrome** are installed. Refer to the [Prerequisites](/docs/prerequisites).

## Support Halo2 Circuit Implementation

This section assumes the existence of a user-defined circuit implementation based on the [PSE Halo2](https://github.com/privacy-scaling-explorations/halo2). 

Note that there are multiple Halo2 implementations (e.g., Zcash, PSE, Axiom). Mopro primarily supports the [PSE Halo2](https://github.com/privacy-scaling-explorations/halo2), which is a Plonk backend and works well with [wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon). Refer to the [README](https://github.com/zkmopro/mopro/tree/main/mopro-wasm#introduction-to-wasm-compilation-with-halo2) of mopro-wasm for information about compatibility between Halo2 and Wasm.

## Update "mopro-wasm-lib"

Once followed ["3. Mopro build"](/docs/getting-started.md#3-build-bindings) for Web(Wasm) in ["Getting Started"](/docs/getting-started.md) page, there is "mopro-wasm-lib" would be generated when it selected with `web` template.

The "mopro-wasm-lib" is the place that compile wasm code eventually. So, it should be updated users Halo2 circuit instead of the example circuit implementations: fibonacci circuit with different backends: "plonk", "hyperplonk" and "gemini".

### 1. Modify 'Cargo.toml' in the "mopro-wasm-lib"

The user-defined circuit implmentation crate should be added as a dependency manually, as illustrated below:

```rust
[package]
name = "mopro-wasm-lib"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["rlib", "cdylib"]

[dependencies]
// mopro-wasm = { git = "https://github.com/zkmopro/mopro",features = [
//     "gemini",
//     "hyperplonk",
//     "plonk",
// ]}
my-halo2-circuit = { git = "http://github.com/users/my-halo2-circuit.git" }

[target.wasm32-unknown-unknown.dependencies]
console_error_panic_hook = "0.1.7"
getrandom = { version = "0.2.15", features = ["js"] }
serde-wasm-bindgen = "0.6.5"
wasm-bindgen = { version = "0.2.95", features = ["serde-serialize"] }
wasm-bindgen-console-logger = "0.1.1"
wasm-bindgen-futures = "0.4.47"
wasm-bindgen-rayon = { version = "1.2.2", features = ["no-bundler"] }
wasm-bindgen-test = "0.3.42"

```

The `mopro-wasm` crate no longer required for compiling users-defined circuit implementation: **"my-halo2-circuit"** in that case.

```shell
mopro-wasm-lib $ rustup run nightly-2024-07-18 wasm-pack build --target web --out-dir MoproWasmBindings
```


### 2. Create Wrapper Functions for Generate/Verify proof method

To compile Wasm code with the circuit, wrapper functions for generating and verifying proof methods in the user-defined circuit implementation must be created in `mopro-wasm-lib/src/lib.rs`, using the example structure provided below:

```rust
use my_halo2_circuit;

#[wasm_bindgen]
pub fn generate_proof(input: JsValue) -> Result<JsValue, JsValue> {
   // function implementations with `my-halo2-circuit`
   let proof = my_halo2_circuit::generate_proof(parsed_input);
   to_value(...)
}

#[wasm_bindgen]
pub fn verify_proof(proof: JsValue, public_inputs: JsValue) -> Result<JsValue, JsValue> {
   // function implementations with `my-halo2-circuit`
   let result = my_halo2_circuit::verify_proof(parsed_proof, parsed_public_input);
   to_value(...)
}
```

### 3. Build the Wasm Package

The following command builds the wasm package in "mopro-wasm-lib":

```shell
mopro-wasm-lib $ wasm-pack build --target web --out-name my-halo2-circuit
```

The generated wasm files will be located in the "pkg" folder. Refer to the [**wasm-pack**](https://rustwasm.github.io/wasm-pack/book/) documentation for more details.


### 4. **Integrate the Wasm Code**:

The wasm code can be imported and used in a web application, as illustrated below:

```javascript
const mopro_wasm = await import('./pkg/my-halo2-circuit.js');
await mopro_wasm.default();
await mopro_wasm.initThreadPool(navigator.hardwareConcurrency);

async function generateProof (input) {
      const proof = await mopro_wasm.generate_proof(input);
      console.log(proof);
}
```

Initializing with `initThreadPool` is necessary to enable multi-threading n WebAssembly within the browser.
