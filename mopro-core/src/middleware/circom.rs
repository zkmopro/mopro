use crate::MoproError;

use ark_bn254::Bn254;
use ark_circom::{CircomBuilder, CircomCircuit, CircomConfig};
use ark_crypto_primitives::snark::SNARK;
use ark_ec::pairing::Pairing;
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::{rngs::ThreadRng, thread_rng};
use color_eyre::Result;
use std::path::Path;

type GrothBn = Groth16<Bn254>;

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone)]
pub struct SerializableProvingKey(pub ProvingKey<Bn254>);

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone)]
pub struct SerializableProof(pub Proof<Bn254>);

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone)]
pub struct SerializableInputs(pub Vec<<Bn254 as Pairing>::ScalarField>);

pub fn serialize_proof(proof: &SerializableProof) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    proof
        .serialize_uncompressed(&mut serialized_data)
        .expect("Serialization failed");
    serialized_data
}

pub fn deserialize_proof(data: Vec<u8>) -> SerializableProof {
    SerializableProof::deserialize_uncompressed(&mut &data[..]).expect("Deserialization failed")
}

pub fn serialize_proving_key(pk: &SerializableProvingKey) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    pk.serialize_uncompressed(&mut serialized_data)
        .expect("Serialization failed");
    serialized_data
}

pub fn deserialize_proving_key(data: Vec<u8>) -> SerializableProvingKey {
    SerializableProvingKey::deserialize_uncompressed(&mut &data[..])
        .expect("Deserialization failed")
}

pub struct CircomState {
    circuit: Option<CircomCircuit<Bn254>>,
    params: Option<ProvingKey<Bn254>>,
    // Add other state data members here as required
}

// XXX: Temporary function to test ark-serialize
pub fn generate_serializable_proving_key(
    wasm_path: &str,
    r1cs_path: &str,
) -> Result<SerializableProvingKey, MoproError> {
    assert_paths_exists(wasm_path, r1cs_path)?;

    let cfg = CircomConfig::<Bn254>::new(wasm_path, r1cs_path)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 5);
    let circom = builder.setup();

    let mut rng = thread_rng();
    let raw_params = GrothBn::generate_random_parameters_with_reduction(circom, &mut rng)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    Ok(SerializableProvingKey(raw_params))
}

impl CircomState {
    pub fn new() -> Self {
        Self {
            circuit: None,
            params: None,
        }
    }

    // TODO: Improve how we add inputs
    pub fn setup(
        &mut self,
        wasm_path: &str,
        r1cs_path: &str,
    ) -> Result<(SerializableProvingKey, SerializableInputs), MoproError> {
        assert_paths_exists(wasm_path, r1cs_path)?;

        println!("Setup");

        // Load the WASM and R1CS for witness and proof generation
        let cfg = CircomConfig::<Bn254>::new(wasm_path, r1cs_path)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

        // Insert our inputs as key value pairs
        let mut builder = CircomBuilder::new(cfg);
        builder.push_input("a", 3);
        builder.push_input("b", 5);

        // Create an empty instance for setting it up
        let circom = builder.setup();

        // Run a trusted setup using the rng in the state
        let mut rng = thread_rng();

        let params = GrothBn::generate_random_parameters_with_reduction(circom, &mut rng)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

        self.params = Some(params.clone());

        // Get the populated instance of the circuit with the witness
        let circom = builder
            .build()
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

        self.circuit = Some(circom.clone());

        // This is the instance, public input and public output
        // Together with the witness provided above this satisfies the circuit
        let inputs = circom.get_public_inputs().ok_or(MoproError::CircomError(
            "Failed to get public inputs".to_string(),
        ))?;

        Ok((SerializableProvingKey(params), SerializableInputs(inputs)))
    }

    pub fn generate_proof(&mut self, rng: &mut ThreadRng) -> Result<SerializableProof, MoproError> {
        println!("Generating proof");

        let circuit = self.circuit.as_ref().ok_or(MoproError::CircomError(
            "Circuit has not been set up".to_string(),
        ))?;

        let params = self.params.as_ref().ok_or(MoproError::CircomError(
            "Parameters have not been set up".to_string(),
        ))?;

        let proof = GrothBn::prove(params, circuit.clone(), rng)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

        Ok(SerializableProof(proof))
    }

    pub fn verify_proof(
        &self,
        serialized_proof: SerializableProof,
        serialized_inputs: SerializableInputs,
    ) -> Result<bool, MoproError> {
        println!("Verifying proof");

        let params = self.params.as_ref().ok_or(MoproError::CircomError(
            "Parameters have not been set up".to_string(),
        ))?;

        let pvk =
            GrothBn::process_vk(&params.vk).map_err(|e| MoproError::CircomError(e.to_string()))?;

        let proof_verified =
            GrothBn::verify_with_processed_vk(&pvk, &serialized_inputs.0, &serialized_proof.0)
                .map_err(|e| MoproError::CircomError(e.to_string()))?;

        Ok(proof_verified)
    }
}

// TODO: Remove this once end-to-end with CircomState is working
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
    fn test_setup_failure() {
        // Providing incorrect paths
        let wasm_path = "./nonexistent_path/multiplier2.wasm";
        let r1cs_path = "./nonexistent_path/multiplier2.r1cs";

        let mut circom_state = CircomState::new();
        let setup_res = circom_state.setup(wasm_path, r1cs_path);

        // The setup should fail and return an error
        assert!(setup_res.is_err());
        if let Err(e) = setup_res {
            assert_eq!(
                format!("{}", e),
                "CircomError: Path does not exist: ./nonexistent_path/multiplier2.wasm"
            );
        }
    }

    #[test]
    fn test_setup_prove_verify() {
        let wasm_path = "./examples/circom/target/multiplier2_js/multiplier2.wasm";
        let r1cs_path = "./examples/circom/target/multiplier2.r1cs";

        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.setup(wasm_path, r1cs_path);
        assert!(setup_res.is_ok());

        let (_serialized_pk, serialized_inputs) = setup_res.unwrap();

        // Deserialize the proving key and inputs if necessary

        // Proof generation
        let mut rng = thread_rng();
        let proof_res = circom_state.generate_proof(&mut rng);

        // Check and print the error if there is one
        if let Err(e) = &proof_res {
            eprintln!("Error: {:?}", e);
        }

        assert!(proof_res.is_ok());

        let serialized_proof = proof_res.unwrap();

        // Proof verification
        let verify_res = circom_state.verify_proof(serialized_proof, serialized_inputs);
        assert!(verify_res.is_ok());
        assert!(verify_res.unwrap()); // Verifying that the proof was indeed verified
    }

    // #[test]
    // fn test_setup_prove_verify_err() {
    //     // Setup
    //     let setup_res = setup("foo", "bar");
    //     assert!(setup_res.is_err());
    // }

    #[test]
    fn test_setup() {
        // Arrange: Create a new CircomState instance
        let mut circom_state = CircomState::new();

        let wasm_path = "./examples/circom/target/multiplier2_js/multiplier2.wasm";
        let r1cs_path = "./examples/circom/target/multiplier2.r1cs";

        // Act: Call the setup method
        let result = circom_state.setup(wasm_path, r1cs_path);

        // Assert: Check that the method returned an Ok
        assert!(
            result.is_ok(),
            "Setup failed with error: {:?}",
            result.err().unwrap()
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let wasm_path = "./examples/circom/target/multiplier2_js/multiplier2.wasm";
        let r1cs_path = "./examples/circom/target/multiplier2.r1cs";

        // Generate a serializable proving key for testing
        let serializable_pk = generate_serializable_proving_key(wasm_path, r1cs_path)
            .expect("Failed to generate serializable proving key");

        // Serialize
        let serialized_data = serialize_proving_key(&serializable_pk);

        // Deserialize
        let deserialized_pk = deserialize_proving_key(serialized_data);

        // Assert that the original and deserialized ProvingKeys are the same
        assert_eq!(
            serializable_pk.0, deserialized_pk.0,
            "Original and deserialized proving keys do not match"
        );
    }
}
