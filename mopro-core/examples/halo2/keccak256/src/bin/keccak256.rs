use std::env;
use std::path::Path;

use halo2_proofs::halo2curves::bn256::Bn256;
use halo2_proofs::plonk::{keygen_pk, keygen_vk};
use halo2_proofs::poly::commitment::ParamsProver;
use halo2_proofs::poly::kzg::commitment::ParamsKZG;

use keccak_test_2::circuit::KeccakCircuit;
use keccak_test_2::io::{write_keys, write_srs};
use keccak_test_2::vanilla::KeccakConfigParams;
use keccak_test_2::{K, ROWS_PER_ROUND};

/// This binary is picked up by the `mopro prepare` command as a backup option to generate the
/// srs, as well as proving and verification keys for the circuit when the keys are not found in the
/// `<YOUR_CIRCUIT>/out` directory.
///
/// When integrating your own Halo2 circuit with the mopro-core library, you should:
/// 1. Provide your own implementation of the `<your_circuit_name>` binary that prepares the keys.
/// 2. Generate the keys yourself and store them in the `out` directory of your circuit directory.
pub fn main() {
    // Get the project's root directory from the `CARGO_MANIFEST_DIR` environment variable
    let project_root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");

    // Create the path to the `out` directory under the project's root directory
    let out_dir = Path::new(&project_root).join("out");

    // Check if the `out` directory exists, if not, create it
    if !out_dir.exists() {
        std::fs::create_dir(&out_dir).expect("Unable to create out directory");
    }

    // Circuit name
    let circuit_name = "keccak256";

    // Set up the circuit
    let circuit = KeccakCircuit::new(
        KeccakConfigParams {
            k: K,
            rows_per_round: ROWS_PER_ROUND,
        },
        Some(2usize.pow(K)),
        vec![],
        false,
        false,
    );
    // Generate SRS
    let srs = ParamsKZG::<Bn256>::new(K);

    let srs_path = out_dir.join(format!("{}_srs", circuit_name));
    write_srs(&srs, srs_path.as_path());

    // Generate the proving key - should be loaded from disk in production
    let vk = keygen_vk(&srs, &circuit).expect("keygen_vk should not fail");
    let vk_path = out_dir.join(format!("{}_vk", circuit_name));

    let pk = keygen_pk(&srs, vk, &circuit).expect("keygen_pk should not fail");
    let pk_path = out_dir.join(format!("{}_pk", circuit_name));

    write_keys(&pk, pk_path.as_path(), vk_path.as_path());

    println!("Circuit file preparation finished successfully.");
    println!("SRS stored in {}", srs_path.display());
    println!("Proving key stored in {}", pk_path.display());
    println!("Verification key stored in {}", vk_path.display());
}
