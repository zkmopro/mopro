use ark_bn254::Bn254;
use ark_circom::{CircomBuilder, CircomConfig};
use ark_crypto_primitives::snark::SNARK;
use ark_groth16::Groth16;
use ark_std::rand::thread_rng;
use color_eyre::Result;

use mopro_core::middleware::circom::run_example;

fn main() {
    let _ = run_example();
}
