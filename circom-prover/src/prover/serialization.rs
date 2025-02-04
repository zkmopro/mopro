use anyhow::Result;
use ark_ec::pairing::Pairing;
use ark_groth16::{Proof, ProvingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

#[cfg(feature = "ethereum")]
use super::ethereum;
#[cfg(feature = "ethereum")]
use crate::{ProofCalldata, G1, G2};
#[cfg(feature = "ethereum")]
use ark_bn254::Bn254;

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

#[cfg(feature = "ethereum")]
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

#[cfg(feature = "ethereum")]
// Only supports bn254 for now
pub fn to_ethereum_inputs(inputs: Vec<u8>) -> Vec<String> {
    let deserialized_inputs = deserialize_inputs::<Bn254>(inputs);
    let inputs = ethereum::Inputs::from(&deserialized_inputs.0[..]);
    let inputs = inputs.0.iter().map(|x| x.to_string()).collect();
    inputs
}

#[cfg(test)]
mod tests {
    use crate::prover::ark_circom::{CircomCircuit, R1CSFile};

    use super::*;
    use anyhow::Result;
    use ark_bn254::Bn254;
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

    #[cfg(feature = "ethereum")]
    mod ethereum {
        use super::*;

        #[test]
        fn test_to_ethereum_proof() {
            let raw_proof = vec![
                22, 2, 28, 144, 134, 93, 1, 6, 180, 134, 137, 11, 130, 217, 116, 35, 22, 58, 213,
                215, 39, 9, 76, 99, 93, 46, 166, 183, 200, 20, 234, 26, 86, 182, 126, 104, 167,
                218, 67, 106, 232, 177, 113, 76, 217, 110, 167, 101, 215, 168, 67, 23, 2, 2, 50,
                131, 103, 159, 241, 197, 1, 75, 72, 154, 107, 226, 61, 6, 227, 5, 193, 103, 229,
                40, 232, 183, 170, 218, 136, 73, 194, 166, 135, 22, 128, 83, 94, 84, 179, 66, 38,
                17, 200, 0, 107, 4, 237, 57, 13, 157, 153, 39, 204, 59, 155, 91, 76, 89, 209, 195,
                76, 165, 72, 165, 188, 119, 12, 210, 184, 168, 78, 56, 125, 146, 97, 253, 159, 42,
                16, 203, 73, 47, 174, 29, 163, 124, 34, 156, 218, 243, 97, 226, 65, 123, 95, 132,
                40, 158, 63, 255, 94, 39, 196, 45, 251, 145, 188, 37, 155, 16, 201, 208, 50, 33,
                199, 98, 119, 172, 71, 240, 191, 110, 243, 225, 180, 215, 97, 98, 252, 124, 220,
                169, 163, 130, 43, 114, 242, 40, 46, 60, 6, 5, 51, 186, 24, 73, 62, 221, 213, 61,
                116, 62, 159, 150, 165, 183, 78, 86, 26, 236, 214, 9, 54, 152, 13, 135, 124, 137,
                89, 119, 212, 15, 212, 24, 181, 54, 115, 197, 150, 31, 22, 150, 210, 187, 28, 94,
                109, 138, 22, 234, 67, 58, 115, 199, 93, 121, 182, 221, 62, 212, 88, 84, 103, 215,
                109, 154,
            ];

            let proof = to_ethereum_proof(raw_proof);
            assert!(!proof.a.x.is_empty());
            assert!(!proof.a.y.is_empty());
            assert!(!proof.b.x.is_empty());
            assert!(!proof.b.y.is_empty());
            assert!(!proof.c.x.is_empty());
            assert!(!proof.c.y.is_empty());
        }

        #[test]
        fn test_to_ethereum_inputs() {
            let raw_inputs = vec![
                2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 147, 245, 225, 67, 145, 112, 185, 121, 72,
                232, 51, 40, 93, 88, 129, 129, 182, 69, 80, 184, 41, 160, 49, 225, 114, 78, 100,
                48, 0, 0, 0, 240, 147, 245, 225, 67, 145, 112, 185, 121, 72, 232, 51, 40, 93, 88,
                129, 129, 182, 69, 80, 184, 41, 160, 49, 225, 114, 78, 100, 48,
            ];
            let inputs = to_ethereum_inputs(raw_inputs);
            let expected_inputs = vec![
                "21888242871839275222246405745257275088548364400416034343698204186575808495616",
                "21888242871839275222246405745257275088548364400416034343698204186575808495616",
            ];
            assert_eq!(inputs, expected_inputs);
        }

        #[test]
        fn test_to_ethereum_inputs_with_zero() {
            let raw_inputs = vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let inputs = to_ethereum_inputs(raw_inputs);
            let expected_inputs = vec!["0".to_string()];
            assert_eq!(inputs, expected_inputs);
        }
    }
}
