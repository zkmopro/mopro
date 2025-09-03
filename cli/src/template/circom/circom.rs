use crate::MoproError;
use circom_prover::{
    prover::{
        circom::{
            Proof as CircomProverProof, CURVE_BLS12_381, CURVE_BN254, G1 as CircomProverG1,
            G2 as CircomProverG2,
        },
        ProofLib as CircomProverProofLib,
    },
    CircomProver,
};
use num_bigint::BigUint;
use std::str::FromStr;

//
// Data structures for Circom proof representation
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

impl Into<CircomProverProofLib> for ProofLib {
    fn into(self) -> CircomProverProofLib {
        match self {
            ProofLib::Arkworks => CircomProverProofLib::Arkworks,
            ProofLib::Rapidsnark => CircomProverProofLib::Rapidsnark,
        }
    }
}

//
// Main functions for proof generation and verification
//

#[uniffi::export]
pub(crate) fn generate_circom_proof(
    zkey_path: String,
    circuit_inputs: String,
    proof_lib: ProofLib,
) -> Result<CircomProofResult, MoproError> {
    let name = std::path::Path::new(zkey_path.as_str())
        .file_name()
        .ok_or_else(|| {
            MoproError::CircomError("failed to parse file name from zkey_path".to_string())
        })?;

    let witness_fn = crate::circom_get(name.to_str().unwrap()).ok_or_else(|| {
        MoproError::CircomError(format!("Unknown ZKEY: {}", name.to_string_lossy()))
    })?;

    let ret = CircomProver::prove(proof_lib.into(), witness_fn, circuit_inputs, zkey_path)
        .map_err(|e| MoproError::CircomError(format!("Generate Proof error: {}", e)))?;

    let (proof, pub_inputs) = match ret.proof.curve.as_ref() {
        CURVE_BN254 | CURVE_BLS12_381 => (ret.proof.into(), ret.pub_inputs.into()),
        _ => {
            return Err(MoproError::CircomError(format!(
                "Unsupported curve: {}",
                ret.proof.curve
            )))
        }
    };

    Ok(CircomProofResult {
        proof,
        inputs: pub_inputs,
    })
}

#[uniffi::export]
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
    )
    .map_err(|e| MoproError::CircomError(format!("Verification error: {}", e)))
}

#[macro_export]
macro_rules! set_circom_circuits {
    // Accept any number of (key, func) pairs
    ($(($key:expr, $func:expr)),+ $(,)?) => {

        // Adjust the path if these types live elsewhere
        use circom_prover::witness::WitnessFn;

        const CIRCOM_CIRCUITS: &[(&'static str, WitnessFn)] = &[
            $(
                ($key, $func),
            )+
        ];

        #[inline]
        pub fn circom_get(name: &str) -> Option<WitnessFn> {
            #[cfg(test)]
            {
                if let Some(v) = $crate::circom::tests::circom_get_override_val(name) {
                    return Some(v);
                }
            }
            CIRCOM_CIRCUITS.iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| *v)
        }
    };
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::WitnessFn;
    use anyhow::Context;
    use anyhow::Result;
    use circom_prover::prover::PublicInputs;
    use num_bigint::BigInt;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::str::FromStr;

    const ZKEY_PATH: &str = "../test-vectors/circom/multiplier2_final.zkey";

    witnesscalc_adapter::witness!(multiplier2_witnesscalc);

    rust_witness::witness!(multiplier2);

    thread_local! {
        static CIRCOM_OVERRIDE: RefCell<Option<HashMap<&'static str, WitnessFn>>> =
            RefCell::new(None);
    }

    pub fn set_circom_override_for_test<I>(pairs: I)
    where
        I: IntoIterator<Item = (&'static str, WitnessFn)>,
    {
        let mut m = HashMap::new();
        for (k, v) in pairs {
            m.insert(k, v);
        }
        CIRCOM_OVERRIDE.with(|c| *c.borrow_mut() = Some(m));
    }

    pub fn circom_get_override_val(name: &str) -> Option<WitnessFn> {
        return CIRCOM_OVERRIDE
            .with(|cell| cell.borrow().as_ref().and_then(|m| m.get(name).cloned()));
    }

    #[test]
    fn test_witnesscalc_proof() -> Result<()> {
        // Override the default witness function for this test
        set_circom_override_for_test([(
            "multiplier2_final.zkey",
            circom_prover::witness::WitnessFn::WitnessCalc(multiplier2_witnesscalc_witness),
        )]);

        let (input_str, expected_output) = prepare_inputs();

        // Generate Proof
        let p = generate_circom_proof(ZKEY_PATH.to_string(), input_str, ProofLib::Arkworks)
            .expect("Proof generation failed");

        let CircomProofResult { proof, inputs } = p.clone();

        assert!(!proof.protocol.is_empty());
        assert!(!proof.curve.is_empty());

        let pub_inputs: PublicInputs = inputs.into();
        assert_eq!(pub_inputs.0, expected_output);

        // Step 3: Verify Proof
        let is_valid = verify_circom_proof(ZKEY_PATH.to_string(), p, ProofLib::Arkworks)
            .context("Proof verification failed")?;
        assert!(is_valid);

        Ok(())
    }

    #[test]
    fn test_rustwitness_prove() -> Result<()> {
        let (input_str, expected_output) = prepare_inputs();

        // Generate Proof
        let p = generate_circom_proof(ZKEY_PATH.to_string(), input_str, ProofLib::Arkworks)
            .expect("Proof generation failed");

        let CircomProofResult { proof, inputs } = p.clone();

        assert!(!proof.protocol.is_empty());
        assert!(!proof.curve.is_empty());

        let pub_inputs: PublicInputs = inputs.into();
        assert_eq!(pub_inputs.0, expected_output);

        // Step 3: Verify Proof
        let is_valid = verify_circom_proof(ZKEY_PATH.to_string(), p, ProofLib::Arkworks)
            .context("Proof verification failed")?;
        assert!(is_valid);

        Ok(())
    }

    #[test]
    fn test_rapidsnark_prove() -> Result<()> {
        let (input_str, expected_output) = prepare_inputs();

        // Generate Proof
        let p = generate_circom_proof(ZKEY_PATH.to_string(), input_str, ProofLib::Rapidsnark)
            .expect("Proof generation failed");

        let CircomProofResult { proof, inputs } = p.clone();

        assert!(!proof.protocol.is_empty());
        assert!(!proof.curve.is_empty());

        let pub_inputs: PublicInputs = inputs.into();
        assert_eq!(pub_inputs.0, expected_output);

        // Step 3: Verify Proof
        let is_valid = verify_circom_proof(ZKEY_PATH.to_string(), p, ProofLib::Rapidsnark)
            .context("Proof verification failed")?;
        assert!(is_valid);

        Ok(())
    }

    fn prepare_inputs() -> (String, Vec<BigUint>) {
        let mut inputs = HashMap::new();
        let a = BigInt::from_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495616",
        )
        .unwrap();
        let b = BigInt::from(1u8);
        inputs.insert("a".to_string(), vec![a.to_string()]);
        inputs.insert("b".to_string(), vec![b.to_string()]);
        let input_str = serde_json::to_string(&inputs).unwrap();

        let c = a.clone() * b.clone();
        // output = [public output c, public input a]
        let expected_output = vec![c.to_biguint().unwrap(), a.to_biguint().unwrap()];
        (input_str, expected_output)
    }
}
