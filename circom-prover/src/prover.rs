use anyhow::Result;
use ethereum::Proof;
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

pub struct PublicInputs(pub Vec<BigUint>);

pub struct CircomProof {
    pub proof: Proof,
    pub pub_inputs: PublicInputs,
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
    public_inputs: PublicInputs,
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
