use self::{
    serialization::{SerializableInputs, SerializableProof},
    utils::bytes_to_bits,
};
use crate::MoproError;

use std::io::Cursor;

use std::time::Instant;
use std::{collections::HashMap, fs::File};

use ark_bn254::{Bn254, Fr};
use ark_circom::{
    read_zkey,
    CircomReduction,
    WitnessCalculator, //read_zkey,
};
use ark_crypto_primitives::snark::SNARK;
use ark_groth16::{prepare_verifying_key, Groth16, ProvingKey};
use ark_std::UniformRand;

use ark_relations::r1cs::ConstraintMatrices;
use ark_std::rand::thread_rng;
use color_eyre::Result;
use core::include_bytes;
use num_bigint::BigInt;
use once_cell::sync::Lazy;

use wasmer::{Module, Store};

#[cfg(feature = "calc-native-witness")]
use {
    ark_std::str::FromStr,
    ruint::aliases::U256,
    witness::{init_graph, Graph},
};

pub mod serialization;
pub mod utils;

type GrothBn = Groth16<Bn254>;

type CircuitInputs = HashMap<String, Vec<BigInt>>;

// TODO: Split up this namespace a bit, right now quite a lot of things going on

pub struct CircomState {
    zkey: Option<(ProvingKey<Bn254>, ConstraintMatrices<Fr>)>,
    wtns: Option<WitnessCalculator>,
    store: Store,
}

impl Default for CircomState {
    fn default() -> Self {
        Self::new()
    }
}

// NOTE: A lot of the contents of this file is inspired by github.com/worldcoin/semaphore-rs

// TODO: Replace printlns with logging

const ZKEY_BYTES: &[u8] = include_bytes!(env!("BUILD_RS_ZKEY_FILE"));

// const ARKZKEY_BYTES: &[u8] = include_bytes!(env!("BUILD_RS_ARKZKEY_FILE"));

static ZKEY: Lazy<(ProvingKey<Bn254>, ConstraintMatrices<Fr>)> = Lazy::new(|| {
    let mut reader = Cursor::new(ZKEY_BYTES);
    read_zkey(&mut reader).expect("Failed to read zkey")
});

// static ARKZKEY: Lazy<(ProvingKey<Bn254>, ConstraintMatrices<Fr>)> = Lazy::new(|| {
//     //let mut reader = Cursor::new(ARKZKEY_BYTES);
//     // TODO: Use reader? More flexible; unclear if perf diff
//     read_arkzkey_from_bytes(ARKZKEY_BYTES).expect("Failed to read arkzkey")
// });

#[cfg(not(feature = "dylib"))]
const WASM: &[u8] = include_bytes!(env!("BUILD_RS_WASM_FILE"));

/// `WITNESS_CALCULATOR` is a lazily initialized, thread-safe singleton of type `WitnessCalculator`.
/// `OnceCell` ensures that the initialization occurs exactly once, and `Mutex` allows safe shared
/// access from multiple threads.

#[cfg(feature = "calc-native-witness")]
const GRAPH_BYTES: &[u8] = include_bytes!(env!("BUILD_RS_GRAPH_FILE"));
#[cfg(feature = "calc-native-witness")]
static WITNESS_GRAPH: Lazy<Graph> =
    Lazy::new(|| init_graph(&GRAPH_BYTES).expect("Failed to initialize Graph"));
#[cfg(feature = "calc-native-witness")]
fn calculate_witness_with_graph(inputs: CircuitInputs) -> Vec<Fr> {
    let inputs_u256: HashMap<String, Vec<U256>> = inputs
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|x| U256::from_str(&x.to_string()).unwrap())
                    .collect(),
            )
        })
        .collect();

    let witness = witness::calculate_witness(inputs_u256, &WITNESS_GRAPH).unwrap();
    let full_assignment = witness
        .into_iter()
        .map(|x| Fr::from_str(&x.to_string()).unwrap())
        .collect::<Vec<_>>();
    full_assignment
}

/// Initializes the `WITNESS_CALCULATOR` singleton with a `WitnessCalculator` instance created from
/// a specified dylib file (WASM circuit). Also initialize `ZKEY`.
#[cfg(feature = "dylib")]
pub fn initialize(dylib_path: &Path) {
    println!("Initializing dylib: {:?}", dylib_path);

    WITNESS_CALCULATOR
        .set(from_dylib(dylib_path))
        .expect("Failed to set WITNESS_CALCULATOR");

    // Initialize ZKEY
    let now = std::time::Instant::now();
    Lazy::force(&ZKEY);
    // Lazy::force(&ARKZKEY);
    println!("Initializing zkey took: {:.2?}", now.elapsed());
}

#[cfg(not(feature = "dylib"))]
pub fn initialize() {
    println!("Initializing library with zkey");

    // Initialize ZKEY
    let now = std::time::Instant::now();
    Lazy::force(&ZKEY);
    // Lazy::force(&ARKZKEY);
    println!("Initializing zkey took: {:.2?}", now.elapsed());
}

/// Creates a `WitnessCalculator` instance from a dylib file.
#[cfg(feature = "dylib")]
fn from_dylib(path: &Path) -> Mutex<WitnessCalculator> {
    let engine = EngineBuilder::headless();
    let mut store = Store::new(engine);
    let module = unsafe {
        Module::deserialize_from_file(&store, path).expect("Failed to load dylib module")
    };
    let result = WitnessCalculator::from_module(&mut store, module)
        .expect("Failed to create WitnessCalculator");

    Mutex::new(result)
}

#[must_use]
pub fn zkey() -> &'static (ProvingKey<Bn254>, ConstraintMatrices<Fr>) {
    &ZKEY
}

// Experimental
// #[must_use]
// pub fn arkzkey() -> &'static (ProvingKey<Bn254>, ConstraintMatrices<Fr>) {
//     &ARKZKEY
// }

/// Provides access to the `WITNESS_CALCULATOR` singleton, initializing it if necessary.
/// It expects the path to the dylib file to be set in the `CIRCUIT_WASM_DYLIB` environment variable.
#[cfg(feature = "dylib")]
#[must_use]
pub fn witness_calculator() -> &'static Mutex<WitnessCalculator> {
    let var_name = "CIRCUIT_WASM_DYLIB";

    WITNESS_CALCULATOR.get_or_init(|| {
        let path = env::var(var_name).unwrap_or_else(|_| {
            panic!(
                "Mopro circuit WASM Dylib not initialized. \
            Please set {} environment variable to the path of the dylib file",
                var_name
            )
        });
        from_dylib(Path::new(&path))
    })
}

#[cfg(not(feature = "dylib"))]
#[must_use]
pub fn witness_calculator() -> (WitnessCalculator, Store) {
    let mut store = Store::default();
    let module = Module::from_binary(&store, WASM).expect("WASM should be valid");
    (
        WitnessCalculator::from_module(&mut store, module)
            .expect("Failed to create WitnessCalculator"),
        store,
    )
}

pub fn generate_proof2(
    inputs: CircuitInputs,
) -> Result<(SerializableProof, SerializableInputs), MoproError> {
    let mut rng = thread_rng();
    let rng = &mut rng;

    let r = ark_bn254::Fr::rand(rng);
    let s = ark_bn254::Fr::rand(rng);

    println!("Generating proof 2");

    let now = std::time::Instant::now();
    let full_assignment;
    #[cfg(not(feature = "calc-native-witness"))]
    {
        // let engine = EngineBuilder::from(Cranelift::new());
        let (mut witness, mut store) = witness_calculator();
        full_assignment = witness
            .calculate_witness_element::<Bn254, _>(&mut store, inputs, false)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;
    }
    #[cfg(feature = "calc-native-witness")]
    let full_assignment = calculate_witness_with_graph(inputs);

    println!("Witness generation took: {:.2?}", now.elapsed());

    let now = std::time::Instant::now();
    let zkey = zkey();
    // let zkey = arkzkey();
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

    // TODO: Add SerializableInputs(inputs)))
    Ok((SerializableProof(proof), SerializableInputs(public_inputs)))
}

pub fn verify_proof2(
    serialized_proof: SerializableProof,
    serialized_inputs: SerializableInputs,
) -> Result<bool, MoproError> {
    let start = Instant::now();
    let zkey = zkey();
    // let zkey = arkzkey();
    let pvk = prepare_verifying_key(&zkey.0.vk);

    let proof_verified =
        GrothBn::verify_with_processed_vk(&pvk, &serialized_inputs.0, &serialized_proof.0)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

    let verification_duration = start.elapsed();
    println!("Verification time 2: {:?}", verification_duration);
    Ok(proof_verified)
}

impl CircomState {
    pub fn new() -> Self {
        Self {
            zkey: None,
            // arkzkey: None,
            wtns: None,
            store: Store::default(),
        }
    }

    pub fn initialize(&mut self, zkey_path: &str, wasm_path: &str) -> Result<(), MoproError> {
        let mut file = File::open(zkey_path).map_err(|e| MoproError::CircomError(e.to_string()))?;
        let zkey = read_zkey(&mut file).map_err(|e| MoproError::CircomError(e.to_string()))?;

        // read_arkzkey(arkzkey_path).map_err(|e| MoproError::CircomError(e.to_string()))?;
        self.zkey = Some(zkey);

        let wtns: WitnessCalculator = WitnessCalculator::new(&mut self.store, wasm_path)
            .map_err(|e| MoproError::CircomError(e.to_string()))
            .unwrap();
        self.wtns = Some(wtns);

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
        let full_assignment = self
            .wtns
            // .clone()
            // I _think_ the above clone is unnecessary because this function does not modify our self.wtns
            .as_mut()
            .unwrap()
            .calculate_witness_element::<Bn254, _>(&mut self.store, inputs, false)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

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

    #[tokio::test]
    async fn test_setup_prove_verify_simple() {
        let wasm_path = "./examples/circom/multiplier2/target/multiplier2_js/multiplier2.wasm";
        let zkey_path = "./examples/circom/multiplier2/target/multiplier2_final.zkey";
        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, wasm_path);
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

    #[tokio::test]
    async fn test_setup_prove_verify_keccak() {
        let wasm_path =
            "./examples/circom/keccak256/target/keccak256_256_test_js/keccak256_256_test.wasm";
        let zkey_path = "./examples/circom/keccak256/target/keccak256_256_test_final.zkey";
        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, wasm_path);
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

    #[test]
    fn test_setup_error() {
        // Arrange: Create a new CircomState instance
        let mut circom_state = CircomState::new();

        let wasm_path = "badpath/multiplier2.wasm";
        let zkey_path = "badpath/multiplier2.zkey";

        // Act: Call the setup method
        let result = circom_state.initialize(zkey_path, wasm_path);

        // Assert: Check that the method returns an error
        assert!(result.is_err());
    }

    #[cfg(feature = "dylib")]
    #[test]
    fn test_dylib_init_and_generate_witness() {
        // Assumes that the dylib file has been built and is in the following location
        let dylib_path = "target/debug/aarch64-apple-darwin/keccak256.dylib";

        // Initialize libray
        initialize(Path::new(&dylib_path));

        let input_vec = vec![
            116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];

        let inputs = bytes_to_circuit_inputs(&input_vec);

        let engine = EngineBuilder::headless();
        let mut store = Store::new(engine);
        let now = std::time::Instant::now();
        let full_assignment = witness_calculator()
            .lock()
            .expect("Failed to lock witness calculator")
            .calculate_witness_element::<Bn254, _>(&mut store, inputs, false)
            .map_err(|e| MoproError::CircomError(e.to_string()));

        println!("Witness generation took: {:.2?}", now.elapsed());

        assert!(full_assignment.is_ok());
    }

    #[tokio::test]
    async fn test_generate_proof2() {
        // XXX: This can be done better
        #[cfg(feature = "dylib")]
        {
            // Assumes that the dylib file has been built and is in the following location
            let dylib_path = "target/debug/aarch64-apple-darwin/keccak256.dylib";

            // Initialize libray
            initialize(Path::new(&dylib_path));
        }

        let input_vec = vec![
            116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        let expected_output_vec = vec![
            37, 17, 98, 135, 161, 178, 88, 97, 125, 150, 143, 65, 228, 211, 170, 133, 153, 9, 88,
            212, 4, 212, 175, 238, 249, 210, 214, 116, 170, 85, 45, 21,
        ];
        let inputs = bytes_to_circuit_inputs(&input_vec);
        let serialized_outputs = bytes_to_circuit_outputs(&expected_output_vec);

        let generate_proof_res = generate_proof2(inputs);
        let (serialized_proof, serialized_inputs) = generate_proof_res.unwrap();
        assert_eq!(serialized_inputs, serialized_outputs);

        // Proof verification
        let verify_res = verify_proof2(serialized_proof, serialized_inputs);
        assert!(verify_res.is_ok());
        assert!(verify_res.unwrap()); // Verifying that the proof was indeed verified
    }

    #[ignore = "ignore for ci"]
    #[test]
    fn test_setup_prove_rsa() {
        let wasm_path = "./examples/circom/rsa/target/main_js/main.wasm";
        let zkey_path = "./examples/circom/rsa/target/main_final.zkey";

        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, wasm_path);
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
    fn test_setup_prove_rsa2() {
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
        let generate_proof_res = generate_proof2(inputs);

        // Check and print the error if there is one
        if let Err(e) = &generate_proof_res {
            println!("Error: {:?}", e);
        }

        assert!(generate_proof_res.is_ok());

        let (serialized_proof, serialized_inputs) = generate_proof_res.unwrap();

        // Proof verification
        let verify_res = verify_proof2(serialized_proof, serialized_inputs);
        assert!(verify_res.is_ok());

        assert!(verify_res.unwrap()); // Verifying that the proof was indeed verified
    }

    #[ignore = "ignore for ci"]
    #[test]
    fn test_setup_prove_anon_aadhaar() {
        let wasm_path =
            "./examples/circom/anonAadhaar/target/aadhaar-verifier_js/aadhaar-verifier.wasm";
        let zkey_path = "./examples/circom/anonAadhaar/target/aadhaar-verifier_final.zkey";

        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, wasm_path);
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

    #[ignore = "ignore for ci"]
    #[test]
    fn test_setup_prove_anon_aadhaar2() {
        // Prepare inputs
        #[derive(serde::Deserialize)]
        struct InputData {
            aadhaar_data: Vec<String>,
            signature: Vec<String>,
            pub_key: Vec<String>,
        }

        let file_data = std::fs::read_to_string("./examples/circom/anonAadhaar/input.json")
            .expect("Unable to read file");
        let data: InputData =
            serde_json::from_str(&file_data).expect("JSON was not well-formatted");

        let mut inputs: CircuitInputs = HashMap::new();
        inputs.insert(
            "aadhaarData".to_string(),
            strings_to_circuit_inputs(data.aadhaar_data),
        );
        inputs.insert("aadhaarDataLength".to_string(), vec![BigInt::from(64)]);
        inputs.insert(
            "signature".to_string(),
            strings_to_circuit_inputs(data.signature),
        );
        inputs.insert(
            "pubKey".to_string(),
            strings_to_circuit_inputs(data.pub_key),
        );
        inputs.insert("signalHash".to_string(), vec![BigInt::from(1)]);

        // Proof generation
        let generate_proof_res = generate_proof2(inputs);

        // Check and print the error if there is one
        if let Err(e) = &generate_proof_res {
            println!("Error: {:?}", e);
        }

        assert!(generate_proof_res.is_ok());

        let (serialized_proof, serialized_inputs) = generate_proof_res.unwrap();

        // Proof verification
        let verify_res = verify_proof2(serialized_proof, serialized_inputs);
        assert!(verify_res.is_ok());

        assert!(verify_res.unwrap()); // Verifying that the proof was indeed verified
    }
}
