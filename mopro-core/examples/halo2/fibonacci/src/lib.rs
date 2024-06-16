mod fibonacci;
mod io;

pub use fibonacci::*;
use halo2_proofs::halo2curves::bn256::{Bn256, Fr as Fp, Fr, G1Affine};
use halo2_proofs::plonk::{create_proof, verify_proof, ProvingKey, VerifyingKey};
use halo2_proofs::poly::commitment::ParamsProver;
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2_proofs::poly::kzg::multiopen::{ProverGWC, VerifierGWC};
use halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_proofs::transcript::{
    Blake2bRead, Blake2bWrite, TranscriptReadBuffer, TranscriptWriterBuffer,
};
pub use io::*;
use rand::thread_rng;
use std::collections::HashMap;

pub use FinbonaciCircuit as Circuit;

// TODO - consider making this universal for different curves
pub fn prove(
    inputs: HashMap<String, Vec<Fp>>,
    srs: &ParamsKZG<Bn256>,
    pk: &ProvingKey<G1Affine>,
) -> Result<(Vec<Fp>, Vec<u8>), String> {
    let circuit = FinbonaciCircuit::<Fp>::default();

    // Generate the proof - so far using dummy inputs, will be replaced with actual inputs

    let a = Fp::from(1); // F[0]
    let b = Fp::from(1); // F[1]
    let out: Fp = inputs
        .get("out")
        .ok_or("`out` value not found in proof input".to_string())?
        .get(0)
        .ok_or("`out` value list is empty".to_string())
        .unwrap()
        .clone();

    let mut transcript = TranscriptWriterBuffer::<_, G1Affine, _>::init(Vec::new());

    let public_input = vec![a, b, out];

    create_proof::<KZGCommitmentScheme<Bn256>, ProverGWC<_>, _, _, Blake2bWrite<_, _, _>, _>(
        &srs,
        &pk,
        &[circuit],
        vec![vec![public_input.clone().as_slice()].as_slice()].as_slice(), // TODO - this might be wrong
        thread_rng(),
        &mut transcript,
    )
    .unwrap();

    let proof = transcript.finalize();
    Ok((public_input, proof))
}

pub fn verify(
    proof: Vec<u8>,
    inputs: &Vec<Fr>,
    srs: &ParamsKZG<Bn256>,
    vk: &VerifyingKey<G1Affine>,
) -> Result<bool, ()> {
    let mut transcript = TranscriptReadBuffer::<_, G1Affine, _>::init(proof.as_slice());
    let proof_verified = verify_proof::<_, VerifierGWC<_>, _, Blake2bRead<_, _, _>, _>(
        srs.verifier_params(),
        &vk,
        SingleStrategy::new(&srs),
        &[&[inputs.as_ref()]],
        &mut transcript,
    )
    .is_ok();
    Ok(proof_verified)
}
