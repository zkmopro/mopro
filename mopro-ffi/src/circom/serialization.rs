use crate::{ProofCalldata, G1, G2};
use ark_bn254::Bn254;
use ark_circom::ethereum;
use ark_ec::pairing::Pairing;
use ark_groth16::{Proof, ProvingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use color_eyre::Result;

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
pub struct SerializableProvingKey<T: Pairing>(pub ProvingKey<T>);

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
pub struct SerializableProof<T: Pairing>(pub Proof<T>);

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug, PartialEq)]
pub struct SerializableInputs<T: Pairing>(pub Vec<T::ScalarField>);

pub fn serialize_proof<T: Pairing>(proof: &SerializableProof<T>) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    proof
        .serialize_uncompressed(&mut serialized_data)
        .expect("Serialization failed");
    serialized_data
}

pub fn deserialize_proof<T: Pairing>(data: Vec<u8>) -> SerializableProof<T> {
    SerializableProof::deserialize_uncompressed(&mut &data[..]).expect("Deserialization failed")
}

pub fn serialize_inputs<T: Pairing>(inputs: &SerializableInputs<T>) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    inputs
        .serialize_uncompressed(&mut serialized_data)
        .expect("Serialization failed");
    serialized_data
}

pub fn deserialize_inputs<T: Pairing>(data: Vec<u8>) -> SerializableInputs<T> {
    SerializableInputs::deserialize_uncompressed(&mut &data[..]).expect("Deserialization failed")
}

// Convert proof to U256-tuples as expected by the Solidity Groth16 Verifier
// Only supports bn254 for now
pub fn to_ethereum_proof(proof: Vec<u8>) -> ProofCalldata {
    let deserialized_proof = deserialize_proof::<Bn254>(proof);
    let proof = ethereum::Proof::from(deserialized_proof.0);
    let a = G1 {
        x: proof.a.x.to_string(),
        y: proof.a.y.to_string(),
    };
    let b = G2 {
        x: proof.b.x.iter().map(|x| x.to_string()).collect(),
        y: proof.b.y.iter().map(|x| x.to_string()).collect(),
    };
    let c = G1 {
        x: proof.c.x.to_string(),
        y: proof.c.y.to_string(),
    };
    ProofCalldata { a, b, c }
}

// Only supports bn254 for now
pub fn to_ethereum_inputs(inputs: Vec<u8>) -> Vec<String> {
    let deserialized_inputs = deserialize_inputs::<Bn254>(inputs);
    let inputs = deserialized_inputs
        .0
        .iter()
        .map(|x| x.to_string())
        .collect();
    inputs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circom::serialization::SerializableProvingKey;
    use anyhow::Result;
    use ark_bn254::Bn254;
    use ark_circom::circom::{r1cs_reader::R1CSFile, CircomCircuit};
    use ark_groth16::Groth16;
    use ark_std::rand::thread_rng;
    use std::{fs::File, path::Path};

    fn serialize_proving_key<T: Pairing>(pk: &SerializableProvingKey<T>) -> Vec<u8> {
        let mut serialized_data = Vec::new();
        pk.serialize_uncompressed(&mut serialized_data)
            .expect("Serialization failed");
        serialized_data
    }

    fn deserialize_proving_key<T: Pairing>(data: Vec<u8>) -> SerializableProvingKey<T> {
        SerializableProvingKey::deserialize_uncompressed(&mut &data[..])
            .expect("Deserialization failed")
    }

    fn generate_serializable_proving_key<T: Pairing>(
        r1cs_path: &str,
    ) -> Result<SerializableProvingKey<T>> {
        // Check that the files exist - ark-circom should probably do this instead and not panic
        if !Path::new(r1cs_path).exists() {
            anyhow::bail!("Path does not exist: {}", r1cs_path);
        }

        let mut circom = CircomCircuit {
            r1cs: R1CSFile::new(File::open(r1cs_path).unwrap())
                .unwrap()
                .into(),
            witness: None,
        } as CircomCircuit<T>;

        // Disable the wire mapping
        circom.r1cs.wire_mapping = None;

        let mut rng = thread_rng();
        let raw_params = Groth16::<T>::generate_random_parameters_with_reduction(circom, &mut rng)?;

        Ok(SerializableProvingKey::<T>(raw_params))
    }

    #[test]
    fn test_serialization_deserialization() {
        let r1cs_path = "../test-vectors/circom/multiplier2.r1cs";

        // Generate a serializable proving key for testing
        let serializable_pk = generate_serializable_proving_key::<Bn254>(r1cs_path)
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
