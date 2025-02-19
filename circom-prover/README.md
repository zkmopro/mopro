# Circom Prover

Circom prover is a Rust library for generating and verifying proofs for [Circom](https://github.com/iden3/circom) circuits.
It is designed to be used in cross-platform applications, and is compatible with the [Mopro](https://github.com/zkmopro/mopro) library.

## Usage

Depends on the witness generation method, build the rust witness function first.
For example, if you use the [Rust Witness](https://github.com/chancehudson/rust-witness), please refer to the [Rust Witness](https://github.com/chancehudson/rust-witness) for more details.

### Proof Generation

```rust
use std::collections::HashMap;
rust_witness::witness!(multiplier2);
use circom_prover::{prover::ProofLib, witness::WitnessFn, CircomProver};

// Prepare inputs
let mut inputs = HashMap::new();
inputs.insert("a".to_string(), vec!["1".to_string()]);
inputs.insert("b".to_string(), vec!["2".to_string()]);

// Prepare zkey path
let zkey_path = "./test-vectors/multiplier2_final.zkey".to_string();

// Generate proof
let result = CircomProver::prove(
    ProofLib::Arkworks,
    WitnessFn::RustWitness(multiplier2_witness),
    inputs,
    zkey_path,
).unwrap();
```

### Proof Verification

```rust
// Verify proof
let valid = CircomProver::verify(
    ProofLib::Arkworks,
    result.proof,
    result.pub_inputs,
    zkey_path,
).unwrap();
```

### Proof Deserialization

```rust
use ark_bn254::Bn254;
use circom_prover::{
    prover::{
        serialization::{deserialize_inputs, deserialize_proof},
    },
};
let deserialized_proof = deserialize_proof::<Bn254>(result.proof);
let deserialized_pub_inputs = deserialize_inputs::<Bn254>(result.pub_inputs);
```

## Adapters

## Witness Generation

-   [x] [Rust Witness](https://github.com/chancehudson/rust-witness)
-   [ ] [Witnesscalc adapter](https://github.com/zkmopro/witnesscalc_adapter)
-   [ ] [circom witnesscalc](https://github.com/iden3/circom-witnesscalc)

## Proof Generation

-   [x] [Arkworks](https://github.com/arkworks-rs)
-   [ ] [Rust rapidsnark](https://github.com/zkmopro/rust-rapidsnark)

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
