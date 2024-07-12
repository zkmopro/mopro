extern crate core;

// First, configure the Mopro FFI library
mopro_ffi::app!();

// Circom Example of setting up 4 circuits
rust_witness::witness!(multiplier2);
rust_witness::witness!(multiplier2bls);
rust_witness::witness!(keccak256256test);
rust_witness::witness!(hashbenchbls);

mopro_ffi::set_circom_circuits!(
    "multiplier2_final.zkey",
    multiplier2_witness,
    "multiplier2_bls_final.zkey",
    multiplier2bls_witness,
    "keccak256_256_test_final.zkey",
    keccak256256test_witness,
);
// Circom Snipet End

// Halo2 Sample of using a single proving and verifying circuit

// Module containing the Halo2 circuit logic (FibonacciMoproCircuit)
mod halo2;

mopro_ffi::set_halo2_proving_circuits!("fibonacci_pk", halo2::FibonacciMoproCircuit::prove);
mopro_ffi::set_halo2_verifying_circuits!("fibonacci_vk", halo2::FibonacciMoproCircuit::verify);
// Halo2 Snipet End
