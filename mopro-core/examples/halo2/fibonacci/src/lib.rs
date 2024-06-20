//! This is a sample Fibonacci implementation in Halo2 to be used with Mopro

/// This crate contains the functions picked up by the `mopro-core`.
/// **Make sure to re-implement these functions with your own circuit logic.**
mod mopro;
pub use mopro::{prove, read_pk, read_vk, verify};

pub(crate) mod circuit;
pub(crate) mod io;

pub use circuit::FinbonaciCircuit;
pub use io::{write_keys, write_srs};
