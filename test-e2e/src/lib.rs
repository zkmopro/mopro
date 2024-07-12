extern crate core;

// First, configure the Mopro FFI library
mopro_ffi::app!();

// --- Circom Example of setting up 4 circuits ---
rust_witness::witness!(multiplier2);
rust_witness::witness!(multiplier2bls);
rust_witness::witness!(keccak256256test);
rust_witness::witness!(hashbenchbls);

mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", multiplier2_witness),
    ("multiplier2_bls_final.zkey", multiplier2bls_witness),
    ("keccak256_256_test_final.zkey", keccak256256test_witness),
}

// --- Halo2 Example of using a single proving and verifying circuit ---

// Module containing the Halo2 circuit logic (FibonacciMoproCircuit)

mopro_ffi::set_halo2_circuits! {
    ("fibonacci_pk", halo2_fibonacci::prove, "fibonacci_vk", halo2_fibonacci::verify),
    ("keccak256_pk", halo2_keccak_256::prove, "keccak256_vk", halo2_keccak_256::verify),
}
