pub mod adapters;
pub mod bindings;
pub mod platforms;

#[allow(unused_imports)]
pub use adapters::*;
#[allow(unused_imports)]
pub use bindings::*;

/// This export is added for backwards compatibility.
#[cfg(feature = "circom")]
pub use crate::adapters::circom::witness;

/// This export is added for backwards compatibility.
pub use crate::platforms as app_config;

/// This macro is used to setup the Mopro FFI library
/// It should be included in the `lib.rs` file of the project
///
/// This should be used with the adapter-specific macros, such as `set_circom_circuits!(...)`
/// and `set_halo2_circuits!(...)`, etc.
///
/// # Circom Example
/// ```ignore
/// // Setup the Mopro FFI library
/// mopro_ffi::app!();
///
/// // Generate a Witness Generation function for the `multiplier2` circom circuit
/// rust_witness::witness!(multiplier2);
///
/// // Add `multiplier2` circom circuit to be exposed to the FFI
/// mopro_ffi::set_circom_circuits!(
///     "multiplier2_final.zkey",
///     WitnessFn::RustWitness(multiplier2_witness),
/// )
/// ```
///
/// # Halo2 Example
/// ```ignore
/// // Setup the Mopro FFI library
/// mopro_ffi::app!();
///
/// // Add `Fibonacci` circuit to generate proofs and verify proofs
/// mopro_ffi::set_halo2_circuits!(
///     "plonk_fibonacci_pk.bin",
///     plonk_fibonacci::prove,
///     "plonk_fibonacci_vk.bin",
///     plonk_fibonacci::verify
/// );
/// ```
///
/// # Noir Example
/// You don't need to generate Witness Generation functions first, like `Circom` or `Halo2` does.
/// All you need to do is to setup the Mopro FFi library as below.
///
/// ```ignore
/// // Setup the Mopro FFI library
/// mopro_ffi::app!();
///
/// ```
///
#[macro_export]
macro_rules! setup {
    () => {
        $crate::setup_bindings!();
        $crate::setup_adapters_common!();
    };
}

/// This macro is provided for backward compatibility.
#[macro_export]
macro_rules! app {
    () => {
        $crate::setup!();

        $crate::circom_setup!();
        $crate::halo2_setup!();
        $crate::noir_setup!();
    };
}
