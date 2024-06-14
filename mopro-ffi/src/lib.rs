pub use circom::*;
pub use halo2::*;
use mopro_core::MoproError;

mod circom;
mod halo2;

#[derive(Debug)]
pub enum FFIError {
    MoproError(mopro_core::MoproError),
    SerializationError(String),
}

#[derive(Debug, Clone)]
pub struct GenerateProofResult {
    pub proof: Vec<u8>,
    pub inputs: Vec<u8>,
}

impl From<MoproError> for FFIError {
    fn from(error: MoproError) -> Self {
        FFIError::MoproError(error)
    }
}

pub struct MoproCircom {
    state: RwLock<circom::CircomState>,
}

impl Default for MoproCircom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "dylib"))]
pub fn initialize_mopro() -> Result<(), MoproError> {
    // TODO: Error handle / panic?
    circom::initialize();
    Ok(())
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
    circom::initialize(dylib_path);
    Ok(())
}

#[cfg(not(feature = "dylib"))]
pub fn initialize_mopro_dylib(_dylib_path: String) -> Result<(), MoproError> {
    println!("dylib feature not enabled!");
    panic!("dylib feature not enabled!");
}

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

pub fn verify_proof2(proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, MoproError> {
    let deserialized_proof = circom::serialization::deserialize_proof(proof);
    let deserialized_public_input = circom::serialization::deserialize_inputs(public_input);
    let is_valid = circom::verify_proof2(deserialized_proof, deserialized_public_input)?;
    Ok(is_valid)
}

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

pub fn to_ethereum_inputs(inputs: Vec<u8>) -> Vec<String> {
    let deserialized_inputs = circom::serialization::deserialize_inputs(inputs);
    let inputs = deserialized_inputs
        .0
        .iter()
        .map(|x| x.to_string())
        .collect();
    inputs
}

// TODO: Use FFIError::SerializationError instead
impl MoproCircom {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(circom::CircomState::new()),
        }
    }

    pub fn initialize(&self, zkey_path: String, wasm_path: String) -> Result<(), MoproError> {
        let mut state_guard = self.state.write().unwrap();
        state_guard.initialize(zkey_path.as_str(), wasm_path.as_str())?;
        Ok(())
    }

    //             inputs: circom::serialization::serialize_inputs(&inputs),

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

    pub fn verify_proof(&self, proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, MoproError> {
        let state_guard = self.state.read().unwrap();
        let deserialized_proof = circom::serialization::deserialize_proof(proof);
        let deserialized_public_input = circom::serialization::deserialize_inputs(public_input);
        let is_valid = state_guard.verify_proof(deserialized_proof, deserialized_public_input)?;
        Ok(is_valid)
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
// #[cfg(feature = "gpu-benchmarks")]
// pub fn trapdoortech_zprize_msm(
//     instance_size: u32,
//     num_instance: u32,
//     utils_dir: &str,
// ) -> Result<BenchmarkResult, MoproError> {
//     let benchmarks = gpu_explorations::trapdoortech_zprize_msm::run_benchmark(
//         instance_size,
//         num_instance,
//         &utils_dir,
//     )
//     .unwrap();
//     Ok(benchmarks)
// }

#[cfg(feature = "gpu-benchmarks")]
pub fn metal_msm(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
) -> Result<BenchmarkResult, MoproError> {
    let benchmarks =
        gpu_explorations::metal::msm::run_benchmark(instance_size, num_instance, utils_dir)
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

// #[cfg(not(feature = "gpu-benchmarks"))]
// pub fn trapdoortech_zprize_msm(
//     instance_size: u32,
//     num_instance: u32,
//     utils_dir: &str,
// ) -> Result<BenchmarkResult, MoproError> {
//     println!("gpu-benchmarks feature not enabled!");
//     Ok(BenchmarkResult {
//         instance_size,
//         num_instance,
//         avg_processing_time: 0.0,
//     })
// }

#[cfg(not(feature = "gpu-benchmarks"))]
pub fn metal_msm(
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

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn hello() -> String {
    "Hello World from Rust".to_string()
}

// TODO: Remove me
// UniFFI expects String type
// See https://mozilla.github.io/uniffi-rs/udl/builtin_types.html
// fn run_example(wasm_path: String, r1cs_path: String) -> Result<(), MoproError> {
//     circom::run_example(wasm_path.as_str(), r1cs_path.as_str())
// }

uniffi::include_scaffolding!("mopro");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
