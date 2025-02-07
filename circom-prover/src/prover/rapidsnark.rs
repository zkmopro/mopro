use crate::CircomProof;
use anyhow::Result;
use ark_bn254::{Bn254, Fr};
use ark_ec::pairing::Pairing;
use ark_groth16::Proof;
use num::BigUint;
use num_bigint::BigInt;
use std::str::FromStr;
use std::thread::JoinHandle;

use super::serialization::{self, SerializableInputs, SerializableProof};

type Fq = ark_bn254::Fq;
type Fq2 = ark_bn254::Fq2;

pub fn generate_circom_proof(
    zkey_path: String,
    witness_thread: JoinHandle<Vec<BigUint>>,
) -> Result<CircomProof> {
    let witnesses = witness_thread
        .join()
        .map_err(|_e| anyhow::anyhow!("witness thread panicked"))
        .unwrap();
    let witnesses_bigint: Vec<BigInt> = witnesses.iter().map(|w| BigInt::from(w.clone())).collect();
    let wtns_buffer = rust_rapidsnark::parse_bigints_to_witness(witnesses_bigint).unwrap();
    let proof = rust_rapidsnark::groth16_prover_zkey_file_wrapper(&zkey_path, wtns_buffer).unwrap();
    let proof_json: serde_json::Value = serde_json::from_str(&proof.proof).unwrap();
    let public_signals_json: serde_json::Value =
        serde_json::from_str(&proof.public_signals).unwrap();

    let a_x = Fq::from_str(proof_json["pi_a"][0].as_str().unwrap()).unwrap();
    let a_y = Fq::from_str(proof_json["pi_a"][1].as_str().unwrap()).unwrap();
    let a = <Bn254 as Pairing>::G1Affine::new_unchecked(a_x, a_y);
    let c_x = Fq::from_str(proof_json["pi_c"][0].as_str().unwrap()).unwrap();
    let c_y = Fq::from_str(proof_json["pi_c"][1].as_str().unwrap()).unwrap();
    let c = <Bn254 as Pairing>::G1Affine::new_unchecked(c_x, c_y);
    let b1_x = Fq::from_str(proof_json["pi_b"][0][0].as_str().unwrap()).unwrap();
    let b1_y = Fq::from_str(proof_json["pi_b"][0][1].as_str().unwrap()).unwrap();
    // let b1 = <Bn254 as Pairing>::G2Affine::new_unchecked(b1_x, b1_y);
    let b1 = Fq2::new(b1_x, b1_y);
    let b2_x = Fq::from_str(proof_json["pi_b"][1][0].as_str().unwrap()).unwrap();
    let b2_y = Fq::from_str(proof_json["pi_b"][1][1].as_str().unwrap()).unwrap();
    let b2 = Fq2::new(b2_x, b2_y);
    let b = <Bn254 as Pairing>::G2Affine::new_unchecked(b1, b2);

    let ark_proof = Proof::<Bn254> { a, b, c };
    let public_signals: Vec<Fr> = public_signals_json
        .as_array()
        .unwrap()
        .iter()
        .map(|v| Fr::from_str(v.as_str().unwrap()).unwrap())
        .collect();

    Ok(CircomProof {
        proof: serialization::serialize_proof(&SerializableProof(ark_proof)),
        pub_inputs: serialization::serialize_inputs(&SerializableInputs::<Bn254>(public_signals)),
    })
}
