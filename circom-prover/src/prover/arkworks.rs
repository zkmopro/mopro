use ark_bls12_381::Bls12_381;
use ark_bn254::Bn254;
use ark_circom::{read_zkey, CircomReduction, FieldSerialization, ZkeyHeaderReader};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use ark_groth16::{Groth16, ProvingKey};
use ark_relations::r1cs::ConstraintMatrices;
use ark_std::rand::thread_rng;
use ark_std::UniformRand;
use serialization::{SerializableInputs, SerializableProof};

use anyhow::Result;
use num_bigint::BigUint;
use std::fs::File;

use super::{serialization, ProofResult};

pub fn generate_circom_proof(zkey_path: String, witnesses: Vec<BigUint>) -> Result<ProofResult> {
    // here we make a loader just to get the groth16 header
    // this header tells us what curve the zkey was compiled for
    // this loader will only load the first few bytes
    let mut header_reader = ZkeyHeaderReader::new(&zkey_path);
    header_reader.read();
    let file = File::open(&zkey_path)?;
    let mut reader = std::io::BufReader::new(file);
    // check the prime in the header
    // println!("{} {} {}", header.q, header.n8q, ark_bls12_381::Fq::MODULUS);
    if header_reader.r == BigUint::from(ark_bn254::Fr::MODULUS) {
        let (proving_key, matrices) = read_zkey::<_, Bn254>(&mut reader)?;
        prove(proving_key, matrices, witnesses)
    } else if header_reader.r == BigUint::from(ark_bls12_381::Fr::MODULUS) {
        let (proving_key, matrices) = read_zkey::<_, Bls12_381>(&mut reader)?;
        prove(proving_key, matrices, witnesses)
    } else {
        panic!("unknown curve detected in zkey");
    }
}

fn prove<T: Pairing + FieldSerialization>(
    pkey: ProvingKey<T>,
    matrices: ConstraintMatrices<T::ScalarField>,
    witness: Vec<BigUint>,
) -> Result<ProofResult> {
    let witness_fr = witness
        .iter()
        .map(|v| T::ScalarField::from(v.clone()))
        .collect::<Vec<_>>();
    let mut rng = thread_rng();
    let rng = &mut rng;
    let r = T::ScalarField::rand(rng);
    let s = T::ScalarField::rand(rng);
    let public_inputs = witness_fr.as_slice()[1..matrices.num_instance_variables].to_vec();

    // build the proof
    let ark_proof = Groth16::<T, CircomReduction>::create_proof_with_reduction_and_matrices(
        &pkey,
        r,
        s,
        &matrices,
        matrices.num_instance_variables,
        matrices.num_constraints,
        witness_fr.as_slice(),
    );

    let proof = ark_proof?;

    Ok(ProofResult {
        proof: serialization::serialize_proof(&SerializableProof(proof)),
        pub_inputs: serialization::serialize_inputs(&SerializableInputs::<T>(public_inputs)),
    })
}
