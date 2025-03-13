pub mod ethereum;

use anyhow::{bail, Ok, Result};
use ark_bls12_381::Bls12_381;
use ark_bn254::Bn254;
use ark_ff::PrimeField;
pub use ethereum::*;
use num_bigint::BigUint;

use crate::GenerateProofResult;
use circom_prover::{
    prover::{
        ark_circom::ZkeyHeaderReader,
        ethereum::{CURVE_BLS12_381, CURVE_BN254},
        serialization::{self, SerializableProof},
        ProofLib,
    },
    witness::WitnessFn,
    CircomProver,
};

#[macro_export]
macro_rules! circom_app {
    ($result:ty, $proof_call_data:ty, $err:ty, $proof_lib:ty) => {
        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn generate_circom_proof(
            zkey_path: String,
            circuit_inputs: String,
            proof_lib: $proof_lib,
        ) -> Result<$result, $err> {
            let name = match std::path::Path::new(zkey_path.as_str()).file_name() {
                Some(v) => v,
                None => {
                    return Err(<$err>::CircomError(format!(
                        "failed to parse file name from zkey_path"
                    )))
                }
            };
            let witness_fn = get_circom_wtns_fn(name.to_str().unwrap())?;
            let chosen_proof_lib = match proof_lib {
                <$proof_lib>::Arkworks => mopro_ffi::prover::ProofLib::Arkworks,
                <$proof_lib>::Rapidsnark => mopro_ffi::prover::ProofLib::RapidSnark,
            };
            let result = mopro_ffi::generate_circom_proof_wtns(
                chosen_proof_lib,
                zkey_path,
                circuit_inputs,
                witness_fn,
            )
            .map_err(|e| <$err>::CircomError(format!("Unknown ZKEY: {}", e)))
            .unwrap();

            Ok(result.into())
        }

        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn verify_circom_proof(
            zkey_path: String,
            proof: Vec<u8>,
            public_input: Vec<u8>,
            proof_lib: $proof_lib,
        ) -> Result<bool, $err> {
            let chosen_proof_lib = match proof_lib {
                <$proof_lib>::Arkworks => mopro_ffi::prover::ProofLib::Arkworks,
                <$proof_lib>::Rapidsnark => mopro_ffi::prover::ProofLib::RapidSnark,
            };
            mopro_ffi::verify_circom_proof(chosen_proof_lib, zkey_path, proof, public_input)
                .map_err(|e| <$err>::CircomError(format!("Verification error: {}", e)))
        }

        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn to_ethereum_proof(proof: Vec<u8>) -> $proof_call_data {
            mopro_ffi::to_ethereum_proof(proof).into()
        }

        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn to_ethereum_inputs(inputs: Vec<u8>) -> Vec<String> {
            mopro_ffi::to_ethereum_inputs(inputs)
        }

        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn from_ethereum_proof(proof: $proof_call_data) -> Vec<u8> {
            mopro_ffi::from_ethereum_proof(proof.into())
        }

        #[allow(dead_code)]
        #[cfg_attr(not(disable_uniffi_export), uniffi::export)]
        fn from_ethereum_inputs(inputs: Vec<String>) -> Vec<u8> {
            mopro_ffi::from_ethereum_inputs(inputs)
        }
    };
}

/// Set the circuits that can be proven by the mopro library
/// Provide the circuits that you want to be able to generate proofs for
/// as a list of pairs of the form `zkey`, `wtns_fn`
/// Where `zkey` is the name of the zkey file
/// and `wtns_fn` is the function that generates the witness for the circuit.
///
/// ## How to use:
/// You should only use this macro once, in the same module as the `mopro_ffi::app!()`
/// To use this macro, make sure to have `mopro-ffi/circom` feature enabled
///
/// #### Example:
///
///
/// ```ignore
/// mopro_ffi::app!();
///
/// set_circom_circuits! {
///   ("circuit1.zkey", circuit1_witness_fn),
///   ("circuit2.zkey", circuit2_witness_fn),
/// }
/// ```
///
///
/// ## For Advanced Users:
/// This macro is abstracting away the implementation of
/// `get_circom_wtns_fn(circuit: &str) -> Result<mopro_ffi::witness::WitnessFn>`.
/// You can choose to implement it directly with your custom logic:
///
/// #### Example:
/// ```ignore
/// fn get_circom_wtns_fn(circuit: &str) -> Result<mopro_ffi::witness::WitnessFn> {
///    match circuit {
///       "circuit1.zkey" => Ok(circuit1_witness_fn),
///      _ => Err(MoproError::CircomError(format!("Unknown ZKEY: {}", circuit).to_string()))
///   }
/// }
/// ```
#[macro_export]
macro_rules! set_circom_circuits {
    ($(($key:expr, $func:expr)),+ $(,)?) => {
        fn get_circom_wtns_fn(circuit: &str) -> Result<mopro_ffi::witness::WitnessFn, mopro_ffi::MoproError> {
            match circuit {
                $(
                   $key => Ok($func),
                )+
                _ => Err(mopro_ffi::MoproError::CircomError(format!("Unknown ZKEY: {}", circuit)))
            }
        }
    };
}

pub fn generate_circom_proof_wtns(
    proof_lib: ProofLib,
    zkey_path: String,
    json_input_str: String,
    witness_fn: WitnessFn,
) -> Result<GenerateProofResult> {
    let ret = CircomProver::prove(proof_lib, witness_fn, json_input_str, zkey_path).unwrap();
    let (proof, public_inputs) = match ret.proof.curve.as_ref() {
        CURVE_BN254 => (
            serialization::serialize_proof(&SerializableProof::<Bn254>(ret.proof.into())),
            serialization::serialize_inputs::<Bn254>(&ret.pub_inputs.into()),
        ),
        CURVE_BLS12_381 => (
            serialization::serialize_proof(&SerializableProof::<Bls12_381>(ret.proof.into())),
            serialization::serialize_inputs::<Bls12_381>(&ret.pub_inputs.into()),
        ),
        _ => bail!("Not uspported curve"),
    };
    Ok(GenerateProofResult {
        proof,
        inputs: public_inputs,
    })
}

// Prove on a generic curve
pub fn verify_circom_proof(
    proof_lib: ProofLib,
    zkey_path: String,
    proof: Vec<u8>,
    public_inputs: Vec<u8>,
) -> Result<bool> {
    // TODO fix this workaround, change `public_inputs: Vec<u8>` to `public_inputs: Vec<String>`
    let mut header_reader = ZkeyHeaderReader::new(&zkey_path);
    header_reader.read();
    let public_inputs = if header_reader.r == BigUint::from(ark_bn254::Fr::MODULUS) {
        serialization::deserialize_inputs::<Bn254>(public_inputs).into()
    } else if header_reader.r == BigUint::from(ark_bls12_381::Fr::MODULUS) {
        serialization::deserialize_inputs::<Bls12_381>(public_inputs).into()
    } else {
        // unknown curve
        bail!("unknown curve detected in zkey")
    };

    CircomProver::verify(proof_lib, proof, public_inputs, zkey_path)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use num_bigint::BigInt;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[cfg(feature = "witnesscalc")]
    mod witnesscalc {
        use super::*;
        use crate as mopro_ffi;
        use circom_prover::witness::WitnessFn;
        use circom_prover::witnesscalc_adapter;

        // Only build the witness functions for tests, don't bundle them into
        // the final library
        witnesscalc_adapter::witness!(multiplier2);

        #[test]
        fn test_circom_macros() {
            circom_app!(
                mopro_ffi::GenerateProofResult,
                mopro_ffi::ProofCalldata,
                mopro_ffi::MoproError,
                mopro_ffi::ProofLib
            );

            set_circom_circuits! {
                ("multiplier2_final.zkey", WitnessFn::WitnessCalc(multiplier2_witness)),
            }

            const ZKEY_PATH: &str = "../test-vectors/circom/multiplier2_final.zkey";

            let mut inputs = HashMap::new();
            let a = BigInt::from_str(
                "21888242871839275222246405745257275088548364400416034343698204186575808495616",
            )
            .unwrap();
            let b = BigInt::from(1u8);
            inputs.insert("a".to_string(), vec![a.to_string()]);
            inputs.insert("b".to_string(), vec![b.to_string()]);

            let input_str = serde_json::to_string(&inputs).unwrap();
            let result = generate_circom_proof(
                ZKEY_PATH.to_string(),
                input_str,
                mopro_ffi::ProofLib::Arkworks,
            );

            assert!(result.is_ok());
        }
    }

    #[cfg(feature = "rustwitness")]
    mod rustwitness {
        use super::*;
        use crate::circom::ethereum::{to_ethereum_inputs, to_ethereum_proof};
        use crate::circom::{generate_circom_proof_wtns, verify_circom_proof};
        use crate::GenerateProofResult;
        use anyhow::bail;
        use ark_bls12_381::Bls12_381;
        use ark_bn254::Bn254;
        use ark_ff::PrimeField;
        use circom_prover::prover::{serialization, ProofLib};
        use circom_prover::witness::WitnessFn;
        use num_bigint::{BigUint, ToBigInt};
        use std::ops::{Add, Mul};

        // Only build the witness functions for tests, don't bundle them into
        // the final library
        rust_witness::witness!(multiplier2);
        rust_witness::witness!(multiplier2bls);
        rust_witness::witness!(keccak256256test);
        rust_witness::witness!(hashbenchbls);

        use crate as mopro_ffi;

        #[test]
        fn test_circom_macros() {
            circom_app!(
                mopro_ffi::GenerateProofResult,
                mopro_ffi::ProofCalldata,
                mopro_ffi::MoproError,
                mopro_ffi::ProofLib
            );

            set_circom_circuits! {
                ("multiplier2_final.zkey", WitnessFn::RustWitness(multiplier2_witness)),
            }

            const ZKEY_PATH: &str = "../test-vectors/circom/multiplier2_final.zkey";

            let mut inputs = HashMap::new();
            let a = BigInt::from_str(
                "21888242871839275222246405745257275088548364400416034343698204186575808495616",
            )
            .unwrap();
            let b = BigInt::from(1u8);
            inputs.insert("a".to_string(), vec![a.to_string()]);
            inputs.insert("b".to_string(), vec![b.to_string()]);

            let input_str = serde_json::to_string(&inputs).unwrap();
            let result = generate_circom_proof(
                ZKEY_PATH.to_string(),
                input_str,
                mopro_ffi::ProofLib::Arkworks,
            );

            assert!(result.is_ok());
        }

        // This should be defined by a file that the mopro package consumer authors
        // then we reference it in our build somehow
        fn zkey_witness_map(name: &str) -> Result<WitnessFn> {
            match name {
                "multiplier2_final.zkey" => Ok(WitnessFn::RustWitness(multiplier2_witness)),
                "keccak256_256_test_final.zkey" => {
                    Ok(WitnessFn::RustWitness(keccak256256test_witness))
                }
                "hashbench_bls_final.zkey" => Ok(WitnessFn::RustWitness(hashbenchbls_witness)),
                "multiplier2_bls_final.zkey" => Ok(WitnessFn::RustWitness(multiplier2bls_witness)),
                _ => bail!("Unknown circuit name"),
            }
        }

        fn generate_circom_proof(
            zkey_path: String,
            json_input_str: String,
        ) -> Result<GenerateProofResult> {
            let name = std::path::Path::new(zkey_path.as_str())
                .file_name()
                .unwrap();
            if let Ok(witness_fn) = zkey_witness_map(name.to_str().unwrap()) {
                generate_circom_proof_wtns(
                    ProofLib::Arkworks,
                    zkey_path,
                    json_input_str,
                    witness_fn,
                )
            } else {
                bail!("unknown zkey");
            }
        }

        fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
            let mut bits = Vec::new();
            for &byte in bytes {
                for j in 0..8 {
                    let bit = (byte >> j) & 1;
                    bits.push(bit == 1);
                }
            }
            bits
        }

        fn bytes_to_circuit_inputs(input_vec: &[u8]) -> HashMap<String, Vec<String>> {
            let bits = bytes_to_bits(input_vec);
            let converted_vec: Vec<String> = bits
                .into_iter()
                .map(|bit| (bit as i32).to_string())
                .collect();
            let mut inputs = HashMap::new();
            inputs.insert("in".to_string(), converted_vec);
            inputs
        }

        fn bytes_to_circuit_outputs(bytes: &[u8]) -> Vec<u8> {
            let bits = bytes_to_bits(bytes);
            let field_bits = bits
                .into_iter()
                .map(|bit| ark_bn254::Fr::from(bit as u8))
                .collect();
            let circom_outputs = serialization::SerializableInputs::<Bn254>(field_bits);
            serialization::serialize_inputs(&circom_outputs)
        }

        #[test]
        fn test_prove() -> Result<()> {
            // Create a new MoproCircom instance
            let zkey_path = "../test-vectors/circom/multiplier2_final.zkey".to_string();

            let mut inputs = HashMap::new();
            let a = BigInt::from_str(
                "21888242871839275222246405745257275088548364400416034343698204186575808495616",
            )
            .unwrap();
            let b = BigInt::from(1u8);
            let c = a.clone() * b.clone();
            inputs.insert("a".to_string(), vec![a.to_string()]);
            inputs.insert("b".to_string(), vec![b.to_string()]);
            // output = [public output c, public input a]
            let expected_output = vec![
                ark_bn254::Fr::from(c.clone().to_biguint().unwrap()),
                ark_bn254::Fr::from(a.clone().to_biguint().unwrap()),
            ];
            let circom_outputs = serialization::SerializableInputs::<Bn254>(expected_output);
            let serialized_outputs = serialization::serialize_inputs(&circom_outputs);

            // Generate Proof
            let input_str = serde_json::to_string(&inputs).unwrap();
            let p = generate_circom_proof(zkey_path.clone(), input_str)?;
            let serialized_proof = p.proof;
            let serialized_inputs = p.inputs;

            assert!(!serialized_proof.is_empty());
            assert_eq!(serialized_inputs, serialized_outputs);

            // Step 3: Verify Proof
            let is_valid = verify_circom_proof(
                ProofLib::Arkworks,
                zkey_path,
                serialized_proof.clone(),
                serialized_inputs.clone(),
            )?;
            assert!(is_valid);

            // Step 4: Convert Proof to Ethereum compatible proof
            let proof_calldata = to_ethereum_proof(serialized_proof);
            let inputs_calldata = to_ethereum_inputs(serialized_inputs);
            assert!(!proof_calldata.a.x.is_empty());
            assert!(!inputs_calldata.is_empty());

            Ok(())
        }

        #[test]
        fn test_prove_keccak() -> Result<()> {
            // Create a new MoproCircom instance
            let zkey_path = "../test-vectors/circom/keccak256_256_test_final.zkey".to_string();
            // Prepare inputs
            let input_vec = vec![
                116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0,
            ];

            // Expected output
            let expected_output_vec = vec![
                37, 17, 98, 135, 161, 178, 88, 97, 125, 150, 143, 65, 228, 211, 170, 133, 153, 9,
                88, 212, 4, 212, 175, 238, 249, 210, 214, 116, 170, 85, 45, 21,
            ];

            let inputs = bytes_to_circuit_inputs(&input_vec);
            let serialized_outputs = bytes_to_circuit_outputs(&expected_output_vec);

            // Generate Proof
            let input_str = serde_json::to_string(&inputs).unwrap();
            let p = generate_circom_proof(zkey_path.clone(), input_str)?;
            let serialized_proof = p.proof;
            let serialized_inputs = p.inputs;

            assert!(!serialized_proof.is_empty());
            assert_eq!(serialized_inputs, serialized_outputs);

            // Verify Proof

            let is_valid = verify_circom_proof(
                ProofLib::Arkworks,
                zkey_path,
                serialized_proof.clone(),
                serialized_inputs.clone(),
            )?;
            assert!(is_valid);

            // Step 4: Convert Proof to Ethereum compatible proof
            let proof_calldata = to_ethereum_proof(serialized_proof);
            let inputs_calldata = to_ethereum_inputs(serialized_inputs);
            assert!(!proof_calldata.a.x.is_empty());
            assert!(!inputs_calldata.is_empty());

            Ok(())
        }

        #[test]
        fn test_prove_bls_hashbench() -> Result<()> {
            // Create a new MoproCircom instance
            let zkey_path = "../test-vectors/circom/hashbench_bls_final.zkey".to_string();

            let mut inputs = HashMap::new();
            let a = BigInt::from(1);
            let b = BigInt::from(1);
            inputs.insert("inputs".to_string(), vec![a.to_string(), b.to_string()]);

            // The hashbench circuit repeatedly calculates poseidon hashes. We'll
            // hardcode the expected output here
            let expected_output = BigUint::from_str(
                "30695856561167821618075419048973910422865797477786596477999317197379707456163",
            )
            .unwrap();

            // Generate Proof
            let input_str = serde_json::to_string(&inputs).unwrap();
            let p = generate_circom_proof(zkey_path.clone(), input_str)?;
            let serialized_proof = p.proof;
            let serialized_inputs = p.inputs.clone();

            assert!(!serialized_proof.is_empty());

            let output = serialization::deserialize_inputs::<Bls12_381>(p.inputs).0[0];
            assert_eq!(BigUint::from(output), expected_output);

            // Step 3: Verify Proof
            let is_valid = verify_circom_proof(
                ProofLib::Arkworks,
                zkey_path,
                serialized_proof.clone(),
                serialized_inputs.clone(),
            )?;
            assert!(is_valid);

            Ok(())
        }

        #[test]
        fn test_prove_bls_multiplier2() -> Result<()> {
            // Create a new MoproCircom instance
            let zkey_path = "../test-vectors/circom/multiplier2_bls_final.zkey".to_string();

            let mut inputs = HashMap::new();
            // we're using large numbers to ensure we're in the bls field
            let a = BigInt::from(2).pow(250);
            let b: BigInt = BigInt::from(2).pow(254).add(1240);
            let c = a.clone().mul(b.clone())
                % BigUint::from(ark_bls12_381::Fr::MODULUS)
                    .to_bigint()
                    .unwrap();
            inputs.insert("a".to_string(), vec![a.to_string()]);
            inputs.insert("b".to_string(), vec![b.to_string()]);
            // output = [public output c, public input a]
            let expected_output = vec![ark_bls12_381::Fr::from(c.to_biguint().unwrap())];
            let circom_outputs = serialization::SerializableInputs::<Bls12_381>(expected_output);
            let serialized_outputs = serialization::serialize_inputs(&circom_outputs);

            // Generate Proof
            let input_str = serde_json::to_string(&inputs).unwrap();
            let p = generate_circom_proof(zkey_path.clone(), input_str)?;
            let serialized_proof = p.proof;
            let serialized_inputs = p.inputs;

            assert!(!serialized_proof.is_empty());
            assert_eq!(serialized_inputs, serialized_outputs);

            // Step 3: Verify Proof
            let is_valid = verify_circom_proof(
                ProofLib::Arkworks,
                zkey_path,
                serialized_proof.clone(),
                serialized_inputs.clone(),
            )?;
            assert!(is_valid);

            // We don't support formatting for ethereum for the BLS curve.
            // Once the hardfork enables the bls precompile we should
            // revisit this
            //
            // // Step 4: Convert Proof to Ethereum compatible proof
            // let proof_calldata = to_ethereum_proof(serialized_proof);
            // let inputs_calldata = to_ethereum_inputs(serialized_inputs);
            // assert!(proof_calldata.a.x.len() > 0);
            // assert!(inputs_calldata.len() > 0);

            Ok(())
        }
    }
}
