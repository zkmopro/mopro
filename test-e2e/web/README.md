# Simple Web App with Wasm

## Preparations

Follow these steps only if there are changes in the `mopro-wasm` package. Otherwise, these steps are not required.

### Compile Halo2 Circuits to Wasm

This web test page requires three Wasm circuits: `halo2-plonk-fibonacci`, `halo2-hyperplonk-fibonacci`, and `halo2-gemini-fibonacci`.  
Please refer to [`mopro-wasm/README.md`](../../mopro-wasm/README.md) for more details.

Use `wasm-pack` to generate halo2 wasm packages for this testing web app with the following commands:

```bash
mopro-wasm $ wasm-pack build --target web --out-dir ../test-e2e/web/halo2-plonk-fibonacci -- --features plonk
mopro-wasm $ wasm-pack build --target web --out-dir ../test-e2e/web/halo2-hyperplonk-fibonacci -- --features hyperplonk
mopro-wasm $ wasm-pack build --target web --out-dir ../test-e2e/web/halo2-gemini-fibonacci -- --features gemini
```

### Copy Parameters

Each generated halo2 wasm package requires a `parameters` folder, which is located in `test-vectors/halo2`.
Copy the `parameters` folder into each corresponding `halo2-*-fibonacci` directory as shown below:

```text
test-e2e/web/halo2-plonk-fibonacci/parameters
test-e2e/web/halo2-hyperplonk-fibonacci/parameters
test-e2e/web/halo2-gemini-fibonacci/parameters
```

## Test

1. Start a simple server by running the following command:

    ```bash
    yarn start
    ```

2. Test the application either manually or by using the test script:

   **Manually**:
   - Open your browser and navigate to: [http://localhost:3000](http://localhost:3000).

   **Using the test script**:
   - Run the following command:
     ```bash
     yarn test
     ```