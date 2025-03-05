// --- Circom Example of setting up multiplier2 circuit ---
use circom_prover::witness::WitnessFn;

rust_witness::witness!(multiplier2);

mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", WitnessFn::RustWitness(multiplier2_witness))
}
