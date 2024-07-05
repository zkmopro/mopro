mod fibonacci;
pub mod io;
mod serialisation;

use std::collections::HashMap;
use std::io::Cursor;

pub(crate) use halo2_proofs::halo2curves::bn256::{Bn256, Fr as Fp, G1Affine};
use halo2_proofs::plonk::{create_proof, verify_proof, ProvingKey, VerifyingKey};
use halo2_proofs::poly::commitment::{Params, ParamsProver};
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2_proofs::poly::kzg::multiopen::{ProverGWC, VerifierGWC};
use halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_proofs::transcript::{
    Blake2bRead, Blake2bWrite, TranscriptReadBuffer, TranscriptWriterBuffer,
};
use halo2_proofs::SerdeFormat::RawBytes;
use mopro::MoproError;
use once_cell::sync::Lazy;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

pub use serialisation::deserialize_circuit_inputs;

pub use crate::halo2::fibonacci::FibonacciCircuit;

type CircuitInputs = HashMap<String, Vec<Fp>>;
type Halo2Proof = Vec<u8>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableProof(pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct SerializablePublicInputs(pub Vec<Fp>);

/// Read SRS from file
const SRS_BYTES: &[u8] = include_bytes!("../../test-vectors/halo2/fibonacci_srs");

static SRS: Lazy<ParamsKZG<Bn256>> = Lazy::new(|| {
    let mut reader = Cursor::new(SRS_BYTES);
    ParamsKZG::read(&mut reader).expect("Unable to read SRS from file")
});

/// Read Proving Key (PK) from file

const PK_BYTES: &[u8] = include_bytes!("../../test-vectors/halo2/fibonacci_pk");

static PK: Lazy<ProvingKey<G1Affine>> = Lazy::new(|| {
    let mut reader = Cursor::new(PK_BYTES);
    ProvingKey::read::<_, FibonacciCircuit<Fp>>(&mut reader, RawBytes)
        .expect("Unable to read PK from file")
});

/// Read Verification Key (VK) from file

const VK_BYTES: &[u8] = include_bytes!("../../test-vectors/halo2/fibonacci_vk");

static VK: Lazy<VerifyingKey<G1Affine>> = Lazy::new(|| {
    let mut reader = Cursor::new(VK_BYTES);
    VerifyingKey::read::<_, FibonacciCircuit<Fp>>(&mut reader, RawBytes)
        .expect("Unable to read VK from file")
});

pub fn generate_halo2_proof(
    inputs: CircuitInputs,
) -> Result<(Halo2Proof, SerializablePublicInputs), MoproError> {
    let circuit = FibonacciCircuit::<Fp>::default();

    // Generate the proof - so far using dummy inputs, will be replaced with actual inputs

    let a = Fp::from(1); // F[0]
    let b = Fp::from(1); // F[1]

    // `out` value right now must be 55, but will be replaced with the actual output value
    let out: Fp = inputs
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
        &SRS,
        &PK,
        &[circuit],
        vec![vec![public_input.clone().as_slice()].as_slice()].as_slice(),
        thread_rng(),
        &mut transcript,
    )
    .map_err(|_| MoproError::Halo2Error("Failed to create the proof".to_string()))?;

    let proof = transcript.finalize();

    Ok((proof, SerializablePublicInputs(public_input)))
}

pub fn verify_halo2_proof(
    proof: Halo2Proof,
    inputs: SerializablePublicInputs,
) -> Result<bool, MoproError> {
    let mut transcript = TranscriptReadBuffer::<_, G1Affine, _>::init(proof.as_slice());
    verify_proof::<_, VerifierGWC<_>, _, Blake2bRead<_, _, _>, _>(
        &SRS.verifier_params(),
        &VK,
        SingleStrategy::new(&SRS),
        &[&[inputs.0.as_ref()]],
        &mut transcript,
    )
    .map_err(|_| MoproError::Halo2Error("Failed to verify the proof".to_string()))?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_halo2_proof() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec![Fp::from(55)]);

        let (proof, inputs) = generate_halo2_proof(input).unwrap();
        assert_eq!(inputs.0[2], Fp::from(55));
    }

    #[test]
    fn test_verify_halo2_proof() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec![Fp::from(55)]);

        let (proof, inputs) = generate_halo2_proof(input).unwrap();
        let verified = verify_halo2_proof(proof, inputs).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_bad_proof_not_verified() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec![Fp::from(56)]);

        let (proof, inputs) = generate_halo2_proof(input).unwrap();
        let verified = verify_halo2_proof(proof, inputs).unwrap();
        assert!(!verified);
    }
}
