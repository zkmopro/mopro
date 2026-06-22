mod adapter;

pub use adapter::{CircomOutput, CircomProof, CircomProverAdapter, ProofLib, G1, G2};

// WitnessFn is a function pointer type — no FFI derives needed, re-export as-is.
pub use circom_prover::witness::WitnessFn;

// uniffi::Record / Enum derives require UniFfiTag, which setup_scaffolding! generates.
#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
