extern crate core;

use crate::halo2::FibonacciMoproCircuit;
use mopro_ffi::{app, Halo2ProveFn, Halo2VerifyFn, WtnsFn};

app!();

// Circom Sample

rust_witness::witness!(multiplier2);
rust_witness::witness!(multiplier2bls);
rust_witness::witness!(keccak256256test);
rust_witness::witness!(hashbenchbls);

// These circuits are specific to the app we're building here
// e.g. they're on in the mopro-ffi build, only in test-e2e
fn zkey_witness_map(name: &str) -> Result<WtnsFn, MoproError> {
    match name {
        "multiplier2_final.zkey" => Ok(multiplier2_witness),
        "keccak256_256_test_final.zkey" => Ok(keccak256256test_witness),
        "hashbench_bls_final.zkey" => Ok(hashbenchbls_witness),
        "multiplier2_bls_final.zkey" => Ok(multiplier2bls_witness),
        _ => Err(MoproError::CircomError("Unknown circuit name".to_string())),
    }
}

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
