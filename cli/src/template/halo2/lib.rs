// --- Halo2 Example of using a single proving and verifying circuit ---

// Module containing the Halo2 circuit logic (FibonacciMoproCircuit)

mopro_ffi::set_halo2_circuits! {
    ("fibonacci_pk.bin", halo2_fibonacci::prove, "fibonacci_vk.bin", halo2_fibonacci::verify),
}
