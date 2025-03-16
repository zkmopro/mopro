use anyhow::Result;
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use ark_groth16::{Proof, ProvingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use num::BigUint;

use super::PublicInputs;
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

impl<T: Pairing> From<PublicInputs> for SerializableInputs<T> {
    fn from(src: PublicInputs) -> SerializableInputs<T> {
        let si = src
            .0
            .iter()
            .map(|n| {
                if *n > BigUint::from_bytes_le(T::ScalarField::MODULUS.to_bytes_le().as_ref()) {
                    panic!("Invalid input, larger than MODULUS")
                }
                T::ScalarField::from_le_bytes_mod_order(n.to_bytes_le().as_ref())
            })
            .collect();
        SerializableInputs(si)
    }
}

impl<T: Pairing> From<SerializableInputs<T>> for PublicInputs {
    fn from(src: SerializableInputs<T>) -> PublicInputs {
        let pi = src
            .0
            .iter()
            .map(|&si| BigUint::from_bytes_le(si.into_bigint().to_bytes_le().as_ref()))
            .collect();
        PublicInputs(pi)
    }
}

#[cfg(feature = "arkworks")]
#[cfg(test)]
mod tests {
    use crate::prover::ark_circom::{CircomCircuit, R1CSFile};

    use super::*;
    use anyhow::Result;
    use ark_bn254::Bn254;
    use ark_groth16::Groth16;
    use rand::thread_rng;
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
