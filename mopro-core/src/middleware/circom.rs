use crate::MoproError;

use ark_bn254::Bn254;
use ark_circom::{CircomBuilder, CircomCircuit, CircomConfig};
use ark_crypto_primitives::snark::SNARK;
use ark_ec::pairing::Pairing;
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_std::rand::{rngs::ThreadRng, thread_rng};
use color_eyre::Result;
use std::path::Path;

type GrothBn = Groth16<Bn254>;

// TODO: Do we want some kind of Circom object that we can use for setup, prove, verify etc?

fn assert_paths_exists(wasm_path: &str, r1cs_path: &str) -> Result<(), MoproError> {
    // Check that the files exist - ark-circom should probably do this instead and not panic
    if !Path::new(wasm_path).exists() {
        return Err(MoproError::CircomError(format!(
            "Path does not exist: {}",
            wasm_path
        )));
    }

    if !Path::new(r1cs_path).exists() {
        return Err(MoproError::CircomError(format!(
            "Path does not exist: {}",
            r1cs_path
        )));
    };

    Ok(())
}

pub fn setup(
    wasm_path: &str,
    r1cs_path: &str,
) -> Result<
    (
        ProvingKey<Bn254>,
        CircomCircuit<Bn254>,
        ThreadRng,
        Vec<<Bn254 as Pairing>::ScalarField>,
    ),
    MoproError,
> {
    assert_paths_exists(wasm_path, r1cs_path)?;

    println!("Setup");

    // Load the WASM and R1CS for witness and proof generation
    let cfg = CircomConfig::<Bn254>::new(wasm_path, r1cs_path)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    // Insert our inputs as key value pairs
    // In Circom this is the private input (witness) and public input
    // It does not include the public output
    let mut builder = CircomBuilder::new(cfg);

    // XXX: We probably want to do this separately, after trusted setup
    builder.push_input("a", 3);
    builder.push_input("b", 5);

    // Create an empty instance for setting it up
    let circom = builder.setup();

    // Run a trusted setup
    let mut rng = thread_rng();
    let params = GrothBn::generate_random_parameters_with_reduction(circom, &mut rng)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    // Get the populated instance of the circuit with the witness
    let circom = builder
        .build()
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    // This is the instance, public input and public output
    // Together with the witness provided above this satisfies the circuit
    let inputs = circom.get_public_inputs().unwrap();

    Ok((params, circom, rng, inputs))
}

pub fn generate_proof(
    params: ProvingKey<Bn254>,
    circom: CircomCircuit<Bn254>,
    rng: &mut ThreadRng,
) -> Result<Proof<Bn254>, MoproError> {
    // Generate the proof
    println!("Generating proof");
    let proof =
        GrothBn::prove(&params, circom, rng).map_err(|e| MoproError::CircomError(e.to_string()))?;

    Ok(proof)
}

pub fn verify_proof(
    params: ProvingKey<Bn254>,
    inputs: Vec<<Bn254 as Pairing>::ScalarField>,
    proof: Proof<Bn254>,
) -> Result<bool, MoproError> {
    // Check that the proof is valid
    println!("Verifying proof");
    let pvk =
        GrothBn::process_vk(&params.vk).map_err(|e| MoproError::CircomError(e.to_string()))?;
    let proof_verified = GrothBn::verify_with_processed_vk(&pvk, &inputs, &proof)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;
    match proof_verified {
        true => println!("Proof verified"),
        false => println!("Proof not verified"),
    }
    assert!(proof_verified);

    // To provide the instance manually, i.e. public input and output in Circom:
    println!("Verifying proof with manual inputs");
    let c = 15;
    let a = 3;
    let inputs_manual = vec![c.into(), a.into()];

    let verified_alt = GrothBn::verify_with_processed_vk(&pvk, &inputs_manual, &proof)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;
    //println!("Proof verified (alt): {}", verified_alt);
    assert!(verified_alt);

    Ok(proof_verified)
}

// TODO: Refactor this to be a proper API with setup, prove, verify etc
// This is just a temporary function to get things working end-to-end.
// Later we call as native Rust in example, and use from mopro-ffi
pub fn run_example(wasm_path: &str, r1cs_path: &str) -> Result<(), MoproError> {
    assert_paths_exists(wasm_path, r1cs_path)?;

    println!("Setup");

    // Load the WASM and R1CS for witness and proof generation
    let cfg = CircomConfig::<Bn254>::new(wasm_path, r1cs_path)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    // Insert our inputs as key value pairs
    // In Circom this is the private input (witness) and public input
    // It does not include the public output
    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 5);

    // Create an empty instance for setting it up
    let circom = builder.setup();

    // Run a trusted setup
    let mut rng = thread_rng();
    let params = GrothBn::generate_random_parameters_with_reduction(circom, &mut rng)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    // Get the populated instance of the circuit with the witness
    let circom = builder
        .build()
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    // This is the instance, public input and public output
    // Together with the witness provided above this satisfies the circuit
    let inputs = circom.get_public_inputs().unwrap();

    // Generate the proof
    println!("Generating proof");
    let proof = GrothBn::prove(&params, circom, &mut rng)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    // Check that the proof is valid
    println!("Verifying proof");
    let pvk =
        GrothBn::process_vk(&params.vk).map_err(|e| MoproError::CircomError(e.to_string()))?;
    let proof_verified = GrothBn::verify_with_processed_vk(&pvk, &inputs, &proof)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;
    match proof_verified {
        true => println!("Proof verified"),
        false => println!("Proof not verified"),
    }
    assert!(proof_verified);

    // To provide the instance manually, i.e. public input and output in Circom:
    let c = 15;
    let a = 3;
    let inputs_manual = vec![c.into(), a.into()];

    let verified_alt = GrothBn::verify_with_processed_vk(&pvk, &inputs_manual, &proof)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;
    //println!("Proof verified (alt): {}", verified_alt);
    assert!(verified_alt);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_example_ok() {
        let wasm_path = "./examples/circom/target/multiplier2_js/multiplier2.wasm";
        let r1cs_path = "./examples/circom/target/multiplier2.r1cs";
        let result = run_example(wasm_path, r1cs_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_example_err() {
        let result = run_example("foo", "bar");
        assert!(result.is_err());
    }

    // TODO: Just a stub, make this work
    #[test]
    fn test_setup_prove_verify_err() {
        let result = setup();
        assert!(result.is_err());

        let result = generate_proof();
        assert!(result.is_err());

        let result = verify_proof();
        assert!(result.is_err());
    }
}
