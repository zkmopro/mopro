use anyhow::Result;
use circom::Proof;
use num::BigUint;
use std::{str::FromStr, thread::JoinHandle};

pub mod ark_circom;
pub mod circom;

#[cfg(feature = "arkworks")]
pub mod arkworks;
#[cfg(feature = "rapidsnark")]
pub mod rapidsnark;

#[derive(Debug, Clone)]
pub struct PublicInputs(pub Vec<BigUint>);

#[derive(Debug, Clone)]
pub struct CircomProof {
    pub proof: Proof,
    pub pub_inputs: PublicInputs,
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
