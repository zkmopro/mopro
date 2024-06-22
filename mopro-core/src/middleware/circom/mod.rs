use self::{
    serialization::{SerializableInputs, SerializableProof},
    utils::bytes_to_bits,
};
use crate::MoproError;

use std::time::Instant;
use std::{collections::HashMap, fs::File, io::BufReader};

use ark_bn254::{Bn254, Fr};
use ark_circom::{read_zkey, CircomReduction};

use ark_crypto_primitives::snark::SNARK;
use ark_groth16::{prepare_verifying_key, Groth16, ProvingKey};
use ark_std::UniformRand;

use ark_relations::r1cs::ConstraintMatrices;
use ark_std::rand::thread_rng;
use color_eyre::Result;

use num_bigint::BigInt;

use ark_zkey::{read_arkzkey, read_arkzkey_from_bytes}; //SerializableConstraintMatrices

pub mod serialization;
pub mod utils;

pub type WtnsFn = fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>;
type GrothBn = Groth16<Bn254>;

type CircuitInputs = HashMap<String, Vec<BigInt>>;

// TODO: Split up this namespace a bit, right now quite a lot of things going on

pub struct CircomState {
    zkey: Option<(ProvingKey<Bn254>, ConstraintMatrices<Fr>)>,
    wtns: Option<fn(HashMap<String, Vec<BigInt>>) -> Vec<BigInt>>,
}

impl Default for CircomState {
    fn default() -> Self {
        Self::new()
    }
}

// NOTE: A lot of the contents of this file is inspired by github.com/worldcoin/semaphore-rs

// TODO: Replace printlns with logging

impl CircomState {
    pub fn new() -> Self {
        Self {
            zkey: None,
            // arkzkey: None,
            wtns: None,
        }
    }

    pub fn initialize(&mut self, zkey_path: &str, witness_func: WtnsFn) -> Result<(), MoproError> {
        let mut file = File::open(zkey_path).map_err(|e| MoproError::CircomError(e.to_string()))?;
        let zkey = read_zkey(&mut file).map_err(|e| MoproError::CircomError(e.to_string()))?;

        // read_arkzkey(arkzkey_path).map_err(|e| MoproError::CircomError(e.to_string()))?;
        self.zkey = Some(zkey);

        self.wtns = Some(witness_func);

        Ok(())
    }

    pub fn generate_proof(
        &mut self,
        inputs: CircuitInputs,
    ) -> Result<(SerializableProof, SerializableInputs), MoproError> {
        let mut rng = thread_rng();
        let rng = &mut rng;

        let r = ark_bn254::Fr::rand(rng);
        let s = ark_bn254::Fr::rand(rng);

        println!("Generating proof");

        let now = std::time::Instant::now();
        let full_assignment = (self.wtns.unwrap())(inputs)
            .into_iter()
            .map(|w| ark_bn254::Fr::from(w.to_biguint().unwrap()))
            .collect::<Vec<_>>();

        println!("Witness generation took: {:.2?}", now.elapsed());

        let now = std::time::Instant::now();
        let zkey = self.zkey.as_ref().ok_or(MoproError::CircomError(
            "Zkey has not been set up".to_string(),
        ))?;
        println!("Loading zkey took: {:.2?}", now.elapsed());

        let public_inputs = full_assignment.as_slice()[1..zkey.1.num_instance_variables].to_vec();

        let now = std::time::Instant::now();
        let ark_proof = Groth16::<_, CircomReduction>::create_proof_with_reduction_and_matrices(
            &zkey.0,
            r,
            s,
            &zkey.1,
            zkey.1.num_instance_variables,
            zkey.1.num_constraints,
            full_assignment.as_slice(),
        );

        let proof = ark_proof.map_err(|e| MoproError::CircomError(e.to_string()))?;

        println!("proof generation took: {:.2?}", now.elapsed());
        Ok((SerializableProof(proof), SerializableInputs(public_inputs)))
    }

    pub fn verify_proof(
        &self,
        serialized_proof: SerializableProof,
        serialized_inputs: SerializableInputs,
    ) -> Result<bool, MoproError> {
        let start = Instant::now();
        let zkey = self.zkey.as_ref().ok_or(MoproError::CircomError(
            "Zkey has not been set up".to_string(),
        ))?;
        let pvk = prepare_verifying_key(&zkey.0.vk);

        let proof_verified =
            GrothBn::verify_with_processed_vk(&pvk, &serialized_inputs.0, &serialized_proof.0)
                .map_err(|e| MoproError::CircomError(e.to_string()))?;

        let verification_duration = start.elapsed();
        println!("Verification time: {:?}", verification_duration);
        Ok(proof_verified)
    }
}

// Helper function for Keccak256 example
pub fn bytes_to_circuit_inputs(bytes: &[u8]) -> CircuitInputs {
    let bits = bytes_to_bits(bytes);
    let big_int_bits = bits
        .into_iter()
        .map(|bit| BigInt::from(bit as u8))
        .collect();
    let mut inputs = HashMap::new();
    inputs.insert("in".to_string(), big_int_bits);
    inputs
}

pub fn strings_to_circuit_inputs(strings: Vec<String>) -> Vec<BigInt> {
    strings
        .into_iter()
        .map(|value| BigInt::parse_bytes(value.as_bytes(), 10).unwrap())
        .collect()
}

pub fn bytes_to_circuit_outputs(bytes: &[u8]) -> SerializableInputs {
    let bits = bytes_to_bits(bytes);
    let field_bits = bits.into_iter().map(|bit| Fr::from(bit as u8)).collect();
    SerializableInputs(field_bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    rust_witness::witness!(multiplier2);
    rust_witness::witness!(keccak256256test);
    rust_witness::witness!(mainrsa);
    rust_witness::witness!(aadhaarverifier);

    #[test]
    fn test_setup_prove_verify_simple() {
        let zkey_path = "./examples/circom/multiplier2/target/multiplier2_final.zkey";
        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, multiplier2_witness);
        assert!(setup_res.is_ok());

        let _serialized_pk = setup_res.unwrap();

        // Prepare inputs
        let mut inputs = HashMap::new();
        let a = 3;
        let b = 5;
        let c = a * b;
        inputs.insert("a".to_string(), vec![BigInt::from(a)]);
        inputs.insert("b".to_string(), vec![BigInt::from(b)]);
        // output = [public output c, public input a]
        let expected_output = vec![Fr::from(c), Fr::from(a)];
        let serialized_outputs = SerializableInputs(expected_output);

        // Proof generation
        let generate_proof_res = circom_state.generate_proof(inputs);

        // Check and print the error if there is one
        if let Err(e) = &generate_proof_res {
            println!("Error: {:?}", e);
        }

        assert!(generate_proof_res.is_ok());

        let (serialized_proof, serialized_inputs) = generate_proof_res.unwrap();

        // Check output
        assert_eq!(serialized_inputs, serialized_outputs);

        // Proof verification
        let verify_res = circom_state.verify_proof(serialized_proof, serialized_inputs);
        assert!(verify_res.is_ok());
        assert!(verify_res.unwrap()); // Verifying that the proof was indeed verified
    }

    #[test]
    fn test_setup_prove_verify_keccak() {
        let zkey_path = "./examples/circom/keccak256/target/keccak256_256_test_final.zkey";
        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, keccak256256test_witness);
        assert!(setup_res.is_ok());

        let _serialized_pk = setup_res.unwrap();

        // Prepare inputs
        let input_vec = vec![
            116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];

        // Expected output
        let expected_output_vec = vec![
            37, 17, 98, 135, 161, 178, 88, 97, 125, 150, 143, 65, 228, 211, 170, 133, 153, 9, 88,
            212, 4, 212, 175, 238, 249, 210, 214, 116, 170, 85, 45, 21,
        ];

        let inputs = bytes_to_circuit_inputs(&input_vec);
        let serialized_outputs = bytes_to_circuit_outputs(&expected_output_vec);

        // Proof generation
        let generate_proof_res = circom_state.generate_proof(inputs);

        // Check and print the error if there is one
        if let Err(e) = &generate_proof_res {
            println!("Error: {:?}", e);
        }

        assert!(generate_proof_res.is_ok());

        let (serialized_proof, serialized_inputs) = generate_proof_res.unwrap();

        // Check output
        assert_eq!(serialized_inputs, serialized_outputs);

        // Proof verification
        let verify_res = circom_state.verify_proof(serialized_proof, serialized_inputs);
        assert!(verify_res.is_ok());

        assert!(verify_res.unwrap()); // Verifying that the proof was indeed verified
    }

    #[ignore = "ignore for ci"]
    #[test]
    fn test_setup_prove_rsa() {
        let zkey_path = "./examples/circom/rsa/target/main_final.zkey";

        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, mainrsa_witness);
        assert!(setup_res.is_ok());

        let _serialized_pk = setup_res.unwrap();

        // Prepare inputs
        #[derive(serde::Deserialize)]
        struct InputData {
            signature: Vec<String>,
            modulus: Vec<String>,
            base_message: Vec<String>,
        }

        let file_data = std::fs::read_to_string("./examples/circom/rsa/input.json")
            .expect("Unable to read file");
        let data: InputData =
            serde_json::from_str(&file_data).expect("JSON was not well-formatted");

        let mut inputs: HashMap<String, Vec<BigInt>> = HashMap::new();
        inputs.insert(
            "signature".to_string(),
            strings_to_circuit_inputs(data.signature),
        );
        inputs.insert(
            "modulus".to_string(),
            strings_to_circuit_inputs(data.modulus),
        );
        inputs.insert(
            "base_message".to_string(),
            strings_to_circuit_inputs(data.base_message),
        );

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

    #[ignore = "ignore for ci"]
    #[test]
    fn test_setup_prove_anon_aadhaar() {
        let zkey_path = "./examples/circom/anonAadhaar/target/aadhaar-verifier_final.zkey";

        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, aadhaarverifier_witness);
        assert!(setup_res.is_ok());

        let _serialized_pk = setup_res.unwrap();

        // Prepare inputs
        #[derive(serde::Deserialize)]
        struct InputData {
            qr_data_padded: Vec<String>,
            delimiter_indices: Vec<String>,
            signature: Vec<String>,
            pub_key: Vec<String>,
        }

        let file_data = std::fs::read_to_string("./examples/circom/anonAadhaar/input.json")
            .expect("Unable to read file");
        let data: InputData =
            serde_json::from_str(&file_data).expect("JSON was not well-formatted");

        let mut inputs: CircuitInputs = HashMap::new();
        inputs.insert(
            "qrDataPadded".to_string(),
            strings_to_circuit_inputs(data.qr_data_padded),
        );
        inputs.insert("qrDataPaddedLength".to_string(), vec![BigInt::from(1152)]);
        inputs.insert("nonPaddedDataLength".to_string(), vec![BigInt::from(1137)]);
        inputs.insert(
            "delimiterIndices".to_string(),
            strings_to_circuit_inputs(data.delimiter_indices),
        );
        inputs.insert(
            "signature".to_string(),
            strings_to_circuit_inputs(data.signature),
        );
        inputs.insert(
            "pubKey".to_string(),
            strings_to_circuit_inputs(data.pub_key),
        );
        inputs.insert("nullifierSeed".to_string(), vec![BigInt::from(12345678)]);
        inputs.insert("signalHash".to_string(), vec![BigInt::from(1)]);
        inputs.insert("revealGender".to_string(), vec![BigInt::from(0)]);
        inputs.insert("revealAgeAbove18".to_string(), vec![BigInt::from(0)]);
        inputs.insert("revealState".to_string(), vec![BigInt::from(0)]);
        inputs.insert("revealPinCode".to_string(), vec![BigInt::from(0)]);
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
}
