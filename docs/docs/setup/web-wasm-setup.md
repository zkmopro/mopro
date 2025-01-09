# Web(Wasm) setup

This tutorial will show you how to build static library with Halo2 adapter for web browser.

Before proceeding, ensure that **Rust**, **Wasm-Pack** and **Chrome** are installed. Refer to the [Prerequisites](/docs/prerequisites).

## Proving from Web Browser

This web test page requires three Wasm circuits: `halo2-plonk-fibonacci`, `halo2-hyperplonk-fibonacci`, and `halo2-gemini-fibonacci`.  

For more details on building and integrating these circuits, refer to [`mopro-wasm/README.md`](https://github.com/zkmopro/mopro/blob/main/mopro-wasm/README.md).

1. **Generate the Wasm package**:  
   Use `wasm-pack` to build the `mopro` Wasm package with all features enabled. Run the following command from the `mopro-wasm` directory:

   ```bash
   mopro-wasm $ wasm-pack build --target web --out-dir ../test-e2e/MoproWasmBindings -- --all-features
   ```

2. Run the Test Server:
    Navigate to the 'test-e2e/web' directory, install dependencies, and start the server with following commands:

    ```bash
    test-e2e/web $ yarn && yarn start
    ```

3. **Verify the Results**:  
   Open a web browser and visit the test page at the default url: `http://localhost:3000`.
   
   Check the results displayed in the browser console or user interface.


## Integrating a New Circuit Implementation

This section assumes the existence of a user-defined circuit implementation based on the [PSE Halo2](https://github.com/privacy-scaling-explorations/halo2). 

Note that there are multiple Halo2 implementations (e.g., Zcash, PSE, Axiom). Mopro primarily supports the [PSE Halo2](https://github.com/privacy-scaling-explorations/halo2), which is a Plonk backend and works well with [wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon). Refer to the [README](https://github.com/zkmopro/mopro/tree/main/mopro-wasm#introduction-to-wasm-compilation-with-halo2) of mopro-wasm for information about compatibility between Halo2 and Wasm.

Follow these steps to update and build the user-defined circuit implementation for Wasm:

1. **Update `Cargo.toml` in 'mopro-wasm'**

    The customized circuit crate should be added as a dependency manually, as illustrated below:

      ```toml
      [dependencies]
      ...
      gemini-fibonacci = { package = "gemini-fibonacci", git = "https://github.com/sifnoc/plonkish-fibonacci-sample.git", optional = true }
      my-halo2-circuit = { ... }
      ```

2. **Create Wrapper Functions for Generate/Verify proof method**:

   To compile Wasm code with the circuit, wrapper functions for generating and verifying proof methods in the user-defined circuit implementation must be created in `mopro-wasm/src/lib.rs`, using the example structure provided below:

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

3. **Build the Wasm Package**

    The following command builds the wasm package in "mopro-wasm":

   ```shell
   mopro-wasm $ wasm-pack build --target web --out-name my-halo2-circuit
   ```

   The generated wasm files will be located in the "pkg" folder. Refer to the [**wasm-pack**](https://rustwasm.github.io/wasm-pack/book/) documentation for more details.


4. **Integrate the Wasm Code**:
    
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