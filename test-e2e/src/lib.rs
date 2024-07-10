mopro_ffi::setup_mopro!();

rust_witness::witness!(multiplier2);
mopro_ffi::mopro_circom_circuit!(Multiplier2CircomCircuit, multiplier2_witness);

rust_witness::witness!(keccak256256test);
mopro_ffi::mopro_circom_circuit!(Keccak256256TestCircomCircuit, keccak256256test_witness);
