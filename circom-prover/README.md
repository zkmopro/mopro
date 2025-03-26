# Circom Prover

Circom prover is a Rust library for generating and verifying proofs for [Circom](https://github.com/iden3/circom) circuits.
It is designed to be used in cross-platform applications, and is compatible with the [Mopro](https://github.com/zkmopro/mopro) library.

## Usage

Install `circom-prover`

```sh
cargo add circom-prover
```

Depends on the witness generation method, build the rust witness function first.
Here is how to use the [rust-witness](https://github.com/chancehudson/rust-witness) generator.

Include the crate in your Cargo.toml:

```toml
[dependencies]
rust-witness = "0.1"
num-bigint = "0.4"

[build-dependencies]
rust-witness = "0.1"
```

In build.rs, add the following code to compile the witness generator wasm sources (<circuit name>.wasm) into a native library and link to it:

```rs
rust_witness::transpile::transpile_wasm("../path to directory containing your wasm sources");
// e.g. rust_witness::transpile::transpile_wasm("./test-vectors".to_string());
// The directory should contain the following files:
// - <circuit name>.wasm
```

### Proof Generation

```rust
use std::collections::HashMap;
rust_witness::witness!(multiplier2);
use circom_prover::{prover::ProofLib, witness::WitnessFn, CircomProver};

// Prepare inputs
let inputs = HashMap::from([
    ("a".to_string(), vec!["1".to_string()]),
    ("b".to_string(), vec!["2".to_string()]),
]);
let input_str = serde_json::to_string(&inputs).unwrap();

// Prepare zkey path
let zkey_path = "./test-vectors/multiplier2_final.zkey".to_string();

// Generate proof
let result = CircomProver::prove(
    ProofLib::Arkworks,
    WitnessFn::RustWitness(multiplier2_witness),
    input_str,
    zkey_path.clone(),
).unwrap();
```

### Proof Verification

```rust
// Verify proof
let valid = CircomProver::verify(
    ProofLib::Arkworks,
    result,
    zkey_path,
).unwrap();
```

## Advanced usage

`circom-prover` also supports [`witnesscalc`](https://github.com/0xPolygonID/witnesscalc) and [`rapidsnark`](https://github.com/iden3/rapidsnark) for advanced users. These tools offer better performance but may not be entirely stable.

Below is a tutorial on how to enable `witnesscalc` and `rapidsnark`.

### Set feature flags

```toml
[dependencies]
circom-prover = {
    version = "0.1",
    default-features = false, 
    features = ["witnesscalc", "rapidsnark"]
}
witnesscalc-adapter = "0.1"
anyhow = "1.0"

[build-dependencies]
witnesscalc-adapter = "0.1"
circom-prover = {
    version = "0.1",
    default-features = false, 
    features = ["rapidsnark"]
}
```

### `witnesscalc`

In build.rs, add the following code to compile the witness generator wasm sources (<circuit name>.wasm) into a native library and link to it:

```rs
witnesscalc_adapter::build_and_link("../path to directory containing your C++ sources");
// e.g. witnesscalc_adapter::build_and_link("../testdata");
// The directory should contain the following files:
// - <circuit name>.cpp
// - <circuit name>.dat
```

### Proof Generation

```rust
witnesscalc_adapter::witness!(multiplier2);
use circom_prover::{prover::ProofLib, witness::WitnessFn, CircomProver};
use std::collections::HashMap;
use anyhow::Result;

// Prepare inputs
let inputs = HashMap::from([
    ("a".to_string(), vec!["1".to_string()]),
    ("b".to_string(), vec!["2".to_string()]),
]);
let input_str = serde_json::to_string(&inputs).unwrap();

// Prepare zkey path
let zkey_path = "./test-vectors/multiplier2_final.zkey".to_string();

// Generate proof
let result = CircomProver::prove(
    ProofLib::Rapidsnark,
    WitnessFn::WitnessCalc(multiplier2_witness),
    input_str,
    zkey_path.clone(),
)
.unwrap();
```

### Proof Verification

```rust
// Verify proof
let valid = CircomProver::verify(
    ProofLib::Rapidsnark,
    result,
    zkey_path,
).unwrap();
```
## Adapters

## Witness Generation

-   [x] [Rust Witness](https://github.com/chancehudson/rust-witness)
-   [x] [Witnesscalc adapter](https://github.com/zkmopro/witnesscalc_adapter)
-   [ ] [circom witnesscalc](https://github.com/iden3/circom-witnesscalc)

## Proof Generation

-   [x] [Arkworks](https://github.com/arkworks-rs)
-   [x] [Rust rapidsnark](https://github.com/zkmopro/rust-rapidsnark)

## Performance

It speeds up circom proof by ~100x comparing to [arkworks-rs/circom-compat](https://github.com/arkworks-rs/circom-compat) in keccak256 circuits.
We will provide more benchmarks with different adapters in the future.
And you can also check the [Mopro documentation](https://zkmopro.org/docs/performance) for more benchmarks.

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
-   Mopro Documentation: https://zkmopro.org

## Acknowledgements

This work is sponsored by [PSE](https://pse.dev/).
