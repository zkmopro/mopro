//! The zkEVM keccak circuit implementation, with some modifications.
//! Credit goes to https://github.com/privacy-scaling-explorations/zkevm-circuits/tree/main/zkevm-circuits/src/keccak_circuit
//!
//! This is a lookup table based implementation, where bytes are packed into big field elements as efficiently as possible.
//! The circuits can be configured to use different numbers of columns, by specifying the number of rows per internal
//! round of the keccak_f permutation.

use std::collections::HashMap;

use halo2_proofs::halo2curves::bn256::{Bn256, Fr as Fp, Fr, G1Affine};
use halo2_proofs::plonk::{create_proof, verify_proof, ProvingKey, VerifyingKey};
use halo2_proofs::poly::commitment::ParamsProver;
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2_proofs::poly::kzg::multiopen::{ProverSHPLONK, VerifierSHPLONK};
use halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_proofs::transcript::{
    Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
};
use rand::thread_rng;

use crate::circuit::{pack_input_to_instance, KeccakCircuit};
use crate::util::prime_field::ScalarField;
use crate::vanilla::KeccakConfigParams;

pub mod circuit;
pub mod io;
pub mod util;
/// Module for Keccak circuits in vanilla halo2.
pub mod vanilla;

#[cfg(test)]
mod tests;

pub const K: u32 = 12;
pub const ROWS_PER_ROUND: usize = 25;

/// This function is picked up by the `mopro-core` when generating the proof.
/// It should be implemented the proving logic for the circuit.
pub fn prove(
    inputs: HashMap<String, Vec<Fr>>,
    srs: &ParamsKZG<Bn256>,
    pk: &ProvingKey<G1Affine>,
) -> Result<(Vec<Fp>, Vec<u8>), String> {
    // Get inputs by name "input" from the inputs hashmap
    let raw_inputs = inputs
        .get("input")
        .ok_or("`input` value not found in proof input".to_string())?;

    // Convert the raw inputs to a vector of u8
    // TODO - can be optimized by packing multiple bytes into field elements
    let inputs = vec![raw_inputs
        .iter()
        .map(|x| *x.to_bytes_le().first().unwrap())
        .collect::<Vec<u8>>()];

    let instance = pack_input_to_instance::<Fr>(&inputs);

    // Set up the circuit
    let circuit = KeccakCircuit::new(
        KeccakConfigParams {
            k: K,
            rows_per_round: ROWS_PER_ROUND,
        },
        Some(2usize.pow(K)),
        inputs,
        true, // Prover side-check to verify the circuit correctly computes the hash
        true, // Use the instance column for the input
    );

    let mut transcript = Blake2bWrite::<_, G1Affine, Challenge255<_>>::init(vec![]);

    create_proof::<
        KZGCommitmentScheme<Bn256>,
        ProverSHPLONK<'_, Bn256>,
        Challenge255<G1Affine>,
        _,
        Blake2bWrite<Vec<u8>, G1Affine, Challenge255<G1Affine>>,
        _,
    >(
        &srs,
        &pk,
        &[circuit],
        &[&[&instance[..]]],
        thread_rng(),
        &mut transcript,
    )
    .unwrap();

    let proof = transcript.finalize();
    Ok((instance, proof))
}

pub fn verify(
    proof: Vec<u8>,
    inputs: &Vec<Fr>,
    srs: &ParamsKZG<Bn256>,
    vk: &VerifyingKey<G1Affine>,
) -> Result<bool, ()> {
    let mut transcript = Blake2bRead::<_, G1Affine, Challenge255<_>>::init(&proof[..]);
    let proof_verified = verify_proof::<
        KZGCommitmentScheme<Bn256>,
        VerifierSHPLONK<'_, Bn256>,
        Challenge255<G1Affine>,
        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
        _,
    >(
        srs.verifier_params(),
        &vk,
        SingleStrategy::new(&srs),
        &[&[&inputs[..]]],
        &mut transcript,
    )
    .is_ok();
    Ok(proof_verified)
}

#[cfg(test)]
#[test]
fn test_conversion() {
    let input = vec![0u8, 151u8, 200u8, 255u8];
    // Convert the input to field elements
    let f_input = input.iter().map(|x| Fr::from(*x as u64));

    // Convert the field elements back to bytes
    let output = f_input
        .map(|x| x.to_bytes_le()[0] as u8)
        .collect::<Vec<u8>>();
    assert_eq!(input, output);
}
