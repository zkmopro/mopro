use std::ffi::CString;
use std::fs::File;
use std::os::raw::c_char;
use std::os::raw::c_uint;
use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;
use ark_bn254::Bn254;
use ark_circom::read_proving_key;
use ark_circom::ZkeyHeaderReader;
use num_bigint::BigInt;
use serde::Deserialize;
use serde::Serialize;

use super::WtnsFn;

// match what rapidsnark expects
#[derive(Debug, Serialize, Deserialize)]
struct VerificationKey {
    protocol: String,
    curve: String,
    nPublic: u32,
    vk_alpha_1: [String; 3],
    vk_beta_2: [[String; 2]; 3],
    vk_gamma_2: [[String; 2]; 3],
    vk_delta_2: [[String; 2]; 3],
    IC: Vec<[String; 3]>,
}

#[repr(C)]
pub struct ProofResult {
    proof: *mut c_char,
    public_signals: *mut c_char,
}

extern "C" {
    fn groth16_api_prove(
        zkeyFilename: *const c_char,
        wtnsData: *mut u8,
        wtnsDataLen: c_uint,
    ) -> *mut ProofResult;
    fn groth16_api_verify(proof: *mut ProofResult, key_json: *const c_char) -> bool;
    fn free_proof_result(result: *mut ProofResult);
}

pub fn verify_proof(zkey_path: &str, proof: String, public_signals: String) -> Result<bool> {
    let mut header_reader = ZkeyHeaderReader::new(&zkey_path);
    header_reader.read();
    let file = File::open(&zkey_path)?;
    let mut reader = std::io::BufReader::new(file);
    let proving_key = read_proving_key::<_, Bn254>(&mut reader)?;
    // convert out proving key to json so we can
    // use it with rapidsnark
    let vk = proving_key.vk;
    // let v = proving_key.vk.alpha_g1.to_string();
    let vkey = VerificationKey {
        protocol: "groth16".to_string(),
        curve: "bn128".to_string(),
        nPublic: 0, // this is unused in the rapidsnark verifier
        vk_alpha_1: [
            vk.alpha_g1.x.to_string(),
            vk.alpha_g1.y.to_string(),
            "1".to_string(),
        ],
        vk_beta_2: [
            [vk.beta_g2.x.c0.to_string(), vk.beta_g2.x.c1.to_string()],
            [vk.beta_g2.y.c0.to_string(), vk.beta_g2.y.c1.to_string()],
            ["1".to_string(), "0".to_string()],
        ],
        vk_gamma_2: [
            [vk.gamma_g2.x.c0.to_string(), vk.gamma_g2.x.c1.to_string()],
            [vk.gamma_g2.y.c0.to_string(), vk.gamma_g2.y.c1.to_string()],
            ["1".to_string(), "0".to_string()],
        ],
        vk_delta_2: [
            [vk.delta_g2.x.c0.to_string(), vk.delta_g2.x.c1.to_string()],
            [vk.delta_g2.y.c0.to_string(), vk.delta_g2.y.c1.to_string()],
            ["1".to_string(), "0".to_string()],
        ],
        IC: vk
            .gamma_abc_g1
            .iter()
            .map(|p| [p.x.to_string(), p.y.to_string(), "1".to_string()])
            .collect(),
    };
    let vkey_json = serde_json::to_string(&vkey)?;
    let vkey_json_cstr = CString::new(vkey_json)?;
    unsafe {
        let result = groth16_api_verify(
            &mut ProofResult {
                proof: CString::new(proof).unwrap().into_raw(),
                public_signals: CString::new(public_signals).unwrap().into_raw(),
            },
            vkey_json_cstr.as_ptr(),
        );
        Ok(result)
    }
}

pub fn generate_proof(
    zkey_path: &str,
    inputs: std::collections::HashMap<String, Vec<String>>,
    witness_fn: WtnsFn,
) -> Result<(String, String)> {
    // Form the inputs
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

    let mut wtns = witness_fn(bigint_inputs)
        .into_iter()
        .map(|w| w.to_biguint().unwrap())
        .map(|v| {
            let mut bytes = v.to_bytes_le();
            bytes.resize(32, 0);
            bytes
        })
        .flatten()
        .collect::<Vec<_>>();

    // Convert Rust strings to C strings
    let zkey_cstr = CString::new(zkey_path).context("Failed to create CString for zkey path")?;

    unsafe {
        let proof_ptr =
            groth16_api_prove(zkey_cstr.as_ptr(), wtns.as_mut_ptr(), wtns.len() as c_uint);

        if proof_ptr.is_null() {
            return Err(anyhow::anyhow!("Proof generation failed"));
        }

        // Convert both strings
        let result = &*proof_ptr;
        let proof = std::ffi::CStr::from_ptr(result.proof)
            .to_string_lossy()
            .into_owned();
        let public_signals = std::ffi::CStr::from_ptr(result.public_signals)
            .to_string_lossy()
            .into_owned();

        free_proof_result(proof_ptr);
        Ok((proof, public_signals))
    }
}
