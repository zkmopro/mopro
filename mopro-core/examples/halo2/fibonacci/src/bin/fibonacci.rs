use std::env;
use std::path::Path;

use halo2_proofs::halo2curves::bn256::{Bn256, Fr};
use halo2_proofs::plonk::{keygen_pk, keygen_vk};
use halo2_proofs::poly::commitment::ParamsProver;
use halo2_proofs::poly::kzg::commitment::ParamsKZG;

use halo2_circuit::{write_keys, write_srs, FinbonaciCircuit};

pub fn main() {
    // Get the project's root directory from the `CARGO_MANIFEST_DIR` environment variable
    let project_root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");

    // Create the path to the `out` directory under the project's root directory
    let out_dir = Path::new(&project_root).join("out");

    // Check if the `out` directory exists, if not, create it
    if !out_dir.exists() {
        std::fs::create_dir(&out_dir).expect("Unable to create out directory");
    }

    // Set up the circuit
    let k = 4;
    let circuit = FinbonaciCircuit::<Fr>::default();

    // Generate SRS
    let srs = ParamsKZG::<Bn256>::new(k);

    let srs_path = out_dir.join("fib_srs");
    write_srs(&srs, srs_path.as_path());

    // Generate the proving key - should be loaded from disk in production
    let vk = keygen_vk(&srs, &circuit).expect("keygen_vk should not fail");
    let vk_path = out_dir.join("fib_vk");

    let pk = keygen_pk(&srs, vk, &circuit).expect("keygen_pk should not fail");
    let pk_path = out_dir.join("fib_pk");

    write_keys(&pk, pk_path.as_path(), vk_path.as_path());

    println!("Preparation finished successfully.");
    println!("SRS stored in {}", srs_path.display());
    println!("Proving key stored in {}", pk_path.display());
    println!("Verification key stored in {}", vk_path.display());
}
