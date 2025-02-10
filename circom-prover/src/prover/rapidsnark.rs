use crate::CircomProof;
use anyhow::Result;
use ark_bn254::{Bn254, Fr};
use ark_ec::pairing::Pairing;
use ark_groth16::{prepare_verifying_key, Proof};
use num::BigUint;
use num_bigint::BigInt;
use serde_json::json;
use std::thread::JoinHandle;
use std::{fs::File, str::FromStr};

use super::{
    ark_circom::read_proving_key,
    serialization::{self, SerializableInputs, SerializableProof},
};

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

pub fn verify_circom_proof(
    zkey_path: String,
    proof: Vec<u8>,
    public_inputs: Vec<u8>,
) -> Result<bool> {
    let proof_parsed = serialization::deserialize_proof::<Bn254>(proof);
    let public_inputs_parsed = serialization::deserialize_inputs::<Bn254>(public_inputs);

    let pi_a: Vec<String> = vec![
        proof_parsed.0.a.x.to_string(),
        proof_parsed.0.a.y.to_string(),
        "1".to_string(),
    ];

    let pi_b: Vec<Vec<String>> = vec![
        vec![
            proof_parsed.0.b.x.c0.to_string(),
            proof_parsed.0.b.x.c1.to_string(),
        ],
        vec![
            proof_parsed.0.b.y.c0.to_string(),
            proof_parsed.0.b.y.c1.to_string(),
        ],
        vec!["1".to_string(), "0".to_string()],
    ];

    let pi_c: Vec<String> = vec![
        proof_parsed.0.c.x.to_string(),
        proof_parsed.0.c.y.to_string(),
        "1".to_string(),
    ];

    let proof_json = json!({
        "pi_a": pi_a,
        "pi_c": pi_c,
        "pi_b": pi_b,
        "protocol": "groth16",
    });

    let inputs_json = json!(public_inputs_parsed
        .clone()
        .0
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>());

    let file = File::open(&zkey_path)?;
    let mut reader = std::io::BufReader::new(file);
    let proving_key = read_proving_key::<_, Bn254>(&mut reader)?;
    let pvk = prepare_verifying_key(&proving_key.vk);
    let ic = pvk
        .vk
        .gamma_abc_g1
        .iter()
        .map(|g| vec![g.x.to_string(), g.y.to_string(), "1".to_string()])
        .collect::<Vec<Vec<String>>>();
    let vkey_json = json!({
        "protocol": "groth16",
        "curve": "bn128",
        "nPublic": public_inputs_parsed.0.len(),
        "vk_alpha_1": vec![
            pvk.vk.alpha_g1.x.to_string(),
            pvk.vk.alpha_g1.y.to_string(),
            "1".to_string(),
        ],
        "vk_beta_2": vec![
            vec![
                pvk.vk.beta_g2.x.c0.to_string(),
                pvk.vk.beta_g2.x.c1.to_string(),
            ],
            vec![
                pvk.vk.beta_g2.y.c0.to_string(),
                pvk.vk.beta_g2.y.c1.to_string(),
            ],
            vec!["1".to_string(), "0".to_string()],
        ],
        "vk_gamma_2": vec![
            vec![
                pvk.vk.gamma_g2.x.c0.to_string(),
                pvk.vk.gamma_g2.x.c1.to_string(),
            ],
            vec![
                pvk.vk.gamma_g2.y.c0.to_string(),
                pvk.vk.gamma_g2.y.c1.to_string(),
            ],
            vec!["1".to_string(), "0".to_string()],
        ],
        "vk_delta_2": vec![
            vec![
                pvk.vk.delta_g2.x.c0.to_string(),
                pvk.vk.delta_g2.x.c1.to_string(),
            ],
            vec![
                pvk.vk.delta_g2.y.c0.to_string(),
                pvk.vk.delta_g2.y.c1.to_string(),
            ],
            vec!["1".to_string(), "0".to_string()],
        ],
        "vk_alphabeta_12": vec![
            vec![
                vec![
                    pvk.alpha_g1_beta_g2.c0.c0.c0.to_string(),
                    pvk.alpha_g1_beta_g2.c0.c0.c1.to_string(),
                ],
                vec![
                    pvk.alpha_g1_beta_g2.c0.c1.c0.to_string(),
                    pvk.alpha_g1_beta_g2.c0.c1.c1.to_string(),
                ],
                vec![
                    pvk.alpha_g1_beta_g2.c0.c2.c0.to_string(),
                    pvk.alpha_g1_beta_g2.c0.c2.c1.to_string(),
                ],
            ],
            vec![
                vec![
                    pvk.alpha_g1_beta_g2.c1.c0.c0.to_string(),
                    pvk.alpha_g1_beta_g2.c1.c0.c1.to_string(),
                ],
                vec![
                    pvk.alpha_g1_beta_g2.c1.c1.c0.to_string(),
                    pvk.alpha_g1_beta_g2.c1.c1.c1.to_string(),
                ],
                vec![
                    pvk.alpha_g1_beta_g2.c1.c2.c0.to_string(),
                    pvk.alpha_g1_beta_g2.c1.c2.c1.to_string(),
                ],
            ],
        ],
        "IC": ic
    });
    let valid = rust_rapidsnark::groth16_verify_wrapper(
        &proof_json.to_string(),
        &inputs_json.to_string(),
        &vkey_json.to_string(),
    )?;

    Ok(valid)
}
