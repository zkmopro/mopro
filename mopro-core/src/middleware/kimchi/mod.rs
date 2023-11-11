use crate::MoproError;
//use ark_ff::fields::Fp256;
//use ark_ff::{biginteger::BigInteger256 as BigInteger, Fp256, FpParameters};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use kimchi::{
    bench::{self, BenchmarkCtx},
    poly_commitment::evaluation_proof::OpeningProof,
    proof::ProverProof,
};
use mina_curves::pasta::fields::FpParameters;
use mina_curves::pasta::{Fp, Vesta, VestaParameters};
use serde::{Deserialize, Serialize};
use std::time::Instant; // Import serde traits

// XXX: This is hacky just to get some basic proof/verification of Kimchi setup going

//#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableProverProof(pub ProverProof<Vesta, OpeningProof<Vesta>>);

// XXX: Something wrong with serialization traits
// pub struct SerializablePublicInputs(pub Vec<Fp256<FpParameters>>);
//#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct SerializablePublicInputs(pub Vec<Fp>);

// TODO: Public inputs should be part of API but couldn't get serialization to work within timebox
pub struct KimchiContext {
    ctx: BenchmarkCtx,
    public_inputs: Vec<Fp>, // XXX: Added to store public inputs
}

impl KimchiContext {
    // Initializes the benchmark context
    pub fn bench_init(srs_size: u32) -> Self {
        let ctx = BenchmarkCtx::new(srs_size);
        KimchiContext {
            ctx,
            public_inputs: Vec::new(),
        }
    }

    // Creates a proof and updates public inputs in the context
    pub fn create_proof(&mut self) -> SerializableProverProof {
        let start = Instant::now();
        let (proof, public_input) = self.ctx.create_proof();
        self.public_inputs = public_input; // Store the public inputs in the context
        println!("proof created in {}s", start.elapsed().as_secs());
        SerializableProverProof(proof)
    }

    // Verifies a proof using stored public inputs
    pub fn verify_proof(&self, proof: SerializableProverProof) -> bool {
        let start = Instant::now();
        let proof = proof.0;
        let result = self
            .ctx
            .batch_verification(&[(proof, self.public_inputs.clone())]);
        println!("proof verified in {}s", start.elapsed().as_secs());

        // XXX Assume it works? No result type
        return true;
    }
}

pub fn bench() {
    // context created in 21.2235 ms
    let start = Instant::now();
    let srs_size = 4;
    let ctx = BenchmarkCtx::new(srs_size);
    println!("testing bench code for SRS of size {srs_size}");
    println!("context created in {}s", start.elapsed().as_secs());

    // proof created in 7.1227 ms
    let start = Instant::now();
    let (proof, public_input) = ctx.create_proof();
    println!("proof created in {}s", start.elapsed().as_secs());

    // proof verified in 1.710 ms
    let start = Instant::now();
    ctx.batch_verification(&vec![(proof, public_input)]);
    println!("proof verified in {}", start.elapsed().as_secs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kimchi_bench() {
        bench();
    }

    #[test]
    fn test_kimchi_context() {
        // Initialize the KimchiContext with a sample SRS size
        let mut ctx = KimchiContext::bench_init(4);

        // Create a proof and store it in the context
        let proof = ctx.create_proof();

        // Verify the proof and assert that the verification is successful
        assert!(
            ctx.verify_proof(proof),
            "Proof verification should be successful"
        );
    }
}
