extern crate core;

// First, configure the Mopro FFI library
mopro_ffi::app!();

// --- Circom Example of setting up 4 circuits ---
witnesscalc_adapter::witness!(multiplier2);
rust_witness::witness!(multiplier2bls);
witnesscalc_adapter::witness!(keccak256_256_test);
rust_witness::witness!(hashbenchbls);

mopro_ffi::set_circom_circuits! {
    ("multiplier2.zkey", mopro_ffi::WtnsFn::WithDatArg(multiplier2_witness)),
    ("multiplier2_bls_final.zkey", mopro_ffi::WtnsFn::NoDatArg(multiplier2bls_witness)),
    ("keccak256_256_test.zkey", mopro_ffi::WtnsFn::WithDatArg(keccak256_256_test_witness)),
}

// --- Halo2 Example of using a single proving and verifying circuit ---

// Module containing the Halo2 circuit logic (FibonacciMoproCircuit)

mopro_ffi::set_halo2_circuits! {
    ("plonk_fibonacci_pk.bin", plonk_fibonacci::prove, "plonk_fibonacci_vk.bin", plonk_fibonacci::verify),
    ("hyperplonk_fibonacci_pk.bin", hyperplonk_fibonacci::prove, "hyperplonk_fibonacci_vk.bin", hyperplonk_fibonacci::verify),
    ("gemini_fibonacci_pk.bin", gemini_fibonacci::prove, "gemini_fibonacci_vk.bin", gemini_fibonacci::verify),
    ("keccak256_pk.bin", halo2_keccak_256::prove, "keccak256_vk.bin", halo2_keccak_256::verify),
}
