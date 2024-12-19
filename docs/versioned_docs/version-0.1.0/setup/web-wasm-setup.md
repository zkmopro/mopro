# Web(Wasm) setup

This tutorial will show you how to build static library with Halo2 adapter for web browser.

Before proceeding, ensure that **Rust**, **Wasm-Pack** and **Chrome** are installed. Refer to the [Prerequisites](/docs/prerequisites).

### Proving from Web Browser

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
