#![allow(unused_variables)]

use std::collections::HashMap;
#[cfg(feature = "dylib")]
use std::path::Path;

#[cfg(not(feature = "halo2"))]
use std::str::FromStr;
#[cfg(not(feature = "halo2"))]
use std::sync::RwLock;

#[cfg(not(feature = "halo2"))]
use num_bigint::BigInt;

#[cfg(not(feature = "halo2"))]
use mopro_core::middleware::circom;
#[cfg(feature = "gpu-benchmarks")]
use mopro_core::middleware::gpu_explorations::{self, utils::benchmark::BenchmarkResult};
use mopro_core::MoproError;

use crate::GenerateProofResult;

#[derive(Debug, Clone, Default)]
pub struct G1 {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Default)]
pub struct G2 {
    pub x: Vec<String>,
    pub y: Vec<String>,
}


#[derive(Debug, Clone, Default)]
pub struct ProofCalldata {
    pub a: G1,
    pub b: G2,
    pub c: G1,
}


// NOTE: Need to hardcode the types here, otherwise UniFFI will complain if the gpu-benchmarks feature is not enabled
#[derive(Debug, Clone)]
#[cfg(not(feature = "gpu-benchmarks"))]
pub struct BenchmarkResult {
    pub instance_size: u32,
    pub num_instance: u32,
    pub avg_processing_time: f64,
}

pub struct MoproCircom {
    #[cfg(not(feature = "halo2"))]
    state: RwLock<circom::CircomState>,
}
impl Default for MoproCircom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(not(feature = "dylib"), not(feature = "halo2")))]
pub fn initialize_mopro() -> Result<(), MoproError> {
    // TODO: Error handle / panic?
    mopro_core::middleware::circom::initialize();
    Ok(())
}

#[cfg(feature = "halo2")]
pub fn initialize_mopro() -> Result<(), MoproError> {
    panic!("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.") // TODO - replace with an error
}

#[cfg(feature = "dylib")]
pub fn initialize_mopro() -> Result<(), MoproError> {
    println!("need to use dylib to init!");
    panic!("need to use dylib to init!");
}

#[cfg(feature = "dylib")]
pub fn initialize_mopro_dylib(dylib_path: String) -> Result<(), MoproError> {
    // TODO: Error handle / panic?
    let dylib_path = Path::new(dylib_path.as_str());
    crate::circom::initialize(dylib_path);
    Ok(())
}

#[cfg(not(feature = "dylib"))]
pub fn initialize_mopro_dylib(_dylib_path: String) -> Result<(), MoproError> {
    println!("dylib feature not enabled!");
    panic!("dylib feature not enabled!");
}

#[cfg(not(feature = "halo2"))]
pub fn generate_proof2(
    inputs: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    // Convert inputs to BigInt
    let bigint_inputs = inputs
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|i| BigInt::from_str(&i).unwrap())
                    .collect(),
            )
        })
        .collect();

    let (proof, inputs) = circom::generate_proof2(bigint_inputs)?;

    let serialized_proof = circom::serialization::serialize_proof(&proof);
    let serialized_inputs = circom::serialization::serialize_inputs(&inputs);
    Ok(GenerateProofResult {
        proof: serialized_proof,
        inputs: serialized_inputs,
    })
}

#[cfg(feature = "halo2")]
pub fn generate_proof2(
    inputs: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    Err(MoproError::CircomError("Project is compiled for Halo2 proving system. Use `generate_halo2_proof2` instead.".to_string()))
}

#[cfg(not(feature = "halo2"))]
pub fn verify_proof2(proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, MoproError> {
    let deserialized_proof = circom::serialization::deserialize_proof(proof);
    let deserialized_public_input = circom::serialization::deserialize_inputs(public_input);
    let is_valid = circom::verify_proof2(deserialized_proof, deserialized_public_input)?;
    Ok(is_valid)
}

#[cfg(feature = "halo2")]
pub fn verify_proof2(proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, MoproError> {
    Err(MoproError::CircomError("Project is compiled for Halo2 proving system. Use `verify_halo2_proof2` instead.".to_string()))
}

#[cfg(not(feature = "halo2"))]
// Convert proof to String-tuples as expected by the Solidity Groth16 Verifier
pub fn to_ethereum_proof(proof: Vec<u8>) -> ProofCalldata {
    let deserialized_proof = circom::serialization::deserialize_proof(proof);
    let proof = circom::serialization::to_ethereum_proof(&deserialized_proof);
    let a = G1 {
        x: proof.a.x.to_string(),
        y: proof.a.y.to_string(),
    };
    let b = G2 {
        x: proof.b.x.iter().map(|x| x.to_string()).collect(),
        y: proof.b.y.iter().map(|x| x.to_string()).collect(),
    };
    let c = G1 {
        x: proof.c.x.to_string(),
        y: proof.c.y.to_string(),
    };
    ProofCalldata { a, b, c }
}

#[cfg(feature = "halo2")]
pub fn to_ethereum_proof(proof: Vec<u8>) -> ProofCalldata {
    panic!("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.") // TODO - replace with an error
}

#[cfg(not(feature = "halo2"))]
pub fn to_ethereum_inputs(inputs: Vec<u8>) -> Vec<String> {
    let deserialized_inputs = circom::serialization::deserialize_inputs(inputs);
    let inputs = deserialized_inputs
        .0
        .iter()
        .map(|x| x.to_string())
        .collect();
    inputs
}

#[cfg(feature = "halo2")]
pub fn to_ethereum_inputs(inputs: Vec<u8>) -> Vec<String> {
    panic!("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.") // TODO - replace with an error
}

// TODO: Use FFIError::SerializationError instead
impl MoproCircom {
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "halo2"))]
            state: RwLock::new(circom::CircomState::new()),
        }
    }

    #[cfg(not(feature = "halo2"))]
    pub fn initialize(&self, zkey_path: String, wasm_path: String) -> Result<(), MoproError> {
        let mut state_guard = self.state.write().unwrap();
        state_guard.initialize(zkey_path.as_str(), wasm_path.as_str())?;
        Ok(())
    }

    #[cfg(feature = "halo2")]
    pub fn initialize(&self, zkey_path: String, wasm_path: String) -> Result<(), MoproError> {
        Err(MoproError::CircomError("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.".to_string()))
    }

    //             inputs: circom::serialization::serialize_inputs(&inputs),

    #[cfg(not(feature = "halo2"))]
    pub fn generate_proof(
        &self,
        inputs: HashMap<String, Vec<String>>,
    ) -> Result<GenerateProofResult, MoproError> {
        let mut state_guard = self.state.write().unwrap();

        // Convert inputs to BigInt
        let bigint_inputs = inputs
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter()
                        .map(|i| BigInt::from_str(&i).unwrap())
                        .collect(),
                )
            })
            .collect();

        let (proof, inputs) = state_guard.generate_proof(bigint_inputs)?;

        Ok(GenerateProofResult {
            proof: circom::serialization::serialize_proof(&proof),
            inputs: circom::serialization::serialize_inputs(&inputs),
        })
    }
    
    #[cfg(feature = "halo2")]
    pub fn generate_proof(
        &self,
        inputs: HashMap<String, Vec<String>>,
    ) -> Result<GenerateProofResult, MoproError> {
        Err(MoproError::CircomError("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.".to_string()))
    }

    #[cfg(not(feature = "halo2"))]
    pub fn verify_proof(&self, proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, MoproError> {
        let state_guard = self.state.read().unwrap();
        let deserialized_proof = circom::serialization::deserialize_proof(proof);
        let deserialized_public_input = circom::serialization::deserialize_inputs(public_input);
        let is_valid = state_guard.verify_proof(deserialized_proof, deserialized_public_input)?;
        Ok(is_valid)
    }
    
    #[cfg(feature = "halo2")]
    pub fn verify_proof(&self, proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, MoproError> {
        Err(MoproError::CircomError("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.".to_string()))
    }
}

#[cfg(feature = "gpu-benchmarks")]
pub fn arkworks_pippenger(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
) -> Result<BenchmarkResult, MoproError> {
    let benchmarks = gpu_explorations::arkworks_pippenger::run_benchmark(
        instance_size,
        num_instance,
        &utils_dir,
    )
    .unwrap();
    Ok(benchmarks)
}
#[cfg(feature = "gpu-benchmarks")]
pub fn trapdoortech_zprize_msm(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
) -> Result<BenchmarkResult, MoproError> {
    let benchmarks = gpu_explorations::trapdoortech_zprize_msm::run_benchmark(
        instance_size,
        num_instance,
        &utils_dir,
    )
    .unwrap();
    Ok(benchmarks)
}

#[cfg(not(feature = "gpu-benchmarks"))]
pub fn arkworks_pippenger(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
) -> Result<BenchmarkResult, MoproError> {
    println!("gpu-benchmarks feature not enabled!");
    Ok(BenchmarkResult {
        instance_size,
        num_instance,
        avg_processing_time: 0.0,
    })
}

#[cfg(not(feature = "gpu-benchmarks"))]
pub fn trapdoortech_zprize_msm(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
) -> Result<BenchmarkResult, MoproError> {
    println!("gpu-benchmarks feature not enabled!");
    Ok(BenchmarkResult {
        instance_size,
        num_instance,
        avg_processing_time: 0.0,
    })
}

#[test]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use ark_bn254::Fr;
    use num_bigint::BigUint;

    use mopro_core::middleware::circom;
    use mopro_core::MoproError;

    use crate::circom::{MoproCircom, to_ethereum_inputs, to_ethereum_proof};

    #[cfg(not(feature = "halo2"))]
    fn bytes_to_circuit_inputs(input_vec: &Vec<u8>) -> HashMap<String, Vec<String>> {
        let bits = circom::utils::bytes_to_bits(&input_vec);
        let converted_vec: Vec<String> = bits
            .into_iter()
            .map(|bit| (bit as i32).to_string())
            .collect();
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), converted_vec);
        inputs
    }

    #[cfg(not(feature = "halo2"))]
    fn bytes_to_circuit_outputs(bytes: &[u8]) -> Vec<u8> {
        let bits = circom::utils::bytes_to_bits(bytes);
        let field_bits = bits.into_iter().map(|bit| Fr::from(bit as u8)).collect();
        let circom_outputs = circom::serialization::SerializableInputs(field_bits);
        circom::serialization::serialize_inputs(&circom_outputs)
    }



    #[test]
    fn test_end_to_end() -> Result<(), MoproError> {
        // Paths to your wasm and zkey files
        let wasm_path =
            "./../mopro-core/examples/circom/multiplier2/target/multiplier2_js/multiplier2.wasm";
        let zkey_path = "./../mopro-core/examples/circom/multiplier2/target/multiplier2_final.zkey";

        // Create a new MoproCircom instance
        let mopro_circom = MoproCircom::new();

        // Step 1: Initialize
        let init_result = mopro_circom.initialize(zkey_path.to_string(), wasm_path.to_string());
        assert!(init_result.is_ok());

        let mut inputs = HashMap::new();
        let a = BigUint::from_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495616",
        )
        .unwrap();
        let b = BigUint::from(1u8);
        let c = a.clone() * b.clone();
        inputs.insert("a".to_string(), vec![a.to_string()]);
        inputs.insert("b".to_string(), vec![b.to_string()]);
        // output = [public output c, public input a]
        let expected_output = vec![Fr::from(c), Fr::from(a)];
        let circom_outputs = circom::serialization::SerializableInputs(expected_output);
        let serialized_outputs = circom::serialization::serialize_inputs(&circom_outputs);

        // Step 2: Generate Proof
        let generate_proof_result = mopro_circom.generate_proof(inputs)?;
        let serialized_proof = generate_proof_result.proof;
        let serialized_inputs = generate_proof_result.inputs;

        assert!(serialized_proof.len() > 0);
        assert_eq!(serialized_inputs, serialized_outputs);

        // Step 3: Verify Proof
        let is_valid =
            mopro_circom.verify_proof(serialized_proof.clone(), serialized_inputs.clone())?;
        assert!(is_valid);

        // Step 4: Convert Proof to Ethereum compatible proof
        let proof_calldata = to_ethereum_proof(serialized_proof);
        let inputs_calldata = to_ethereum_inputs(serialized_inputs);
        assert!(proof_calldata.a.x.len() > 0);
        assert!(inputs_calldata.len() > 0);

        Ok(())
    }

    #[test]
    fn test_end_to_end_keccak() -> Result<(), MoproError> {
        // Paths to your wasm and r1cs files
        let wasm_path =
            "./../mopro-core/examples/circom/keccak256/target/keccak256_256_test_js/keccak256_256_test.wasm";
        let zkey_path =
            "./../mopro-core/examples/circom/keccak256/target/keccak256_256_test_final.zkey";

        // Create a new MoproCircom instance
        let mopro_circom = MoproCircom::new();

        // Step 1: Setup
        let setup_result = mopro_circom.initialize(zkey_path.to_string(), wasm_path.to_string());
        assert!(setup_result.is_ok());

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

        // Step 2: Generate Proof
        let generate_proof_result = mopro_circom.generate_proof(inputs)?;
        let serialized_proof = generate_proof_result.proof;
        let serialized_inputs = generate_proof_result.inputs;

        assert!(serialized_proof.len() > 0);
        assert_eq!(serialized_inputs, serialized_outputs);

        // Step 3: Verify Proof

        let is_valid =
            mopro_circom.verify_proof(serialized_proof.clone(), serialized_inputs.clone())?;
        assert!(is_valid);

        // Step 4: Convert Proof to Ethereum compatible proof
        let proof_calldata = to_ethereum_proof(serialized_proof);
        let inputs_calldata = to_ethereum_inputs(serialized_inputs);
        assert!(proof_calldata.a.x.len() > 0);
        assert!(inputs_calldata.len() > 0);

        Ok(())
    }

    #[test]
    #[cfg(feature = "gpu-benchmarks")]
    fn test_arkworks_pippenger() -> Result<(), MoproError> {
        let instance_size = 16;
        let num_instance = 10;
        let utils_dir = "../mopro-core/src/middleware/gpu_explorations/utils/vectors/16x10";
        let result = arkworks_pippenger(instance_size, num_instance, &utils_dir).unwrap();
        println!("Benchmark result: {:#?}", result);
        Ok(())
    }

    #[test]
    #[cfg(feature = "gpu-benchmarks")]
    fn test_trapdoortech_zprize_msm() -> Result<(), MoproError> {
        let instance_size = 16;
        let num_instance = 10;
        let utils_dir = "../mopro-core/src/middleware/gpu_explorations/utils/vectors/16x10";
        let result = trapdoortech_zprize_msm(instance_size, num_instance, utils_dir);
        println!("Benchmark result: {:#?}", result);
        Ok(())
    }
}