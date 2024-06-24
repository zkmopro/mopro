use std::collections::HashMap;
use std::io;

use crate::FinbonaciCircuit;
use halo2_proofs::halo2curves::bn256::{Bn256, Fr, G1Affine};
use halo2_proofs::plonk::{create_proof, verify_proof, ProvingKey, VerifyingKey};
use halo2_proofs::poly::commitment::ParamsProver;
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2_proofs::poly::kzg::multiopen::{ProverGWC, VerifierGWC};
use halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_proofs::transcript::{
    Blake2bRead, Blake2bWrite, TranscriptReadBuffer, TranscriptWriterBuffer,
};
use halo2_proofs::SerdeFormat::RawBytes;
use rand::thread_rng;

/// This function is picked up by the `mopro-core` when generating the proof.
/// It should be implemented the proving logic for the circuit.
pub fn prove(
    inputs: HashMap<String, Vec<Fr>>,
    srs: &ParamsKZG<Bn256>,
    pk: &ProvingKey<G1Affine>,
) -> Result<(Vec<Fr>, Vec<u8>), String> {
    let circuit = FinbonaciCircuit::<Fr>::default();

    // Fix the starting values for the Fibonacci sequence
    let a = Fr::from(1); // F[0]
    let b = Fr::from(1); // F[1]
                         // Check that the `out` value is indeed the 9th Fibonacci number
    let out: Fr = inputs
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
        &[&[&public_input[..]]],
        thread_rng(),
        &mut transcript,
    )
    .unwrap();

    let proof = transcript.finalize();
    Ok((public_input, proof))
}

/// This function is picked up by the `mopro-core` when generating the proof.
/// It should be implemented the proving logic for the circuit.
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

/// Read a proving key from the file.
/// This function is picked up by the `mopro-core` when reading the proving key.
/// It has not been implemented in the `mopro-core` because some implementations might
/// have `halo2_proofs` `params` feature enabled, which changes the way the proving key is read,
/// To avoid compilation errors because of feature unification we have delegated the implementation.
pub fn read_pk<R: io::Read>(reader: &mut R) -> io::Result<ProvingKey<G1Affine>> {
    ProvingKey::read::<_, FinbonaciCircuit<_>>(reader, RawBytes)
}

/// Read a verification key from the file.
/// This function is picked up by the `mopro-core` when reading the proving key.
/// It has not been implemented in the `mopro-core` because some implementations might
/// have `halo2_proofs` `params` feature enabled, which changes the way the proving key is read,
/// To avoid compilation errors because of feature unification we have delegated the implementation.
pub fn read_vk<R: io::Read>(reader: &mut R) -> io::Result<VerifyingKey<G1Affine>> {
    VerifyingKey::read::<_, FinbonaciCircuit<_>>(reader, RawBytes)
}
