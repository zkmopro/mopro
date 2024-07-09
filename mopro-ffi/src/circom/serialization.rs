use crate::circom::ethereum;
use crate::{ProofCalldata, G1, G2};
use ark_bn254::Bn254;
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
