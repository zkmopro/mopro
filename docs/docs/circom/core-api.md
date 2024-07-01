# Core API

## `CircomState`

This object stores the states required to generate a ZK proof.

Example of usage:

```rust
// Instantiate CircomState
let mut circom_state = CircomState::new();

// Initialize with keys
let wasm_path = "./multiplier2.wasm";
let zkey_path = "./multiplier2_final.zkey";
let setup_res = circom_state.initialize(zkey_path, wasm_path);

// Prepare inputs
let mut inputs = HashMap::new();
let a = 3;
let b = 5;
inputs.insert("a".to_string(), vec![BigInt::from(a)]);
inputs.insert("b".to_string(), vec![BigInt::from(b)]);

// Proof generation
let generate_proof_res = circom_state.generate_proof(inputs);

// Proof verification
let (serialized_proof, serialized_inputs) = generate_proof_res.unwrap();
let verify_res = circom_state.verify_proof(serialized_proof, serialized_inputs);
```

### `new`

Creates and returns a new instance of the struct.

```rust
pub fn new() -> Self
```

### `initialize`

Initializes the instance with the given `zkey_path` and `wasm_path`.

```rust
pub fn initialize(
    &mut self,
    zkey_path: &str,
    wasm_path: &str
) -> Result<(), MoproError>
```

### `generate_proof`

Generates a proof based on the provided circuit inputs.

```rust
pub fn generate_proof(
    &mut self,
    inputs: CircuitInputs,
) -> Result<(SerializableProof, SerializableInputs), MoproError>
```

### `verify_proof`

Verifies the provided proof against the given inputs.

```rust
pub fn verify_proof(
    &self,
    serialized_proof: SerializableProof,
    serialized_inputs: SerializableInputs,
) -> Result<bool, MoproError>
```

## `generate_proof_static`

Generates a proof based on the provided circuit inputs.<br/>
:::warning
**Note: The function is different from [`generate_proof`](#generate_proof).** <br/>
In this function, the zkey and wasm are precompiled during `cargo build`. <br/>
You can specify the [mopro-config.toml](configuration) to build the default circuits.
:::

```rust
pub fn generate_proof_static(
    inputs: CircuitInputs,
) -> Result<(SerializableProof, SerializableInputs), MoproError>
```

## `verify_proof_static`

Verifies the provided proof against the given inputs.<br/>
:::warning
**Note: The function is different from [`verify_proof`](#verify_proof).** <br/>
In this function, the zkey and wasm are precompiled during `cargo build`. <br/>
You can specify the [mopro-config.toml](configuration) to build the default circuits.
:::

```rust
pub fn verify_proof_static(
    serialized_proof: SerializableProof,
    serialized_inputs: SerializableInputs,
) -> Result<bool, MoproError>
```
