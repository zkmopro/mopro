// --- Circom Example of setting up multiplier2 circuit ---
rust_witness::witness!(multiplier2);

mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", multiplier2_witness)
}
