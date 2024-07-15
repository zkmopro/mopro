pub mod app_config;
#[cfg(feature = "circom")]
mod circom;
#[cfg(feature = "halo2")]
mod halo2;

#[cfg(feature = "gpu-acceleration")]
use mopro_msm::msm::{self, utils::benchmark};

use std::collections::HashMap;
use thiserror::Error;

pub type WtnsFn = fn(HashMap<String, Vec<num_bigint::BigInt>>) -> Vec<num_bigint::BigInt>;

#[derive(Debug, Error)]
pub enum MoproError {
    #[error("CircomError: {0}")]
    CircomError(String),
    #[error("Halo2Error: {0}")]
    Halo2Error(String),
    #[error("MSMError: {0}")]
    MSMError(String),
}

#[cfg(feature = "gpu-acceleration")]
impl From<benchmark::BenchmarkResult> for BenchmarkResult {
    fn from(msm_result: benchmark::BenchmarkResult) -> Self {
        BenchmarkResult {
            instance_size: msm_result.instance_size,
            num_instance: msm_result.num_instance,
            avg_processing_time: msm_result.avg_processing_time,
        }
    }
}

#[cfg(feature = "halo2")]
pub fn generate_halo2_proof(
    in0: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    halo2::generate_halo2_proof(in0)
}

#[cfg(not(feature = "halo2"))]
pub fn generate_halo2_proof(
    _: HashMap<String, Vec<String>>,
) -> Result<GenerateProofResult, MoproError> {
    Err(MoproError::Halo2Error(
        "Project does not have Halo2 feature enabled".to_string(),
    ))
}

#[cfg(feature = "halo2")]
pub fn verify_halo2_proof(in0: Vec<u8>, in1: Vec<u8>) -> Result<bool, MoproError> {
    halo2::verify_halo2_proof(in0, in1)
}

#[cfg(not(feature = "halo2"))]
pub fn verify_halo2_proof(_: Vec<u8>, _: Vec<u8>) -> Result<bool, MoproError> {
    Err(MoproError::Halo2Error(
        "Project does not have Halo2 feature enabled".to_string(),
    ))
}

#[cfg(feature = "circom")]
pub fn generate_circom_proof_wtns(
    in0: String,
    in1: HashMap<String, Vec<String>>,
    in2: WtnsFn,
) -> Result<GenerateProofResult, MoproError> {
    circom::generate_circom_proof_wtns(in0, in1, in2)
}

#[cfg(not(feature = "circom"))]
pub fn generate_circom_proof_wtns(
    _: String,
    _: HashMap<String, Vec<String>>,
    _: WtnsFn,
) -> Result<GenerateProofResult, MoproError> {
    Err(MoproError::CircomError("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.".to_string()))
}

#[cfg(feature = "circom")]
pub fn verify_circom_proof(in0: String, in1: Vec<u8>, in2: Vec<u8>) -> Result<bool, MoproError> {
    circom::verify_circom_proof(in0, in1, in2)
}

#[cfg(not(feature = "circom"))]
pub fn verify_circom_proof(_: String, _: Vec<u8>, _: Vec<u8>) -> Result<bool, MoproError> {
    Err(MoproError::CircomError("Project is compiled for Halo2 proving system. This function is currently not supported in Halo2.".to_string()))
}

#[cfg(feature = "circom")]
pub fn to_ethereum_proof(in0: Vec<u8>) -> ProofCalldata {
    circom::serialization::to_ethereum_proof(in0)
}

#[cfg(not(feature = "circom"))]
pub fn to_ethereum_proof(_: Vec<u8>) -> ProofCalldata {
    panic!("not built with circom");
}

#[cfg(feature = "circom")]
pub fn to_ethereum_inputs(in0: Vec<u8>) -> Vec<String> {
    circom::serialization::to_ethereum_inputs(in0)
}

#[cfg(not(feature = "circom"))]
pub fn to_ethereum_inputs(_: Vec<u8>) -> Vec<String> {
    panic!("not built with circom");
}

#[cfg(feature = "gpu-acceleration")]
pub fn arkworks_pippenger(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
) -> Result<BenchmarkResult, MoproError> {
    let benchmarks =
        msm::arkworks_pippenger::run_benchmark(instance_size, num_instance, &utils_dir)
            .unwrap()
            .into();
    Ok(benchmarks)
}

#[cfg(not(feature = "gpu-acceleration"))]
pub fn arkworks_pippenger(_: u32, _: u32, _: &str) -> Result<BenchmarkResult, MoproError> {
    Err(MoproError::MSMError(
        "gpu-acceleration feature not enabled!".to_string(),
    ))
}

#[cfg(feature = "gpu-acceleration")]
pub fn metal_msm(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
) -> Result<BenchmarkResult, MoproError> {
    let benchmarks = msm::metal::msm::run_benchmark(instance_size, num_instance, utils_dir)
        .unwrap()
        .into();
    Ok(benchmarks)
}

#[cfg(not(feature = "gpu-acceleration"))]
pub fn metal_msm(_: u32, _: u32, _: &str) -> Result<BenchmarkResult, MoproError> {
    Err(MoproError::MSMError(
        "gpu-acceleration feature not enabled!".to_string(),
    ))
}

// #[cfg(feature = "gpu-acceleration")]
// pub fn trapdoortech_zprize_msm(
//     instance_size: u32,
//     num_instance: u32,
//     utils_dir: &str,
// ) -> Result<BenchmarkResult, MoproError> {
//     let benchmarks = msm::trapdoortech_zprize_msm::run_benchmark(
//         instance_size,
//         num_instance,
//         &utils_dir,
//     )
//     .unwrap()
//     .into();
//     Ok(benchmarks)
// }

// #[cfg(not(feature = "gpu-acceleration"))]
// pub fn trapdoortech_zprize_msm(_: u32, _: u32, _: &str) -> Result<BenchmarkResult, MoproError> {
//     Err(MoproError::MSMError(
//         "gpu-acceleration feature not enabled!".to_string(),
//     ))
// }

#[derive(Debug)]
pub enum FFIError {
    MoproError(MoproError),
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

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub instance_size: u32,
    pub num_instance: u32,
    pub avg_processing_time: f64,
}

// This macro should be used in dependent crates
//
// This macro handles getting relevant functions into
// scope and calling uniffi
//
// There should be a user defined `zkey_witness_map` function
// that maps zkey file stub to a witness generation function
// see test-e2e/src/lib.rs for an example
#[macro_export]
macro_rules! app {
    () => {
        use mopro_ffi::{BenchmarkResult, GenerateProofResult, MoproError, ProofCalldata, G1, G2};
        use std::collections::HashMap;

        fn generate_halo2_proof(
            in0: HashMap<String, Vec<String>>,
        ) -> Result<GenerateProofResult, MoproError> {
            mopro_ffi::generate_halo2_proof(in0)
        }

        fn verify_halo2_proof(in0: Vec<u8>, in1: Vec<u8>) -> Result<bool, MoproError> {
            mopro_ffi::verify_halo2_proof(in0, in1)
        }

        fn generate_circom_proof(
            in0: String,
            in1: HashMap<String, Vec<String>>,
        ) -> Result<GenerateProofResult, MoproError> {
            let name = std::path::Path::new(in0.as_str()).file_name().unwrap();
            if let Ok(witness_fn) = zkey_witness_map(&name.to_str().unwrap()) {
                mopro_ffi::generate_circom_proof_wtns(in0, in1, witness_fn)
            } else {
                Err(MoproError::CircomError("Unknown ZKEY".to_string()))
            }
        }

        fn verify_circom_proof(
            in0: String,
            in1: Vec<u8>,
            in2: Vec<u8>,
        ) -> Result<bool, MoproError> {
            mopro_ffi::verify_circom_proof(in0, in1, in2)
        }

        fn to_ethereum_proof(in0: Vec<u8>) -> ProofCalldata {
            mopro_ffi::to_ethereum_proof(in0)
        }

        fn to_ethereum_inputs(in0: Vec<u8>) -> Vec<String> {
            mopro_ffi::to_ethereum_inputs(in0)
        }

        fn arkworks_pippenger(
            instance_size: u32,
            num_instance: u32,
            utils_dir: &str,
        ) -> Result<BenchmarkResult, MoproError> {
            mopro_ffi::arkworks_pippenger(instance_size, num_instance, utils_dir)
        }

        fn metal_msm(
            instance_size: u32,
            num_instance: u32,
            utils_dir: &str,
        ) -> Result<BenchmarkResult, MoproError> {
            mopro_ffi::metal_msm(instance_size, num_instance, utils_dir)
        }

        // fn trapdoortech_zprize_msm(
        //     instance_size: u32,
        //     num_instance: u32,
        //     utils_dir: &str,
        // ) -> Result<BenchmarkResult, MoproError> {
        //     mopro_ffi::trapdoortech_zprize_msm(instance_size, num_instance, utils_dir)
        // }

        uniffi::include_scaffolding!("mopro");
    };
}
