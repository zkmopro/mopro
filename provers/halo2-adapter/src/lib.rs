mod adapter;

pub use adapter::{Halo2Output, Halo2ProveFn, Halo2ProverAdapter, Halo2VerifyFn};

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
