use anyhow::Result;
use num::BigUint;
use std::thread::JoinHandle;

pub mod ark_circom;
#[cfg(feature = "arkworks")]
pub mod arkworks;
#[cfg(feature = "ethereum")]
pub mod ethereum;
#[cfg(feature = "rapidsnark")]
pub mod rapidsnark;
pub mod serialization;

pub struct CircomProof {
    pub proof: Vec<u8>,
    pub pub_inputs: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum ProofLib {
    Arkworks,
    RapidSnark,
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
        ProofLib::RapidSnark => rapidsnark::generate_circom_proof(zkey_path, witnesses),
        #[allow(unreachable_patterns)]
        _ => panic!("Unsupported proof library"),
    }
}

pub fn verify(
    lib: ProofLib,
    zkey_path: String,
    proof: Vec<u8>,
    public_inputs: Vec<u8>,
) -> Result<bool> {
    match lib {
        #[cfg(feature = "arkworks")]
        ProofLib::Arkworks => arkworks::verify_circom_proof(zkey_path, proof, public_inputs),
        #[cfg(feature = "rapidsnark")]
        ProofLib::RapidSnark => rapidsnark::verify_circom_proof(zkey_path, proof, public_inputs),
        #[allow(unreachable_patterns)]
        _ => panic!("Unsupported proof library"),
    }
}
