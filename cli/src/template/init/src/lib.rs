use circom_prover::witness::WitnessFn;

// Here we're calling a macro exported with Uniffi. This macro will
// write some functions and bind them to FFI type. These
// functions will invoke the `get_circom_wtns_fn` generated below.
mopro_ffi::app!();

// CIRCOM_TEMPLATE

// HALO2_TEMPLATE