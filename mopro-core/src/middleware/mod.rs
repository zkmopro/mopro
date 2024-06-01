#[cfg(not(feature = "halo2"))]
pub mod circom;

#[cfg(all(feature = "gpu-benchmarks", not(feature = "halo2")))]
pub mod gpu_explorations;

#[cfg(feature = "halo2")]
pub mod halo2;
