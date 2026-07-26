//! Generate `expected_garaga_calldata.json` from SnarkJS fixtures using Garaga v1.1.0.
//!
//! Usage:
//!   cargo run -p garaga-calldata-tests --bin gen-garaga-calldata-fixture

use garaga_calldata_tests::generate_circom_groth16_garaga_calldata;
use std::fs;
use std::path::PathBuf;

fn main() {
    let fixture_dir = PathBuf::from("cli/src/template/init/test-vectors/circom/garaga/bn254");
    let proof = fs::read_to_string(fixture_dir.join("proof.json")).expect("proof.json");
    let public = fs::read_to_string(fixture_dir.join("public.json")).expect("public.json");
    let vk =
        fs::read_to_string(fixture_dir.join("verification_key.json")).expect("verification_key");

    let calldata =
        generate_circom_groth16_garaga_calldata(proof, public, vk).expect("generate calldata");

    let out = fixture_dir.join("expected_garaga_calldata.json");
    let json = serde_json::to_string_pretty(&calldata).expect("serialize calldata");
    fs::write(&out, json).expect("write expected_garaga_calldata.json");
    println!("Wrote {} felts to {}", calldata.len(), out.display());
}
