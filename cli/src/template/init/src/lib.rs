// Here we're generating the C witness generator functions
// for a circuit named `multiplier2`.
// Your circuit name will be the name of the wasm file all lowercase
// with spaces, dashes and underscores removed
//
// e.g.
// multiplier2 -> multiplier2
// keccak_256_256_main -> keccak256256main
// aadhaar-verifier -> aadhaarverifier
rust_witness::witness!(multiplier2);

// Here we're calling a macro exported by uniffi. This macro will
// write some functions and bind them to the uniffi UDL file. These
// functions will invoke the `get_circom_wtns_fn` generated below.
mopro_ffi::app!();

// This macro is used to define the `get_circom_wtns_fn` function
// which defines a mapping between zkey filename and witness generator.
// You can pass multiple comma seperated `(filename, witness_function)` pairs to it.
// You can read in the `circom` doc section how you can manually set this function.
// One way to create the witness generator function is to use the `rust_witness!` above.
mopro_ffi::set_circom_circuits! {
    ("multiplier2_final.zkey", multiplier2_witness),
}
