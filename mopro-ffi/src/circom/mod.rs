pub mod ethereum;
pub use ethereum::*;

use crate::GenerateProofResult;
use anyhow::Ok;
use anyhow::Result;
use circom_prover::{prover::ProofLib, witness::WitnessFn, CircomProver};

#[macro_export]
macro_rules! circom_app {
    () => {
        use mopro_ffi::witness::WitnessFn;
        fn generate_circom_proof(
            zkey_path: String,
            inputs: String,
        ) -> Result<mopro_ffi::GenerateProofResult, mopro_ffi::MoproError> {
            let name = match std::path::Path::new(zkey_path.as_str()).file_name() {
                Some(v) => v,
                None => {
                    return Err(mopro_ffi::MoproError::CircomError(format!(
                        "failed to parse file name from zkey_path"
                    )))
                }
            };
            let witness_fn = get_circom_wtns_fn(name.to_str().unwrap())?;
            mopro_ffi::generate_circom_proof_wtns(
                mopro_ffi::prover::ProofLib::Arkworks,
                zkey_path,
                inputs,
                witness_fn,
            )
            .map_err(|e| mopro_ffi::MoproError::CircomError(format!("Unknown ZKEY: {}", e)))
        }

        fn verify_circom_proof(
            zkey_path: String,
            proof_data: Vec<u8>,
            public_inputs: Vec<u8>,
        ) -> Result<bool, mopro_ffi::MoproError> {
            mopro_ffi::verify_circom_proof(
                mopro_ffi::prover::ProofLib::Arkworks,
                zkey_path,
                proof_data,
                public_inputs,
            )
            .map_err(|e| mopro_ffi::MoproError::CircomError(format!("Verification error: {}", e)))
        }

        fn to_ethereum_proof(proof_data: Vec<u8>) -> mopro_ffi::ProofCalldata {
            mopro_ffi::to_ethereum_proof(proof_data)
        }

        fn to_ethereum_inputs(public_inputs: Vec<u8>) -> Vec<String> {
            mopro_ffi::to_ethereum_inputs(public_inputs)
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
/// `get_circom_wtns_fn(circuit: &str) -> Result<mopro_ffi::witness::WitnessFn, mopro_ffi::MoproError>`.
/// You can choose to implement it directly with your custom logic:
///
/// #### Example:
/// ```ignore
/// fn get_circom_wtns_fn(circuit: &str) -> Result<mopro_ffi::witness::WitnessFn, mopro_ffi::MoproError> {
///    match circuit {
///       "circuit1.zkey" => Ok(circuit1_witness_fn),
///      _ => Err(mopro_ffi::MoproError::CircomError(format!("Unknown ZKEY: {}", circuit).to_string()))
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

// build a proof for a zkey using witness_fn to build
// the witness
pub fn generate_circom_proof_wtns(
    proof_lib: ProofLib,
    zkey_path: String,
    input_str: String,
    witness_fn: WitnessFn,
) -> Result<GenerateProofResult> {
    let ret = CircomProver::prove(proof_lib, witness_fn, input_str, zkey_path).unwrap();
    Ok(GenerateProofResult {
        proof: ret.proof,
        inputs: ret.pub_inputs,
    })
}

// Prove on a generic curve
pub fn verify_circom_proof(
    proof_lib: ProofLib,
    zkey_path: String,
    proof: Vec<u8>,
    public_inputs: Vec<u8>,
) -> Result<bool> {
    CircomProver::verify(proof_lib, proof, public_inputs, zkey_path)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::{Add, Mul};
    use std::str::FromStr;

    use crate::circom::ethereum::{to_ethereum_inputs, to_ethereum_proof};
    use crate::circom::{generate_circom_proof_wtns, verify_circom_proof};
    use crate::GenerateProofResult;
    use anyhow::bail;
    use anyhow::Result;
    use ark_bls12_381::Bls12_381;
    use ark_bn254::Bn254;
    use ark_ff::PrimeField;
    use circom_prover::prover::{serialization, ProofLib};
    use circom_prover::witness::WitnessFn;
    use num_bigint::{BigInt, BigUint, ToBigInt};

    #[cfg(feature = "circom-witness-calc")]
    mod witnesscalc {
        use circom_prover::witnesscalc_adapter;

        use super::*;
        // Only build the witness functions for tests, don't bundle them into
        // the final library
        witnesscalc_adapter::witness!(multiplier2);

        use crate as mopro_ffi;

        #[test]
        fn test_circom_macros() {
            circom_app!();

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
            let result = generate_circom_proof(ZKEY_PATH.to_string(), input_str);

            assert!(result.is_ok());
        }
    }

    #[cfg(feature = "circom-rust-witness")]
    mod rustwitness {
        use super::*;
        // Only build the witness functions for tests, don't bundle them into
        // the final library
        rust_witness::witness!(multiplier2);
        rust_witness::witness!(multiplier2bls);
        rust_witness::witness!(keccak256256test);
        rust_witness::witness!(hashbenchbls);

        use crate as mopro_ffi;

        #[test]
        fn test_circom_macros() {
            circom_app!();

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
            let result = generate_circom_proof(ZKEY_PATH.to_string(), input_str);

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
            input_str: String,
        ) -> Result<GenerateProofResult> {
            let name = std::path::Path::new(zkey_path.as_str())
                .file_name()
                .unwrap();
            if let Ok(witness_fn) = zkey_witness_map(name.to_str().unwrap()) {
                generate_circom_proof_wtns(ProofLib::Arkworks, zkey_path, input_str, witness_fn)
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
