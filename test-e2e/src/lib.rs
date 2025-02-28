// Explicitly declare funtion to avoid Uniffi's limitation
use circom_prover::witness::WitnessFn;

// First, configure the Mopro FFI library
mopro_ffi::app!();

// --- Circom Example of setting up 4 circuits ---
rust_witness::witness!(multiplier2);
rust_witness::witness!(multiplier2bls);
rust_witness::witness!(keccak256256test);

mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", WitnessFn::RustWitness(multiplier2_witness)),
    ("multiplier2_bls_final.zkey", WitnessFn::RustWitness(multiplier2bls_witness)),
    ("keccak256_256_test_final.zkey", WitnessFn::RustWitness(keccak256256test_witness)),
}

// --- Halo2 Example of using Plonk proving and verifying circuits ---

// Module containing the Halo2 circuit logic (FibonacciMoproCircuit)

mopro_ffi::set_halo2_circuits! {
    ("plonk_fibonacci_pk.bin", plonk_fibonacci::prove, "plonk_fibonacci_vk.bin", plonk_fibonacci::verify),
    ("hyperplonk_fibonacci_pk.bin", hyperplonk_fibonacci::prove, "hyperplonk_fibonacci_vk.bin", hyperplonk_fibonacci::verify),
    ("gemini_fibonacci_pk.bin", gemini_fibonacci::prove, "gemini_fibonacci_vk.bin", gemini_fibonacci::verify),
    ("keccak256_pk.bin", halo2_keccak_256::prove, "keccak256_vk.bin", halo2_keccak_256::verify),
}
