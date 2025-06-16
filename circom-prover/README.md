# Circom Prover

Circom prover is a Rust library for generating and verifying proofs for [Circom](https://github.com/iden3/circom) circuits.
It is designed to be used in cross-platform applications, and is compatible with the [Mopro](https://github.com/zkmopro/mopro) library.

It supports multiple adapters, allowing you to choose the one that best optimizes performance.

## Features

-   ✅ **Cross-platform proving** — Works seamlessly on Linux, macOS, iOS, and Android.
-   🔄 **Flexible architecture** — Easily switch between different witness generators and proof systems.
-   ⚡ **High performance** — The fastest known Rust-based Circom prover (see [performance](#performance)).
-   🔐 **Curve support** — Supports both `BN254` and `BLS12-381` elliptic curves.

## Adapters

## Witness Generation

-   ✅ [Rust Witness](https://github.com/chancehudson/rust-witness)
-   ✅ [Witnesscalc adapter](https://github.com/zkmopro/witnesscalc_adapter)
-   ✅ [circom witnesscalc](https://github.com/iden3/circom-witnesscalc)

## Proof Generation

-   ✅ [Arkworks](https://github.com/arkworks-rs)
-   ✅ [Rust rapidsnark](https://github.com/zkmopro/rust-rapidsnark)

## Performance

It speeds up circom proof by ~100x comparing to [arkworks-rs/circom-compat](https://github.com/arkworks-rs/circom-compat) in keccak256 circuits.
We will provide more benchmarks with different adapters in the future.
And you can also check the [Mopro documentation](https://zkmopro.org/docs/performance) for more benchmarks.
We’ve also published a blog post that outlines the pros and cons of each adapter: [Comparison of Circom Provers](https://zkmopro.org/blog/circom-comparison).

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

[build-dependencies]
rust-witness = "0.1"
```

In build.rs, add the following code to compile the witness generator wasm sources (<circuit name>.wasm) into a native library and link to it:

```rust
rust_witness::transpile::transpile_wasm("../path to directory containing your wasm sources");
// e.g. rust_witness::transpile::transpile_wasm("./test-vectors".to_string());
// The directory should contain the following files:
// - <circuit name>.wasm
```

### Proof Generation

```rust
use std::collections::HashMap;
use circom_prover::{prover::ProofLib, witness::WitnessFn, CircomProver};

// Prepare witness generator
rust_witness::witness!(multiplier2);

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

`circom-prover` also supports [`witnesscalc`](https://github.com/0xPolygonID/witnesscalc), [`circom-witnesscalc`](https://github.com/iden3/circom-witnesscalc) and [`rapidsnark`](https://github.com/iden3/rapidsnark) for advanced users. These tools offer better performance but may not be entirely stable.

Below is a tutorial on how to enable `witnesscalc`, `circom-witnesscalc` and `rapidsnark`.

### `witnesscalc`

Setup features flags

```toml
[dependencies]
circom-prover = { version = "0.1", default-features = false, features = ["witnesscalc"] }
witnesscalc-adapter = "0.1"

[build-dependencies]
witnesscalc-adapter = "0.1"
```

Install dependencies:

-   Linux:

```sh
sudo apt install build-essential cmake m4 nasm libstdc++6
```

-   MacOS

```sh
brew install nasm
```

Rust toolchain: `cargo 1.81.0 (2dbb1af80 2024-08-20)`

In build.rs, add the following code to compile the witness generator cpp and dat sources (<circuit name>.cpp, <circuit name>.dat) into a native library and link to it:

```rust
witnesscalc_adapter::build_and_link("../path to directory containing your C++ sources");
// e.g. witnesscalc_adapter::build_and_link("../testdata");
// The directory should contain the following files:
// - <circuit name>.cpp
// - <circuit name>.dat
```

Usage

```rust
let result = CircomProver::prove(
    ProofLib::Arkworks,
    WitnessFn::WitnessCalc(multiplier2_witness), // the witnesscalc calculator
    input_str,
    zkey_path.clone(),
).unwrap();
```

### `circom-witnesscalc`

Prepare the graph.

Clone the repo: https://github.com/iden3/circom-witnesscalc, go to the directory `circom-witnesscalc`, and run:

```sh
cargo run --package circom-witnesscalc --bin build-circuit <path_to_circuit.circom> <path_to_circuit_graph.bin> [-l <path_to_circom_libs/>]* [-i <inputs_file.json>]
```

Setup features flags

```toml
[dependencies]
circom-prover = { version = "0.1", default-features = false, features = ["circom-witnesscalc"] }
```

Install dependencies:

-   Linux:

```sh
sudo apt install protobuf-compiler
```

Usage

```rust
// import graph path
const GRAPH_PATH: &str = "./test-vectors/circom/multiplier2.bin";
circom_prover::graph!(multiplier2, &GRAPH_PATH);

let result = CircomProver::prove(
    ProofLib::Arkworks,
    WitnessFn::CircomWitnessCalc(multiplier2_witness), // the circom-witnesscalc calculator
    input_str,
    zkey_path.clone(),
).unwrap();
```

### `rapidsnark`

```toml
[dependencies]
circom-prover = { version = "0.1", default-features = false, features = ["rapidsnark"] }

[build-dependencies]
circom-prover = { version = "0.1", default-features = false, features = ["rapidsnark"] }
```

```rust
use circom_prover::{prover::ProofLib, witness::WitnessFn, CircomProver};
use std::collections::HashMap;
use anyhow::Result;

// Prepare witness generator
witnesscalc_adapter::witness!(multiplier2);

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
    ProofLib::Rapidsnark, // The rapidsnark prover
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
    ProofLib::Rapidsnark, // the rapidsnark verifier
    result,
    zkey_path,
).unwrap();
```

### `BLS12-381` Curve

To use the `BLS12-381` curve, simply generate a `.wasm` with [`circom`](https://docs.circom.io/getting-started/installation/) and generate a `.zkey` file with [`snarkjs`](https://github.com/iden3/snarkjs) using the bls12-381 curve option. Then, provide this .zkey and your input to the prover to produce a BLS12-381 proof. Usage is identical to [proof generation](#proof-generation-1), with only the witness function and zkey path differing.

> [!NOTE]  
> This feature is currently only supported when using the default configuration, which includes `rust-witness` and `arkworks`.

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
-   Mopro Documentation: https://zkmopro.org

## Acknowledgements

This work is sponsored by [PSE](https://pse.dev/).
