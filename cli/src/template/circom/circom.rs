use circom_prover::{
    prover::{
        circom::{
            Proof as CircomProverProof, CURVE_BLS12_381, CURVE_BN254, G1 as CircomProverG1,
            G2 as CircomProverG2,
        },
        ProofLib as CircomProverProofLib,
    },
    witness::WitnessFn,
    CircomProver,
};
use num_bigint::BigUint;
use std::str::FromStr;
use crate::MoproError;

 //
// Circom Section
//
#[derive(Debug, Clone, uniffi::Record)]
pub struct CircomProofResult {
    pub proof: CircomProof,
    pub inputs: Vec<String>,
}

#[derive(Debug, Clone, Default, uniffi::Record)]
pub struct G1 {
    pub x: String,
    pub y: String,
    pub z: String,
}

#[derive(Debug, Clone, Default, uniffi::Record)]
pub struct G2 {
    pub x: Vec<String>,
    pub y: Vec<String>,
    pub z: Vec<String>,
}

#[derive(Debug, Clone, Default, uniffi::Record)]
pub struct CircomProof {
    pub a: G1,
    pub b: G2,
    pub c: G1,
    pub protocol: String,
    pub curve: String,
}

#[derive(Debug, Clone, Default, uniffi::Enum)]
pub enum ProofLib {
    #[default]
    Arkworks,
    Rapidsnark,
}

impl Into<CircomProverProofLib> for ProofLib {
    fn into(self) -> CircomProverProofLib {
        match self {
            ProofLib::Arkworks => CircomProverProofLib::Arkworks,
            ProofLib::Rapidsnark => CircomProverProofLib::Rapidsnark,
        }
    }
}

#[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
pub(crate) fn generate_circom_proof(
    zkey_path: String,
    circuit_inputs: String,
    proof_lib: ProofLib,
) -> Result<CircomProofResult, MoproError> {
    let chosen_proof_lib = proof_lib.into();
    let name = match std::path::Path::new(zkey_path.as_str()).file_name() {
        Some(v) => v,
        None => {
            return Err(MoproError::CircomError(format!(
                "failed to parse file name from zkey_path"
            )))
        }
    };
    let witness_fn = crate::get_circom_wtns_fn(name.to_str().unwrap())
        .map_err(|e| MoproError::CircomError(format!("Unknown ZKEY: {}", e)))?;
    let result = generate_circom_proof_wtns(
        chosen_proof_lib,
        zkey_path,
        circuit_inputs,
        witness_fn,
    )
    .map_err(|e| MoproError::CircomError(format!("Unknown ZKEY: {}", e)))
    .unwrap();

    Ok(result.into())
}

#[cfg_attr(not(feature = "no_uniffi_exports"), uniffi::export)]
pub(crate) fn verify_circom_proof(
    zkey_path: String,
    proof_result: CircomProofResult,
    proof_lib: ProofLib,
) -> Result<bool, MoproError> {
    let chosen_proof_lib = proof_lib.into();
    CircomProver::verify(
        chosen_proof_lib,
        circom_prover::prover::CircomProof {
            proof: proof_result.proof.into(),
            pub_inputs: proof_result.inputs.into(),
        },
        zkey_path,
    ).map_err(|e| MoproError::CircomError(format!("Verification error: {}", e)))
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
/// `get_circom_wtns_fn(circuit: &str) -> Result<circom_prover::witness::WitnessFn>`.
/// You can choose to implement it directly with your custom logic:
///
/// #### Example:
/// ```ignore
/// fn get_circom_wtns_fn(circuit: &str) -> Result<circom_prover::witness::WitnessFn> {
///    match circuit {
///       "circuit1.zkey" => Ok(circuit1_witness_fn),
///      _ => Err(MoproError::CircomError(format!("Unknown ZKEY: {}", circuit).to_string()))
///   }
/// }
/// ```
macro_rules! set_circom_circuits {
    ($(($key:expr, $func:expr)),+ $(,)?) => {
        fn get_circom_wtns_fn(circuit: &str) -> Result<circom_prover::witness::WitnessFn, MoproError> {
            match circuit {
                $(
                   $key => Ok($func),
                )+
                _ => Err(MoproError::CircomError(format!("Unknown ZKEY: {}", circuit)))
            }
        }
    };
}

pub fn generate_circom_proof_wtns(
    proof_lib: CircomProverProofLib,
    zkey_path: String,
    json_input_str: String,
    witness_fn: WitnessFn,
) -> anyhow::Result<CircomProofResult> {
    let ret = CircomProver::prove(proof_lib, witness_fn, json_input_str, zkey_path)?;
    let (proof, public_inputs) = match ret.proof.curve.as_ref() {
        CURVE_BN254 => (ret.proof.into(), ret.pub_inputs.into()),
        CURVE_BLS12_381 => (ret.proof.into(), ret.pub_inputs.into()),
        _ => anyhow::bail!("Not unsupported curve"),
    };
    Ok(CircomProofResult {
        proof,
        inputs: public_inputs,
    })
}

//
// `From` implementation for proof conversion
//
impl From<CircomProverProof> for CircomProof {
    fn from(proof: CircomProverProof) -> Self {
        CircomProof {
            a: proof.a.into(),
            b: proof.b.into(),
            c: proof.c.into(),
            protocol: proof.protocol,
            curve: proof.curve,
        }
    }
}

impl From<CircomProof> for CircomProverProof {
    fn from(proof: CircomProof) -> Self {
        CircomProverProof {
            a: proof.a.into(),
            b: proof.b.into(),
            c: proof.c.into(),
            protocol: proof.protocol,
            curve: proof.curve,
        }
    }
}

impl From<CircomProverG1> for G1 {
    fn from(g1: CircomProverG1) -> Self {
        G1 {
            x: g1.x.to_string(),
            y: g1.y.to_string(),
            z: g1.z.to_string(),
        }
    }
}

impl From<G1> for CircomProverG1 {
    fn from(g1: G1) -> Self {
        CircomProverG1 {
            x: BigUint::from_str(g1.x.as_str()).unwrap(),
            y: BigUint::from_str(g1.y.as_str()).unwrap(),
            z: BigUint::from_str(g1.z.as_str()).unwrap(),
        }
    }
}

impl From<CircomProverG2> for G2 {
    fn from(g2: CircomProverG2) -> Self {
        let x = vec![g2.x[0].to_string(), g2.x[1].to_string()];
        let y = vec![g2.y[0].to_string(), g2.y[1].to_string()];
        let z = vec![g2.z[0].to_string(), g2.z[1].to_string()];
        G2 { x, y, z }
    }
}

impl From<G2> for CircomProverG2 {
    fn from(g2: G2) -> Self {
        let x =
            g2.x.iter()
                .map(|p| BigUint::from_str(p.as_str()).unwrap())
                .collect::<Vec<BigUint>>();
        let y =
            g2.y.iter()
                .map(|p| BigUint::from_str(p.as_str()).unwrap())
                .collect::<Vec<BigUint>>();
        let z =
            g2.z.iter()
                .map(|p| BigUint::from_str(p.as_str()).unwrap())
                .collect::<Vec<BigUint>>();
        CircomProverG2 {
            x: [x[0].clone(), x[1].clone()],
            y: [y[0].clone(), y[1].clone()],
            z: [z[0].clone(), z[1].clone()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use num_bigint::BigInt;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[cfg(feature = "witnesscalc")]
    mod witnesscalc {
        use super::*;
        use crate as mopro_ffi;

        // Only build the witness functions for tests, don't bundle them into
        // the final library
        witnesscalc_adapter::witness!(multiplier2_witnesscalc);

        #[test]
        fn test_circom_macros() {
            set_circom_circuits! {
                ("multiplier2_final.zkey", circom_prover::witness::WitnessFn::WitnessCalc(multiplier2_witnesscalc_witness)),
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
                ProofLib::Arkworks,
            )
            .expect("Proof generation failed");

            let is_valid = verify_circom_proof(
                ZKEY_PATH.to_string(),
                proof,
                ProofLib::Arkworks,
            )
            .expect("Proof verification failed");

            assert!(is_valid, "Expected the proof to be valid");
        }
    }

    mod rustwitness {
        use super::*;
        use circom_prover::prover::{PublicInputs};

        // Only build the witness functions for tests, don't bundle them into
        // the final library
        rust_witness::witness!(multiplier2);
        rust_witness::witness!(multiplier2bls);
        rust_witness::witness!(keccak256256test);
        rust_witness::witness!(hashbenchbls);

        #[test]
        fn test_circom_macros() {
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
                ProofLib::Arkworks,
            )
            .expect("Proof generation failed");

            let is_valid = verify_circom_proof(
                ZKEY_PATH.to_string(),
                proof,
                ProofLib::Arkworks,
            )
            .expect("Proof verification failed");

            assert!(is_valid, "Expected the proof to be valid");
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
            let p = generate_circom_proof(zkey_path.clone(), input_str, ProofLib::Arkworks)?;
            let proof = p.proof.clone();
            let pub_inputs: PublicInputs = p.inputs.clone().into();

            assert!(!proof.protocol.is_empty());
            assert!(!proof.curve.is_empty());
            assert_eq!(pub_inputs.0, expected_output);

            // Step 3: Verify Proof
            let is_valid = verify_circom_proof(zkey_path, p, ProofLib::Arkworks)?;
            assert!(is_valid);

            Ok(())
        }
    }
}
