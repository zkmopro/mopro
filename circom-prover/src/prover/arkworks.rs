use anyhow::Result;
use ark_bls12_381::Bls12_381;
use ark_bn254::Bn254;
use ark_crypto_primitives::snark::SNARK;
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use ark_groth16::{prepare_verifying_key, Groth16, ProvingKey, VerifyingKey};
use ark_relations::r1cs::ConstraintMatrices;
use ark_std::UniformRand;
use std::{fs::File, thread::JoinHandle};

use anyhow::bail;
use num_bigint::BigUint;
use rand::prelude::*;
use serialization::SerializableInputs;

use super::{
    ark_circom::{
        read_proving_key, read_zkey, CircomReduction, FieldSerialization, ZkeyHeaderReader,
    },
    serialization::{self, SerializableProof},
    CircomProof, PublicInputs,
};

pub fn generate_circom_proof(
    zkey_path: String,
    witness_thread: JoinHandle<Vec<BigUint>>,
) -> Result<CircomProof> {
    // here we make a loader just to get the groth16 header
    // this header tells us what curve the zkey was compiled for
    // this loader will only load the first few bytes
    let mut header_reader = ZkeyHeaderReader::new(&zkey_path);
    header_reader.read();
    let file = File::open(&zkey_path)?;
    let mut reader = std::io::BufReader::new(file);

    // check the prime in the header
    let (proof, pub_inputs) = if header_reader.r == BigUint::from(ark_bn254::Fr::MODULUS) {
        let (proving_key, matrices) = read_zkey::<_, Bn254>(&mut reader)?;
        // Get the result witness from the background thread
        let witnesses = witness_thread
            .join()
            .map_err(|_e| anyhow::anyhow!("witness thread panicked"))
            .unwrap();
        let (ark_proof, public_inputs) = prove(proving_key, matrices, witnesses).unwrap();
        (ark_proof.into(), PublicInputs(public_inputs))
    } else if header_reader.r == BigUint::from(ark_bls12_381::Fr::MODULUS) {
        let (proving_key, matrices) = read_zkey::<_, Bls12_381>(&mut reader)?;
        let witnesses = witness_thread
            .join()
            .map_err(|_e| anyhow::anyhow!("witness thread panicked"))
            .unwrap();
        let (ark_proof, public_inputs) = prove(proving_key, matrices, witnesses).unwrap();
        (ark_proof.into(), PublicInputs(public_inputs))
    } else {
        bail!("unknown curve detected in zkey")
    };

    Ok(CircomProof { proof, pub_inputs })
}

pub fn verify_circom_proof(zkey_path: String, proof: CircomProof) -> Result<bool> {
    let mut header_reader = ZkeyHeaderReader::new(&zkey_path);
    header_reader.read();
    let file = File::open(&zkey_path)?;
    let mut reader = std::io::BufReader::new(file);
    if header_reader.r == BigUint::from(ark_bn254::Fr::MODULUS) {
        let proving_key = read_proving_key::<_, Bn254>(&mut reader)?;
        let serialized_proof =
            serialization::serialize_proof::<Bn254>(&SerializableProof(proof.proof.into()));
        verify(proving_key.vk, serialized_proof, proof.pub_inputs)
    } else if header_reader.r == BigUint::from(ark_bls12_381::Fr::MODULUS) {
        let proving_key = read_proving_key::<_, Bls12_381>(&mut reader)?;
        let serialized_proof =
            serialization::serialize_proof::<Bls12_381>(&SerializableProof(proof.proof.into()));
        verify(proving_key.vk, serialized_proof, proof.pub_inputs)
    } else {
        // unknown curve
        bail!("unknown curve detected in zkey")
    }
}

fn prove<T: Pairing + FieldSerialization>(
    pkey: ProvingKey<T>,
    matrices: ConstraintMatrices<T::ScalarField>,
    witness: Vec<BigUint>,
) -> Result<(ark_groth16::Proof<T>, Vec<BigUint>)> {
    let witness_fr = witness
        .iter()
        .map(|v| T::ScalarField::from(v.clone()))
        .collect::<Vec<_>>();
    let mut rng = thread_rng();
    let rng = &mut rng;
    let r = T::ScalarField::rand(rng);
    let s = T::ScalarField::rand(rng);
    let public_inputs = witness_fr.as_slice()[1..matrices.num_instance_variables]
        .iter()
        .map(|scalar| BigUint::from_bytes_le(scalar.into_bigint().to_bytes_le().as_ref()))
        .collect::<Vec<BigUint>>();
    // build the proof
    let ark_proof = Groth16::<T, CircomReduction>::create_proof_with_reduction_and_matrices(
        &pkey,
        r,
        s,
        &matrices,
        matrices.num_instance_variables,
        matrices.num_constraints,
        witness_fr.as_slice(),
    )?;
    Ok((ark_proof, public_inputs))
}

fn verify<T: Pairing + FieldSerialization>(
    vk: VerifyingKey<T>,
    proof: Vec<u8>,
    pub_inputs: PublicInputs,
) -> Result<bool> {
    let pvk = prepare_verifying_key(&vk);
    let serialized_inputs: SerializableInputs<T> = pub_inputs.into();
    let public_inputs_fr = serialized_inputs.0.to_vec();
    let proof_parsed = serialization::deserialize_proof::<T>(proof);
    let verified = Groth16::<T, CircomReduction>::verify_with_processed_vk(
        &pvk,
        &public_inputs_fr,
        &proof_parsed.0,
    )?;
    Ok(verified)
}
