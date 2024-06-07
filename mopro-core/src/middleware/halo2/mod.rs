use std::collections::HashMap;
use std::io::Cursor;
use std::time::Instant;

pub(crate) use halo2_proofs::halo2curves::bn256::{Bn256, Fr as Fp, G1Affine};
use halo2_proofs::plonk::{ProvingKey, VerifyingKey};
use halo2_proofs::poly::commitment::Params;
use halo2_proofs::poly::kzg::commitment::ParamsKZG;
use halo2_proofs::SerdeFormat::RawBytes;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use halo2_circuit::{prove, verify, Circuit as TargetCircuit};
pub use serialisation::deserialize_circuit_inputs;

use crate::MoproError;

mod serialisation;

type CircuitInputs = HashMap<String, Vec<Fp>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableProof(pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct SerializablePublicInputs(pub Vec<Fp>);

/// Read SRS from file
const SRS_BYTES: &[u8] = include_bytes!(env!("BUILD_SRS_FILE"));

static SRS: Lazy<ParamsKZG<Bn256>> = Lazy::new(|| {
    let mut reader = Cursor::new(SRS_BYTES);
    ParamsKZG::read(&mut reader).expect("Unable to read SRS from file")
});

/// Read Proving Key (PK) from file

const PK_BYTES: &[u8] = include_bytes!(env!("BUILD_PK_FILE"));

static PK: Lazy<ProvingKey<G1Affine>> = Lazy::new(|| {
    let mut reader = Cursor::new(PK_BYTES);
    ProvingKey::read::<_, TargetCircuit<Fp>>(&mut reader, RawBytes)
        .expect("Unable to read PK from file")
});

/// Read Verification Key (VK) from file

const VK_BYTES: &[u8] = include_bytes!(env!("BUILD_VK_FILE"));

static VK: Lazy<VerifyingKey<G1Affine>> = Lazy::new(|| {
    let mut reader = Cursor::new(VK_BYTES);
    VerifyingKey::read::<_, TargetCircuit<Fp>>(&mut reader, RawBytes)
        .expect("Unable to read VK from file")
});

pub fn generate_halo2_proof2(
    inputs: CircuitInputs,
) -> color_eyre::Result<(SerializableProof, SerializablePublicInputs), MoproError> {
    let start = Instant::now();

    let (public_input, proof) =
        prove(inputs, &SRS, &PK).map_err(|e| MoproError::Halo2Error(e.to_string()))?;

    let proving_duration = start.elapsed();
    println!("Proving time 2: {:?}", proving_duration);

    Ok((
        SerializableProof(proof),
        SerializablePublicInputs(public_input),
    ))
}

pub fn verify_halo2_proof2(
    serialized_proof: SerializableProof,
    serialized_inputs: SerializablePublicInputs,
) -> color_eyre::Result<bool, MoproError> {
    let start = Instant::now();

    let proof = serialized_proof.0;
    let inputs = serialized_inputs.0;

    let proof_verified = verify(proof, &inputs, &SRS, &VK)
        .map_err(|_| MoproError::Halo2Error("Failed to verify the proof".to_string()))?;

    let verification_duration = start.elapsed();
    println!("Verification time 2: {:?}", verification_duration);
    Ok(proof_verified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_halo2_proof2() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec![Fp::from(55)]);

        let (proof, inputs) = generate_halo2_proof2(input).unwrap();
        assert_eq!(inputs.0[2], Fp::from(55));
    }

    #[test]
    fn test_verify_halo2_proof2() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec![Fp::from(55)]);

        let (proof, inputs) = generate_halo2_proof2(input).unwrap();
        let verified = verify_halo2_proof2(proof, inputs).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_bad_proof_not_verified() {
        let mut input = HashMap::new();
        input.insert("out".to_string(), vec![Fp::from(56)]);

        let (proof, inputs) = generate_halo2_proof2(input).unwrap();
        let verified = verify_halo2_proof2(proof, inputs).unwrap();
        assert!(!verified);
    }
}
