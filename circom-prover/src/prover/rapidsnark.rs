use super::circom::{CURVE_BLS12_381, CURVE_BN254};
use super::{ark_circom::read_proving_key, serialization::SerializableInputs, PublicInputs};
use crate::prover::circom::PROTOCOL_GROTH16;
use crate::CircomProof;
use anyhow::{bail, Result};
use ark_bn254::{Bn254, Fq, Fq2, Fr};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use ark_groth16::{prepare_verifying_key, PreparedVerifyingKey, Proof};
use num::BigUint;
use num_bigint::BigInt;
use serde_json::{json, Value};
use std::thread::JoinHandle;
use std::{fs::File, str::FromStr};

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
    let b1 = Fq2::new(b1_x, b1_y);
    let b2_x = Fq::from_str(proof_json["pi_b"][1][0].as_str().unwrap()).unwrap();
    let b2_y = Fq::from_str(proof_json["pi_b"][1][1].as_str().unwrap()).unwrap();
    let b2 = Fq2::new(b2_x, b2_y);
    let b = <Bn254 as Pairing>::G2Affine::new_unchecked(b1, b2);

    let ark_proof = Proof::<Bn254> { a, b, c };
    let public_signals: Vec<BigUint> = public_signals_json
        .as_array()
        .unwrap()
        .iter()
        .map(|v| Fr::from_str(v.as_str().unwrap()).unwrap())
        .map(|fr| BigUint::from(fr.into_bigint()))
        .collect();

    Ok(CircomProof {
        proof: ark_proof.into(),
        pub_inputs: PublicInputs(public_signals),
    })
}

// Only support Bn254 (Rapidsnark doesn't support BLS12-381 yet)
pub fn verify_circom_proof(zkey_path: String, proof: CircomProof) -> Result<bool> {
    if proof.proof.curve.eq(CURVE_BLS12_381) {
        bail!("Not support {} yet", CURVE_BLS12_381)
    }

    let public_inputs_parsed: SerializableInputs<Bn254> = proof.pub_inputs.into();
    let pi_a: Vec<String> = vec![
        proof.proof.a.x.to_string(),
        proof.proof.a.y.to_string(),
        proof.proof.a.z.to_string(),
    ];

    let pi_b: Vec<Vec<String>> = vec![
        vec![
            proof.proof.b.x[0].to_string(),
            proof.proof.b.x[1].to_string(),
        ],
        vec![
            proof.proof.b.y[0].to_string(),
            proof.proof.b.y[1].to_string(),
        ],
        vec![
            proof.proof.b.z[0].to_string(),
            proof.proof.b.z[1].to_string(),
        ],
    ];

    let pi_c: Vec<String> = vec![
        proof.proof.c.x.to_string(),
        proof.proof.c.y.to_string(),
        proof.proof.c.z.to_string(),
    ];

    let proof_json = json!({
        "pi_a": pi_a,
        "pi_c": pi_c,
        "pi_b": pi_b,
        "protocol": PROTOCOL_GROTH16,
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
    let vkey_json = prepare_vkey(pvk, ic, public_inputs_parsed.0.len(), CURVE_BN254);
    let valid = rust_rapidsnark::groth16_verify_wrapper(
        &proof_json.to_string(),
        &inputs_json.to_string(),
        &vkey_json.to_string(),
    )?;

    Ok(valid)
}

fn prepare_vkey(
    pvk: PreparedVerifyingKey<Bn254>,
    ic: Vec<Vec<String>>,
    public_input_len: usize,
    curve: &str,
) -> Value {
    json!({
        "protocol": PROTOCOL_GROTH16,
        "curve": curve,
        "nPublic": public_input_len,
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
    })
}
