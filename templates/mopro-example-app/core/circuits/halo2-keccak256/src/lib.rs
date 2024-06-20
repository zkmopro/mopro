//! This is a sample Keccak256 implementation in Halo2 to be used in Mopro
//!
//!
//! Credit goes to https://github.com/privacy-scaling-explorations/zkevm-circuits/tree/main/zkevm-circuits/src/keccak_circuit
//!
//! This is a lookup table based implementation, where bytes are packed into big field elements as efficiently as possible.
//! The circuits can be configured to use different numbers of columns, by specifying the number of rows per internal
//! round of the keccak_f permutation.

/// This crate contains the functions picked up by the `mopro-core`.
/// **Make sure to re-implement these functions with your own circuit logic.**
mod mopro;
pub use mopro::{prove, read_pk, read_vk, verify};

pub mod circuit;
pub mod io;
pub(crate) mod util;
/// Module for Keccak low level implementation in vanilla halo2.
pub mod vanilla;

#[cfg(test)]
mod tests;

/// The number of rows in the Keccak Circuit
pub const K: u32 = 12;
/// The number of rows in each round of the Keccak Circuit
/// The larger the number, the more efficient the circuit, but the larger the K and proving key.
pub const ROWS_PER_ROUND: usize = 25;
pub const CIRCUIT_PARAMS: vanilla::KeccakConfigParams = vanilla::KeccakConfigParams {
    k: K,
    rows_per_round: ROWS_PER_ROUND,
};
