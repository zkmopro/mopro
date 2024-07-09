use mopro_ffi::{app, WtnsFn};

rust_witness::witness!(multiplier2);
rust_witness::witness!(multiplier2bls);
rust_witness::witness!(keccak256256test);
rust_witness::witness!(hashbenchbls);

app!();

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
