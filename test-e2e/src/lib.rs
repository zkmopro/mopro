extern crate core;

use once_cell::sync::Lazy;

use mopro_ffi::app;
use mopro_ffi::{Halo2ProveFn, Halo2VerifyFn};

use crate::halo2::FibonacciMoproCircuit;

app!();

// Circom Sample

rust_witness::witness!(multiplier2);
rust_witness::witness!(multiplier2bls);
rust_witness::witness!(keccak256256test);
rust_witness::witness!(hashbenchbls);

// If you are not using circom, you need to have an empty set_circom_circuits! macro
mopro_ffi::set_circom_circuits!(
    "multiplier2_final.zkey",
    multiplier2_witness,
    "multiplier2_bls_final.zkey",
    multiplier2bls_witness
);

// Halo2 Sample
mod halo2;

// These circuits are specific to the app we're building here
// e.g. they're on in the mopro-ffi build, only in test-e2e
fn key_halo2_circuit_map(name: &str) -> Result<(Halo2ProveFn, Halo2VerifyFn), MoproError> {
    match name {
        "fibonacci_pk" | "fibonacci_vk" => {
            Ok((FibonacciMoproCircuit::prove, FibonacciMoproCircuit::verify))
        }
        _ => Err(MoproError::Halo2Error("Unknown circuit name".to_string())),
    }
}
