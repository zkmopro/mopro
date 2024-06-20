use crate::circuit::{pack_input_to_instance, unpack_input, KeccakCircuit};
use crate::vanilla::KeccakConfigParams;
use crate::{CIRCUIT_PARAMS, K, ROWS_PER_ROUND};
use halo2_proofs::halo2curves::bn256::{Bn256, Fr, G1Affine};
use halo2_proofs::plonk::{create_proof, verify_proof, ProvingKey, VerifyingKey};
use halo2_proofs::poly::commitment::ParamsProver;
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2_proofs::poly::kzg::multiopen::{ProverSHPLONK, VerifierSHPLONK};
use halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_proofs::transcript::{
    Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
};
use halo2_proofs::SerdeFormat::RawBytes;
use rand::thread_rng;
use std::collections::HashMap;
use std::io;

/// This function is picked up by the `mopro-core` when generating the proof.
/// It should be implemented the proving logic for the circuit.
pub fn prove(
    inputs: HashMap<String, Vec<Fr>>,
    srs: &ParamsKZG<Bn256>,
    pk: &ProvingKey<G1Affine>,
) -> Result<(Vec<Fr>, Vec<u8>), String> {
    // Get inputs by name "input" from the inputs hashmap
    let raw_inputs = inputs
        .get("input")
        .ok_or("`input` value not found in proof input".to_string())?;

    // Convert the raw inputs to a vector of u8
    // TODO - can be optimized by packing multiple bytes into field elements
    let inputs = vec![unpack_input(raw_inputs)];

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

/// This function is picked up by the `mopro-core` when generating the proof.
/// It should be implemented the proving logic for the circuit.
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

/// Read a proving key from the file.
/// This function is picked up by the `mopro-core` when reading the proving key.
/// It has not been implemented in the `mopro-core` because some implementations might
/// have `halo2_proofs` `params` feature enabled, which changes the way the proving key is read,
/// To avoid compilation errors because of feature unification we have delegated the implementation.
pub fn read_pk<R: io::Read>(reader: &mut R) -> io::Result<ProvingKey<G1Affine>> {
    ProvingKey::read::<_, KeccakCircuit<_>>(reader, RawBytes, CIRCUIT_PARAMS)
}

/// Read a verification key from the file.
/// This function is picked up by the `mopro-core` when reading the proving key.
/// It has not been implemented in the `mopro-core` because some implementations might
/// have `halo2_proofs` `params` feature enabled, which changes the way the proving key is read,
/// To avoid compilation errors because of feature unification we have delegated the implementation.
pub fn read_vk<R: io::Read>(reader: &mut R) -> io::Result<VerifyingKey<G1Affine>> {
    VerifyingKey::read::<_, KeccakCircuit<_>>(reader, RawBytes, CIRCUIT_PARAMS)
}
