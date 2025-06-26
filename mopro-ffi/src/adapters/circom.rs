pub use num_bigint::BigUint;

#[cfg(feature = "circom-witnesscalc")]
pub use circom_prover::graph;

pub use circom_prover::{prover::{ProofLib, CircomProof, circom::{Proof,  CURVE_BLS12_381, CURVE_BN254, G1,
            G2}}, witness, CircomProver};

/// This macro is provided for backward compatibility.
#[macro_export]
macro_rules! set_circom_circuits {
    ($(($key:expr, $func:expr)),+ $(,)?) => {
        mopro_ffi::circom_app! { $( ($key, $func) ),+ }
    }
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
macro_rules! circom_app {
    ($(($key:expr, $func:expr)),+ $(,)?) => {

        mopro_ffi::circom_setup!();

        #[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
        fn generate_circom_proof(
            zkey_path: String,
            circuit_inputs: String,
            proof_lib: CircomProofLib,
        ) -> Result<CircomProofResult, crate::MoproError> {
            let name = match std::path::Path::new(zkey_path.as_str()).file_name() {
                Some(v) => v,
                None => {
                    return Err(crate::MoproError::CircomError(format!(
                        "failed to parse file name from zkey_path"
                    )))
                }
            };
            let witness_fn = get_circom_wtns_fn(name.to_str().unwrap())?;
            mopro_ffi::circom::CircomProver::prove(proof_lib.into(), witness_fn, circuit_inputs, zkey_path)
                .map_err(|e| crate::MoproError::CircomError(format!("Unknown ZKEY: {}", e)))
            .map(|result| {
                CircomProofResult {
                    proof: result.proof.into(),
                    inputs: result.pub_inputs.into(),
                }
            })
        }

        #[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
        fn verify_circom_proof(
            zkey_path: String,
            proof_result: CircomProofResult,
            proof_lib: CircomProofLib,
        ) -> Result<bool, crate::MoproError> {
            mopro_ffi::circom::CircomProver::verify(
                proof_lib.into(),
                mopro_ffi::circom::CircomProof {
                    proof: proof_result.proof.into(),
                    pub_inputs: proof_result.inputs.into(),
                },
                zkey_path,
            )
                .map_err(|e| crate::MoproError::CircomError(format!("Verification error: {}", e)))
        }

        fn get_circom_wtns_fn(circuit: &str) -> Result<mopro_ffi::circom::witness::WitnessFn, crate::MoproError> {
            match circuit {
                $(
                   $key => Ok($func),
                )+
                _ => Err(crate::MoproError::CircomError(format!("Unknown ZKEY: {}", circuit)))
            }
        }
    };
}

#[macro_export]
macro_rules! circom_setup {
    () => {
        #[derive(Debug, Clone, Default, uniffi::Enum)]
        pub enum CircomProofLib {
            #[default]
            Arkworks,
            Rapidsnark,
        }

        #[derive(Debug, Clone)]
        #[cfg_attr(not(feature = "no_uniffi_exports"), derive(uniffi::Record))]
        pub struct CircomProofResult {
            pub proof: CircomProof,
            pub inputs: Vec<String>,
        }

        #[derive(Debug, Clone, Default)]
        #[cfg_attr(not(feature = "no_uniffi_exports"), derive(uniffi::Record))]
        pub struct CircomProof {
            pub a: G1,
            pub b: G2,
            pub c: G1,
            pub protocol: String,
            pub curve: String,
        }

        #[derive(Debug, Clone, Default)]
        #[cfg_attr(not(feature = "no_uniffi_exports"), derive(uniffi::Record))]
        pub struct G1 {
            pub x: String,
            pub y: String,
            pub z: String,
        }

        #[derive(Debug, Clone, Default)]
        #[cfg_attr(not(feature = "no_uniffi_exports"), derive(uniffi::Record))]
        pub struct G2 {
            pub x: Vec<String>,
            pub y: Vec<String>,
            pub z: Vec<String>,
        }

        //
        // `From` implementation for proof conversion
        //
        impl From<CircomProofLib> for mopro_ffi::circom::ProofLib {
            fn from(lib: CircomProofLib) -> Self {
                match lib {
                    CircomProofLib::Arkworks => mopro_ffi::circom::ProofLib::Arkworks,
                    CircomProofLib::Rapidsnark => mopro_ffi::circom::ProofLib::Rapidsnark,
                }
            }
        }

        impl From<mopro_ffi::circom::Proof> for CircomProof {
            fn from(proof: mopro_ffi::circom::Proof) -> Self {
                CircomProof {
                    a: proof.a.into(),
                    b: proof.b.into(),
                    c: proof.c.into(),
                    protocol: proof.protocol,
                    curve: proof.curve,
                }
            }
        }

        impl From<CircomProof> for mopro_ffi::circom::Proof {
            fn from(proof: CircomProof) -> Self {
                mopro_ffi::circom::Proof {
                    a: proof.a.into(),
                    b: proof.b.into(),
                    c: proof.c.into(),
                    protocol: proof.protocol,
                    curve: proof.curve,
                }
            }
        }

        impl From<mopro_ffi::circom::G1> for G1 {
            fn from(g1: mopro_ffi::circom::G1) -> Self {
                G1 {
                    x: g1.x.to_string(),
                    y: g1.y.to_string(),
                    z: g1.z.to_string(),
                }
            }
        }

        impl From<G1> for mopro_ffi::circom::G1 {
            fn from(g1: G1) -> Self {
                mopro_ffi::circom::G1 {
                    x: <mopro_ffi::circom::BigUint as std::str::FromStr>::from_str(g1.x.as_str()).unwrap(),
                    y: <mopro_ffi::circom::BigUint as std::str::FromStr>::from_str(g1.y.as_str()).unwrap(),
                    z: <mopro_ffi::circom::BigUint as std::str::FromStr>::from_str(g1.z.as_str()).unwrap(),
                }
            }
        }

        impl From<mopro_ffi::circom::G2> for G2 {
            fn from(g2: mopro_ffi::circom::G2) -> Self {
                let x = vec![g2.x[0].to_string(), g2.x[1].to_string()];
                let y = vec![g2.y[0].to_string(), g2.y[1].to_string()];
                let z = vec![g2.z[0].to_string(), g2.z[1].to_string()];
                G2 { x, y, z }
            }
        }

        impl From<G2> for mopro_ffi::circom::G2 {
            fn from(g2: G2) -> Self {
                let x =
                    g2.x.iter()
                        .map(|p| <mopro_ffi::circom::BigUint as std::str::FromStr>::from_str(p.as_str()).unwrap())
                        .collect::<Vec<mopro_ffi::circom::BigUint>>();
                let y =
                    g2.y.iter()
                        .map(|p| <mopro_ffi::circom::BigUint as std::str::FromStr>::from_str(p.as_str()).unwrap())
                        .collect::<Vec<mopro_ffi::circom::BigUint>>();
                let z =
                    g2.z.iter()
                        .map(|p| <mopro_ffi::circom::BigUint as std::str::FromStr>::from_str(p.as_str()).unwrap())
                        .collect::<Vec<mopro_ffi::circom::BigUint>>();
                mopro_ffi::circom::G2 {
                    x: [x[0].clone(), x[1].clone()],
                    y: [y[0].clone(), y[1].clone()],
                    z: [z[0].clone(), z[1].clone()],
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use num_bigint::BigInt;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[cfg(feature = "witnesscalc")]
    mod witnesscalc {

        // Only build the witness functions for tests, don't bundle them into
        // the final library
        witnesscalc_adapter::witness!(multiplier2_witnesscalc);

        #[cfg(feature = "no_uniffi_exports")]
        #[test]
        fn test_circom_macros() {
            circom_app!(
                mopro_ffi::CircomProofResult,
                mopro_ffi::CircomProof,
                mopro_ffi::MoproError,
                circom_prover::prover::ProofLib
            );

            set_circom_circuits! {
                ("multiplier2_final.zkey", mopro_ffi::witness::WitnessFn::WitnessCalc(multiplier2_witnesscalc_witness)),
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
            let proof = generate_circom_proof(
                ZKEY_PATH.to_string(),
                input_str,
                circom_prover::prover::ProofLib::Arkworks,
            )
            .expect("Proof generation failed");

            let is_valid = verify_circom_proof(
                ZKEY_PATH.to_string(),
                proof,
                circom_prover::prover::ProofLib::Arkworks,
            )
            .expect("Proof verification failed");

            assert!(is_valid, "Expected the proof to be valid");
        }
    }

    #[cfg(feature = "rustwitness")]
    mod rustwitness {
        use super::*;
        use anyhow::bail;
        use ark_ff::PrimeField;
        use circom_prover::prover::{ProofLib, PublicInputs};
        use circom_prover::witness::WitnessFn;
        use num_bigint::{BigUint, ToBigInt};
        use std::ops::{Add, Mul};

        // Only build the witness functions for tests, don't bundle them into
        // the final library
        rust_witness::witness!(multiplier2);
        rust_witness::witness!(multiplier2bls);
        rust_witness::witness!(keccak256256test);
        rust_witness::witness!(hashbenchbls);

        #[cfg(feature = "no_uniffi_exports")]
        #[test]
        fn test_circom_macros() {
            circom_app!(
                mopro_ffi::CircomProofResult,
                mopro_ffi::CircomProof,
                mopro_ffi::MoproError,
                circom_prover::prover::ProofLib
            );

            set_circom_circuits! {
                ("multiplier2_final.zkey", mopro_ffi::witness::WitnessFn::RustWitness(multiplier2_witness)),
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
            let proof = generate_circom_proof(
                ZKEY_PATH.to_string(),
                input_str,
                circom_prover::prover::ProofLib::Arkworks,
            )
            .expect("Proof generation failed");

            let is_valid = verify_circom_proof(
                ZKEY_PATH.to_string(),
                proof,
                circom_prover::prover::ProofLib::Arkworks,
            )
            .expect("Proof verification failed");

            assert!(is_valid, "Expected the proof to be valid");
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
        ) -> Result<CircomProofResult> {
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

        fn bytes_to_circuit_outputs(bytes: &[u8]) -> Vec<BigUint> {
            let bits = bytes_to_bits(bytes);
            bits.into_iter()
                .map(|bit| BigUint::from(bit as u8))
                .collect()
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
                c.clone().to_biguint().unwrap(),
                a.clone().to_biguint().unwrap(),
            ];

            // Generate Proof
            let input_str = serde_json::to_string(&inputs).unwrap();
            let p = generate_circom_proof(zkey_path.clone(), input_str)?;
            let proof = p.proof.clone();
            let pub_inputs: PublicInputs = p.inputs.clone().into();

            assert!(!proof.protocol.is_empty());
            assert!(!proof.curve.is_empty());
            assert_eq!(pub_inputs.0, expected_output);

            // Step 3: Verify Proof
            let is_valid = verify_circom_proof(ProofLib::Arkworks, zkey_path, p)?;
            assert!(is_valid);

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
            let proof = p.proof.clone();
            let pub_inputs: PublicInputs = p.inputs.clone().into();

            assert!(!proof.protocol.is_empty());
            assert!(!proof.curve.is_empty());
            assert_eq!(pub_inputs.0, serialized_outputs);

            // Verify Proof
            let is_valid = verify_circom_proof(ProofLib::Arkworks, zkey_path, p)?;
            assert!(is_valid);

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
            let proof = p.proof.clone();

            let pub_inputs: PublicInputs = p.inputs.clone().into();
            assert!(!proof.protocol.is_empty());
            assert!(!proof.curve.is_empty());
            assert_eq!(pub_inputs.0[0], expected_output);

            // Step 3: Verify Proof
            let is_valid = verify_circom_proof(ProofLib::Arkworks, zkey_path, p)?;
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
            let expected_output = vec![c.to_biguint().unwrap()];

            // Generate Proof
            let input_str = serde_json::to_string(&inputs).unwrap();
            let p = generate_circom_proof(zkey_path.clone(), input_str)?;
            let proof = p.proof.clone();
            let pub_inputs: PublicInputs = p.inputs.clone().into();

            assert!(!proof.protocol.is_empty());
            assert!(!proof.curve.is_empty());
            assert_eq!(pub_inputs.0, expected_output);

            // Step 3: Verify Proof
            let is_valid = verify_circom_proof(ProofLib::Arkworks, zkey_path, p)?;
            assert!(is_valid);

            Ok(())
        }
    }
}
