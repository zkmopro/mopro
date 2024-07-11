extern crate core;

use once_cell::sync::Lazy;

use mopro_ffi::app;

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
    multiplier2bls_witness,
    "keccak256_256_test_final.zkey",
    keccak256256test_witness,
);

// Halo2 Sample
mod halo2;
use crate::halo2::FibonacciMoproCircuit;
use mopro_ffi::{Halo2ProveFn, Halo2VerifyFn};

mopro_ffi::set_halo2_proving_circuits!("fibonacci_pk", FibonacciMoproCircuit::prove);

mopro_ffi::set_halo2_verifying_circuits!("fibonacci_vk", FibonacciMoproCircuit::verify);
