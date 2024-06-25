use ark_bn254::Bn254;
use ark_circom::ethereum;
use ark_ec::pairing::Pairing;
use ark_groth16::{Proof, ProvingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use color_eyre::Result;

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
pub struct SerializableProvingKey(pub ProvingKey<Bn254>);

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
pub struct SerializableProof(pub Proof<Bn254>);

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug, PartialEq)]
pub struct SerializableInputs(pub Vec<<Bn254 as Pairing>::ScalarField>);

pub fn serialize_proof(proof: &SerializableProof) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    proof
        .serialize_uncompressed(&mut serialized_data)
        .expect("Serialization failed");
    serialized_data
}

pub fn deserialize_proof(data: Vec<u8>) -> SerializableProof {
    SerializableProof::deserialize_uncompressed(&mut &data[..]).expect("Deserialization failed")
}

pub fn serialize_inputs(inputs: &SerializableInputs) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    inputs
        .serialize_uncompressed(&mut serialized_data)
        .expect("Serialization failed");
    serialized_data
}

pub fn deserialize_inputs(data: Vec<u8>) -> SerializableInputs {
    SerializableInputs::deserialize_uncompressed(&mut &data[..]).expect("Deserialization failed")
}

// Convert proof to U256-tuples as expected by the Solidity Groth16 Verifier
pub fn to_ethereum_proof(proof: &SerializableProof) -> ethereum::Proof {
    ethereum::Proof::from(proof.0.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circom::serialization::SerializableProvingKey;
    use crate::MoproError;
    use ark_bn254::Bn254;
    use ark_circom::circom::{r1cs_reader::R1CSFile, CircomCircuit};
    use ark_groth16::Groth16;
    use ark_std::rand::thread_rng;
    use color_eyre::Result;
    use std::{fs::File, path::Path};

    type GrothBn = Groth16<Bn254>;

    fn serialize_proving_key(pk: &SerializableProvingKey) -> Vec<u8> {
        let mut serialized_data = Vec::new();
        pk.serialize_uncompressed(&mut serialized_data)
            .expect("Serialization failed");
        serialized_data
    }

    fn deserialize_proving_key(data: Vec<u8>) -> SerializableProvingKey {
        SerializableProvingKey::deserialize_uncompressed(&mut &data[..])
            .expect("Deserialization failed")
    }

    fn generate_serializable_proving_key(
        r1cs_path: &str,
    ) -> Result<SerializableProvingKey, MoproError> {
        // Check that the files exist - ark-circom should probably do this instead and not panic
        if !Path::new(r1cs_path).exists() {
            return Err(MoproError::CircomError(format!(
                "Path does not exist: {}",
                r1cs_path
            )));
        }

        let mut circom = CircomCircuit {
            r1cs: R1CSFile::new(File::open(r1cs_path).unwrap())
                .unwrap()
                .into(),
            witness: None,
        } as CircomCircuit<Bn254>;

        // Disable the wire mapping
        circom.r1cs.wire_mapping = None;

        let mut rng = thread_rng();
        let raw_params = GrothBn::generate_random_parameters_with_reduction(circom, &mut rng)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

        Ok(SerializableProvingKey(raw_params))
    }

    #[test]
    fn test_serialization_deserialization() {
        let r1cs_path = "../test-vectors/circom/multiplier2.r1cs";

        // Generate a serializable proving key for testing
        let serializable_pk = generate_serializable_proving_key(r1cs_path)
            .expect("Failed to generate serializable proving key");

        // Serialize
        let serialized_data = serialize_proving_key(&serializable_pk);

        // Deserialize
        let deserialized_pk = deserialize_proving_key(serialized_data);

        // Assert that the original and deserialized ProvingKeys are the same
        assert_eq!(
            serializable_pk.0, deserialized_pk.0,
            "Original and deserialized proving keys do not match"
        );
    }
}
