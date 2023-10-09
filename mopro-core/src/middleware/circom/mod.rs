use self::{
    serialization::{SerializableInputs, SerializableProof, SerializableProvingKey},
    utils::assert_paths_exists,
};
use crate::MoproError;

use std::collections::HashMap;

use ark_bn254::Bn254;
use ark_circom::{CircomBuilder, CircomCircuit, CircomConfig};
use ark_crypto_primitives::snark::SNARK;
use ark_groth16::{Groth16, ProvingKey};
use ark_std::rand::thread_rng;
use color_eyre::Result;
use num_bigint::BigInt;

pub mod serialization;
pub mod utils;

type GrothBn = Groth16<Bn254>;

type CircuitInputs = HashMap<String, Vec<BigInt>>;

pub struct CircomState {
    builder: Option<CircomBuilder<Bn254>>,
    circuit: Option<CircomCircuit<Bn254>>,
    params: Option<ProvingKey<Bn254>>,
}

impl Default for CircomState {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Replace printlns with logging

impl CircomState {
    pub fn new() -> Self {
        Self {
            builder: None,
            circuit: None,
            params: None,
        }
    }

    pub fn setup(
        &mut self,
        wasm_path: &str,
        r1cs_path: &str,
    ) -> Result<SerializableProvingKey, MoproError> {
        assert_paths_exists(wasm_path, r1cs_path)?;
        println!("Setup");

        // Load the WASM and R1CS for witness and proof generation
        let cfg = self.load_config(wasm_path, r1cs_path)?;

        // Create an empty instance for setup
        self.builder = Some(CircomBuilder::new(cfg));

        // Run a trusted setup using the rng in the state
        let params = self.run_trusted_setup()?;

        self.params = Some(params.clone());

        Ok(SerializableProvingKey(params))
    }

    // NOTE: Consider generate_proof<T: Into<BigInt>> API
    // XXX: BigInt might present problems for UniFFI
    pub fn generate_proof(
        &mut self,
        inputs: CircuitInputs,
    ) -> Result<(SerializableProof, SerializableInputs), MoproError> {
        println!("Generating proof");

        let mut rng = thread_rng();

        let builder = self.builder.as_mut().ok_or(MoproError::CircomError(
            "Builder has not been set up".to_string(),
        ))?;

        // Insert our inputs as key value pairs
        for (key, values) in &inputs {
            for value in values {
                builder.push_input(&key, value.clone());
            }
        }

        // Clone the builder, then build the circuit
        let circom = builder
            .clone()
            .build()
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

        // Update the circuit in self
        self.circuit = Some(circom.clone());

        let params = self.params.as_ref().ok_or(MoproError::CircomError(
            "Parameters have not been set up".to_string(),
        ))?;

        let inputs = circom.get_public_inputs().ok_or(MoproError::CircomError(
            "Failed to get public inputs".to_string(),
        ))?;

        let proof = GrothBn::prove(params, circom.clone(), &mut rng)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

        Ok((SerializableProof(proof), SerializableInputs(inputs)))
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

    fn load_config(
        &self,
        wasm_path: &str,
        r1cs_path: &str,
    ) -> Result<CircomConfig<Bn254>, MoproError> {
        CircomConfig::<Bn254>::new(wasm_path, r1cs_path)
            .map_err(|e| MoproError::CircomError(e.to_string()))
    }

    fn run_trusted_setup(&mut self) -> Result<ProvingKey<Bn254>, MoproError> {
        let circom_setup = self
            .builder
            .as_mut()
            .ok_or(MoproError::CircomError(
                "Builder has not been set up".to_string(),
            ))?
            .setup();

        let mut rng = thread_rng();

        GrothBn::generate_random_parameters_with_reduction(circom_setup, &mut rng)
            .map_err(|e| MoproError::CircomError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_prove_verify() {
        let wasm_path = "./examples/circom/target/multiplier2_js/multiplier2.wasm";
        let r1cs_path = "./examples/circom/target/multiplier2.r1cs";

        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.setup(wasm_path, r1cs_path);
        assert!(setup_res.is_ok());

        let _serialized_pk = setup_res.unwrap();

        // Deserialize the proving key and inputs if necessary

        // Prepare inputs
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), vec![BigInt::from(3)]);
        inputs.insert("b".to_string(), vec![BigInt::from(5)]);

        // Proof generation
        let generate_proof_res = circom_state.generate_proof(inputs);

        // Check and print the error if there is one
        if let Err(e) = &generate_proof_res {
            println!("Error: {:?}", e);
        }

        assert!(generate_proof_res.is_ok());

        let (serialized_proof, serialized_inputs) = generate_proof_res.unwrap();

        // Proof verification
        let verify_res = circom_state.verify_proof(serialized_proof, serialized_inputs);
        assert!(verify_res.is_ok());
        assert!(verify_res.unwrap()); // Verifying that the proof was indeed verified
    }

    #[test]
    fn test_setup_error() {
        // Arrange: Create a new CircomState instance
        let mut circom_state = CircomState::new();

        let wasm_path = "badpath/multiplier2.wasm";
        let r1cs_path = "badpath/multiplier2.r1cs";

        // Act: Call the setup method
        let result = circom_state.setup(wasm_path, r1cs_path);

        // Assert: Check that the method returns an error
        assert!(result.is_err());
    }
}
