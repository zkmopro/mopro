pub mod prover;
pub mod witness;

use anyhow::Result;
use prover::{CircomProof, ProofLib};
use std::collections::HashMap;

#[cfg(feature = "rustwitness")]
pub use rust_witness::*;
use witness::WitnessFn;
#[cfg(feature = "witnesscalc")]
pub use witnesscalc_adapter;

#[derive(Debug, Clone, Default)]
pub struct G1 {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Default)]
pub struct G2 {
    pub x: Vec<String>,
    pub y: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProofCalldata {
    pub a: G1,
    pub b: G2,
    pub c: G1,
}

#[derive(Debug, Clone)]
pub struct CircomPorver {}

impl CircomPorver {
    pub fn prove(
        proof_lib: ProofLib,
        wit_fn: WitnessFn,
        inputs: HashMap<String, Vec<String>>,
        zkey_path: String,
    ) -> Result<CircomProof> {
        let wit_thread = witness::generate_witness(wit_fn, inputs);
        prover::prove(proof_lib, zkey_path.clone(), wit_thread)
    }

    pub fn verify(
        proof_lib: ProofLib,
        proof: Vec<u8>,
        public_inputs: Vec<u8>,
        zkey_path: String,
    ) -> Result<bool> {
        prover::verify(proof_lib, zkey_path, proof, public_inputs)
    }
}
