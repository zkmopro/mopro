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
        uniffi::setup_scaffolding!();
    };
}

#[cfg(not(feature = "uniffi"))]
#[macro_export]
macro_rules! uniffi_setup {
    () => {
        // No-op when `uniffi` feature isn't enabled in `mopro_ffi`.
    };
}

#[cfg(feature = "flutter")]
pub use flutter_rust_bridge::*;

#[cfg(feature = "flutter")]
#[macro_export]
macro_rules! flutter_setup {
    () => {
        // ::uniffi must be available in the caller’s extern-prelude.
        extern crate mopro_ffi as flutter_rust_bridge;
        pub fn init_app() {
            // Default utilities - feel free to customize
            flutter_rust_bridge::setup_default_user_utils();
        }
    };
}

#[cfg(not(feature = "flutter"))]
#[macro_export]
macro_rules! flutter_setup {
    () => {};
}

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use console_error_panic_hook;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use getrandom;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use serde_wasm_bindgen;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use wasm_bindgen::*;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use wasm_bindgen_console_logger;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use wasm_bindgen_futures;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use wasm_bindgen_rayon::*;

#[macro_export]
macro_rules! wasm_setup {
    () => {
        extern crate mopro_ffi as wasm_bindgen;
        extern crate mopro_ffi as wasm_bindgen_rayon;
        use wasm_bindgen::prelude::*;
        use wasm_bindgen_rayon::init_thread_pool;
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
        mopro_ffi::flutter_setup!();
        #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
        mopro_ffi::wasm_setup!();
    };
}
