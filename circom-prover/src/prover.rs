use anyhow::Result;
use circom::Proof;
use num::BigUint;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, thread::JoinHandle};

pub mod ark_circom;
pub mod circom;

#[cfg(feature = "arkworks")]
pub mod arkworks;
#[cfg(feature = "rapidsnark")]
pub mod rapidsnark;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicInputs(pub Vec<BigUint>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircomProof {
    pub proof: Proof,
    pub pub_inputs: PublicInputs,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prover::circom::{Proof, G1, G2};
    use num::BigUint;

    #[test]
    fn serde_roundtrip_circom_proof() {
        let a = G1 {
            x: BigUint::from(1u32),
            y: BigUint::from(2u32),
            z: BigUint::from(1u32),
        };
        let b = G2 {
            x: [BigUint::from(3u32), BigUint::from(4u32)],
            y: [BigUint::from(5u32), BigUint::from(6u32)],
            z: [BigUint::from(1u32), BigUint::from(0u32)],
        };
        let c = G1 {
            x: BigUint::from(7u32),
            y: BigUint::from(8u32),
            z: BigUint::from(1u32),
        };
        let proof = Proof {
            a,
            b,
            c,
            protocol: "groth16".to_string(),
            curve: "bn128".to_string(),
        };
        let pub_inputs = PublicInputs(vec![BigUint::from(9u32), BigUint::from(10u32)]);
        let cp = CircomProof { proof, pub_inputs };

        let serialized = serde_json::to_string(&cp).unwrap();
        let deserialized: CircomProof = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cp, deserialized);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProofLib {
    Arkworks,
    Rapidsnark,
}

pub fn prove(
    lib: ProofLib,
    zkey_path: String,
    witnesses: JoinHandle<Vec<BigUint>>,
) -> Result<CircomProof> {
    match lib {
        #[cfg(feature = "arkworks")]
        ProofLib::Arkworks => arkworks::generate_circom_proof(zkey_path, witnesses),
        #[cfg(feature = "rapidsnark")]
        ProofLib::Rapidsnark => rapidsnark::generate_circom_proof(zkey_path, witnesses),
        #[allow(unreachable_patterns)]
        _ => panic!("Unsupported proof library"),
    }
}

pub fn verify(lib: ProofLib, zkey_path: String, proof: CircomProof) -> Result<bool> {
    match lib {
        #[cfg(feature = "arkworks")]
        ProofLib::Arkworks => arkworks::verify_circom_proof(zkey_path, proof),
        #[cfg(feature = "rapidsnark")]
        ProofLib::Rapidsnark => rapidsnark::verify_circom_proof(zkey_path, proof),
        #[allow(unreachable_patterns)]
        _ => panic!("Unsupported proof library"),
    }
}

//
// Helper functions to convert PublicInputs to other types we need
//
impl From<Vec<String>> for PublicInputs {
    fn from(src: Vec<String>) -> Self {
        let pi = src
            .iter()
            .map(|str| BigUint::from_str(str).unwrap())
            .collect();
        PublicInputs(pi)
    }
}

impl From<PublicInputs> for Vec<String> {
    fn from(src: PublicInputs) -> Self {
        src.0.iter().map(|p| p.to_string()).collect()
    }
}
