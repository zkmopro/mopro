//! Generate `expected_garaga_calldata.json` from fixtures via generate_circom_groth16_garaga_calldata.
//!
//! Usage:
//!   cargo run -p garaga-calldata-tests --bin gen-garaga-calldata-fixture

use garaga_calldata_tests::{
    generate_circom_groth16_garaga_calldata, CircomProof, CircomProofResult, G1, G2,
};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct FixtureProof {
    pi_a: Vec<String>,
    pi_b: Vec<Vec<String>>,
    pi_c: Vec<String>,
    protocol: String,
    curve: String,
}

fn load_proof_result(fixture_dir: &Path) -> CircomProofResult {
    let proof: FixtureProof =
        serde_json::from_str(&fs::read_to_string(fixture_dir.join("proof.json")).expect("proof"))
            .expect("parse proof");
    let inputs: Vec<String> =
        serde_json::from_str(&fs::read_to_string(fixture_dir.join("public.json")).expect("public"))
            .expect("parse public");

    CircomProofResult {
        proof: CircomProof {
            a: G1 {
                x: proof.pi_a[0].clone(),
                y: proof.pi_a[1].clone(),
                z: proof
                    .pi_a
                    .get(2)
                    .cloned()
                    .unwrap_or_else(|| "1".to_string()),
            },
            b: G2 {
                x: proof.pi_b[0].clone(),
                y: proof.pi_b[1].clone(),
                z: proof
                    .pi_b
                    .get(2)
                    .cloned()
                    .unwrap_or_else(|| vec!["1".to_string(), "0".to_string()]),
            },
            c: G1 {
                x: proof.pi_c[0].clone(),
                y: proof.pi_c[1].clone(),
                z: proof
                    .pi_c
                    .get(2)
                    .cloned()
                    .unwrap_or_else(|| "1".to_string()),
            },
            protocol: proof.protocol,
            curve: proof.curve,
        },
        inputs,
    }
}

fn main() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/bn254");
    let vk =
        fs::read_to_string(fixture_dir.join("verification_key.json")).expect("verification_key");
    let proof_result = load_proof_result(&fixture_dir);

    let calldata =
        generate_circom_groth16_garaga_calldata(proof_result, vk).expect("generate calldata");

    let out = fixture_dir.join("expected_garaga_calldata.json");
    let json = serde_json::to_string_pretty(&calldata).expect("serialize calldata");
    fs::write(&out, json).expect("write expected_garaga_calldata.json");
    println!("Wrote {} felts to {}", calldata.len(), out.display());
}
