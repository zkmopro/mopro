# Web(Wasm) setup

This tutorial will show you how to build static library for web browser.

Before proceeding, ensure that **Rust**, **Wasm-Pack** and **Chrome** are installed. Refer to the [Prerequisites](/docs/prerequisites).

## Support Halo2 Circuit Implementation

This section assumes the existence of a user-defined circuit implementation based on the [PSE Halo2](https://github.com/privacy-scaling-explorations/halo2).

Note that there are multiple Halo2 implementations (e.g., Zcash, PSE, Axiom). Mopro primarily supports the [PSE Halo2](https://github.com/privacy-scaling-explorations/halo2), which is a Plonk backend and works well with [wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon). Refer to the [README](https://github.com/zkmopro/mopro/tree/main/mopro-ffi#introduction-to-wasm-compilation-with-halo2) of mopro-ffi for information about compatibility between Halo2 and Wasm.

## Update `MoproWasmBindings`

Once followed [`mopro init`](/docs/getting-started#2-initialize-adapters) in ["Getting Started"](/docs/getting-started.md) page, there is a Rust project that lets you customize your functions.

### 1. Modify 'Cargo.toml' in the root

The user-defined circuit implementation crate should be added as a dependency manually, as illustrated below:

```toml
[dependencies]
mopro-ffi = { ... }
# ...
# HALO2_DEPENDENCIES
my-halo2-circuit = { git = "http://github.com/users/my-halo2-circuit.git" }
```

### 2. Create Wrapper Functions for Generate/Verify proof method

To compile Wasm code with the circuit, wrapper functions for generating and verifying proof methods in the user-defined circuit implementation must be created in `src/lib.rs`, using the example structure provided below:

```rust
set_halo2_circuits! {
    ("my_halo2_circuit_pk.bin", my_halo2_circuit::prove, "my_halo2_circuit_vk.bin", my_halo2_circuit::verify),
}
```

### 3. Build again for web

(Optional) To ensure a clean build, remove the existing `MoproWasmBindings` directory in the `mopro-example-app`.
Then, execute the `mopro build` command again, selecting the "web" platform in `mopro-example-app`:

```shell
mopro-example-app $ rm -rf MoproWasmBindings
mopro-example-app $ mopro build
```

### 4. **Integrate the Wasm Code**:

The wasm code can be imported and used in a web application, as illustrated below:

```javascript
const mopro_wasm = await import("../MoproWasmBindings/mopro_wasm_lib.js");
await mopro_wasm.default();
await mopro_wasm.initThreadPool(navigator.hardwareConcurrency);

async function fetchBinaryFile(url) {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`Failed to load ${url}`);
    return new Uint8Array(await response.arrayBuffer());
}

async function generateProof(input) {
    const name = "my_halo2_circuit_pk";
    const SRS_KEY = await fetchBinaryFile("./assets/plonk_fibonacci_srs.bin");
    const PROVING_KEY = await fetchBinaryFile(
        "./assets/plonk_fibonacci_pk.bin"
    );
    const proof = await mopro_wasm.generateHalo2Proof(
        name,
        srs_key,
        proving_key,
        input
    );
    console.log(proof);
}
```

Initializing with `initThreadPool` is necessary to enable multi-threading n WebAssembly within the browser.

Note that the web template generated using the `mopro create` command is currently built only for example circuit implementations: `plonk-fibonacci`, `hyperplonk-fibonacci` and `gemini-fibonacci`. User should modify `test_mopros.js` and `index.html` manually if want to use the web template with users' circuit implementation, such as `my-halo2-circuit` in this tutorial.
