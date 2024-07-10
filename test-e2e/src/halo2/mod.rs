use std::collections::HashMap;
use std::path::Path;

use halo2_proofs::plonk::{create_proof, verify_proof, ProvingKey, VerifyingKey};
use halo2_proofs::poly::commitment::ParamsProver;
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2_proofs::poly::kzg::multiopen::{ProverGWC, VerifierGWC};
use halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_proofs::transcript::{
    Blake2bRead, Blake2bWrite, TranscriptReadBuffer, TranscriptWriterBuffer,
};
use halo2curves::bn256::{Bn256, Fr, G1Affine};
use serde::{Deserialize, Serialize};

use halo2_fibonacci::FibonacciCircuit;
use mopro_ffi::MoproError;

use crate::halo2::serialisation::deserialize_circuit_inputs;

mod serialisation;

type CircuitInputs = HashMap<String, Vec<Fr>>;
type Halo2Proof = Vec<u8>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableProof(pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct SerializablePublicInputs(pub Vec<Fr>);

pub(crate) struct FibonacciMoproCircuit;

impl FibonacciMoproCircuit {
    pub(crate) fn prove(
        srs_key_path: &str,
        proving_key_path: &str,
        input: HashMap<String, Vec<String>>,
    ) -> Result<mopro_ffi::GenerateProofResult, MoproError> {
        let circuit_inputs = deserialize_circuit_inputs(input).map_err(|e| {
            MoproError::Halo2Error(format!("Failed to deserialize circuit inputs: {}", e))
        })?;

        let srs = halo2_fibonacci::io::read_srs_path(Path::new(&srs_key_path));

        let proving_key =
            halo2_fibonacci::io::read_pk::<FibonacciCircuit<Fr>>(Path::new(&proving_key_path));

        let (proof, inputs) = generate_halo2_proof(&srs, &proving_key, circuit_inputs)
            .map_err(|e| MoproError::Halo2Error(format!("Failed to generate the proof: {}", e)))?;

        let serialized_inputs = bincode::serialize(&inputs).map_err(|e| {
            MoproError::Halo2Error(format!("Serialisation of Inputs failed: {}", e))
        })?;

        Ok(mopro_ffi::GenerateProofResult {
            proof,
            inputs: serialized_inputs,
        })
    }

    pub(crate) fn verify(
        srs_key_path: &str,
        verifying_key_path: &str,
        proof: Vec<u8>,
        public_inputs: Vec<u8>,
    ) -> Result<bool, MoproError> {
        let deserialized_inputs: SerializablePublicInputs = bincode::deserialize(&public_inputs)
            .map_err(|e| MoproError::Halo2Error(e.to_string()))?;

        let srs = halo2_fibonacci::io::read_srs_path(Path::new(&srs_key_path));

        let verifying_key =
            halo2_fibonacci::io::read_vk::<FibonacciCircuit<Fr>>(Path::new(&verifying_key_path));

        let is_valid =
            verify_halo2_proof(&srs, &verifying_key, proof, deserialized_inputs).unwrap();

        Ok(is_valid)
    }
}

fn generate_halo2_proof(
    srs: &ParamsKZG<Bn256>,
    proving_key: &ProvingKey<G1Affine>,
    inputs: CircuitInputs,
) -> Result<(Halo2Proof, SerializablePublicInputs), MoproError> {
    let circuit = FibonacciCircuit::<Fr>::default();

    // Generate the proof - so far using dummy inputs, will be replaced with actual inputs

    let a = Fr::from(1); // F[0]
    let b = Fr::from(1); // F[1]

    // `out` value right now must be 55, but will be replaced with the actual output value
    let out: Fr = inputs
        .get("out")
        .ok_or(MoproError::Halo2Error(
            "Failed to get `out` value".to_string(),
        ))?
        .get(0)
        .ok_or(MoproError::Halo2Error(
            "Failed to get `out` value".to_string(),
        ))?
        .clone();

    let mut transcript = TranscriptWriterBuffer::<_, G1Affine, _>::init(Vec::new());

    let public_input = vec![a, b, out];

    create_proof::<KZGCommitmentScheme<Bn256>, ProverGWC<_>, _, _, Blake2bWrite<_, _, _>, _>(
        srs,
        proving_key,
        &[circuit],
        vec![vec![public_input.clone().as_slice()].as_slice()].as_slice(),
        rand::thread_rng(),
        &mut transcript,
    )
    .map_err(|_| MoproError::Halo2Error("Failed to create the proof".to_string()))?;

    let proof = transcript.finalize();

    Ok((proof, SerializablePublicInputs(public_input)))
}

fn verify_halo2_proof(
    srs: &ParamsKZG<Bn256>,
    verifying_key: &VerifyingKey<G1Affine>,
    proof: Halo2Proof,
    inputs: SerializablePublicInputs,
) -> Result<bool, MoproError> {
    let mut transcript = TranscriptReadBuffer::<_, G1Affine, _>::init(proof.as_slice());
    verify_proof::<_, VerifierGWC<_>, _, Blake2bRead<_, _, _>, _>(
        srs.verifier_params(),
        verifying_key,
        SingleStrategy::new(srs),
        &[&[inputs.0.as_ref()]],
        &mut transcript,
    )
    .map_err(|_| MoproError::Halo2Error("Failed to verify the proof".to_string()))?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use super::*;

    const ASSETS_PATH: &str = "../test-vectors/halo2";

    static PROVING_KEY: Lazy<ProvingKey<G1Affine>> = Lazy::new(|| {
        halo2_fibonacci::io::read_pk::<FibonacciCircuit<Fr>>(Path::new(&format!(
            "{}/fibonacci_pk",
            ASSETS_PATH
        )))
    });

    static VERIFYING_KEY: Lazy<VerifyingKey<G1Affine>> = Lazy::new(|| {
        halo2_fibonacci::io::read_vk::<FibonacciCircuit<Fr>>(Path::new(&format!(
            "{}/fibonacci_vk",
            ASSETS_PATH
        )))
    });

    static SRS: Lazy<ParamsKZG<Bn256>> = Lazy::new(|| {
        halo2_fibonacci::io::read_srs_path(Path::new(&format!("{}/fibonacci_srs", ASSETS_PATH)))
    });

    #[test]
    fn test_generate_halo2_proof() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec![Fr::from(55)]);

        let (_, inputs) = generate_halo2_proof(&SRS, &PROVING_KEY, input).unwrap();
        assert_eq!(inputs.0, vec![Fr::from(1), Fr::from(1), Fr::from(55)]);
    }

    #[test]
    fn test_verify_halo2_proof() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec![Fr::from(55)]);

        let (proof, inputs) = generate_halo2_proof(&SRS, &PROVING_KEY, input).unwrap();
        let verified = verify_halo2_proof(&SRS, &VERIFYING_KEY, proof, inputs).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_bad_proof_not_verified() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec![Fr::from(56)]);

        let (proof, inputs) = generate_halo2_proof(&SRS, &PROVING_KEY, input).unwrap();
        let verified = verify_halo2_proof(&SRS, &VERIFYING_KEY, proof, inputs).unwrap_or(false);
        assert!(!verified);
    }

    #[test]
    fn test_prove_verify_end_to_end() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec!["55".to_string()]);

        let proving_key_path = format!("{}/fibonacci_pk", ASSETS_PATH);
        let verifying_key_path = format!("{}/fibonacci_vk", ASSETS_PATH);
        let srs_key_path = format!("{}/fibonacci_srs", ASSETS_PATH);

        let result = FibonacciMoproCircuit::prove(&srs_key_path, &proving_key_path, input).unwrap();
        let verified = FibonacciMoproCircuit::verify(
            &srs_key_path,
            &verifying_key_path,
            result.proof,
            result.inputs,
        )
        .unwrap();
        assert!(verified);
    }
}
