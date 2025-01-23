use anyhow::Result;
use num::BigUint;

pub mod arkworks;
pub mod serialization;

pub struct CircomProof {
    pub proof: Vec<u8>,
    pub pub_inputs: Vec<u8>,
}

pub enum ProofLib {
    Arkworks,
    RapidSnark,
}

pub fn prove(lib: ProofLib, zkey_path: String, witnesses: Vec<BigUint>) -> Result<CircomProof> {
    match lib {
        ProofLib::Arkworks => arkworks::generate_circom_proof(zkey_path, witnesses),
        ProofLib::RapidSnark => panic!("Not supported yet."),
    }
}

pub fn verify(
    lib: ProofLib,
    zkey_path: String,
    proof: Vec<u8>,
    public_inputs: Vec<u8>,
) -> Result<bool> {
    match lib {
        ProofLib::Arkworks => arkworks::verify_circom_proof(zkey_path, proof, public_inputs),
        ProofLib::RapidSnark => panic!("Not supported yet."),
    }
}
