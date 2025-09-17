#![allow(unexpected_cfgs)]

#[cfg(feature = "build")]
pub mod app_config;

// UniFFI re-export
//
// Uniffi macros use fully qualified paths (`::uniffi::*`) internally.
// To allow downstream crates to transparently resolve these macros to `mopro_ffi`,
// users must alias it (`extern crate mopro_ffi as uniffi;`, automated via `app!` macro).
//
// However, for this alias to work correctly, `mopro_ffi` must provide the exact same
// exported items as the original `uniffi`. Hence, we re-export all individual items.
#[cfg(feature = "uniffi")]
pub use uniffi::*;

#[cfg(feature = "uniffi")]
#[macro_export]
macro_rules! uniffi_setup {
    () => {
        // `::uniffi` must be available in the caller’s extern-prelude.
        extern crate mopro_ffi as uniffi;
    };
}

#[cfg(not(feature = "uniffi"))]
#[macro_export]
macro_rules! uniffi_setup {
    () => {
        // No-op when `uniffi` feature isn't enabled in `mopro_ffi`.
    };
}

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
///
/// Noir integration supports two hash functions for different use cases:
/// - **Poseidon hash**: Default choice, optimized for performance and off-chain verification
/// - **Keccak256 hash**: Required for Solidity verifier compatibility and on-chain verification
///
/// The hash function is automatically selected based on the `on_chain` parameter:
/// - `on_chain = false` → Uses Poseidon (better performance)
/// - `on_chain = true` → Uses Keccak256 (Solidity compatible)
///
/// Reference: https://noir-lang.org/docs/how_to/how-to-solidity-verifier
///
/// You don't need to generate Witness Generation functions first, like `Circom` or `Halo2` does.
/// All you need to do is to setup the Mopro FFI library as below.
///
/// ```ignore
/// // Setup the Mopro FFI library
/// mopro_ffi::app!();
///
/// ```
///
#[macro_export]
macro_rules! app {
    () => {
        mopro_ffi::uniffi_setup!();
        uniffi::setup_scaffolding!();

        // This should be declared into this macro due to Uniffi's limitation
        // Please refer this issue: https://github.com/mozilla/uniffi-rs/issues/2257
        #[derive(Debug, thiserror::Error)]
        #[cfg_attr(feature = "uniffi", uniffi::Error)]
        pub enum MoproError {
            #[error("CircomError: {0}")]
            CircomError(String),
            #[error("Halo2Error: {0}")]
            Halo2Error(String),
            #[error("NoirError: {0}")]
            NoirError(String),
        }
    };
}
