use anyhow::Result;
use num::BigUint;

pub mod arkworks;
pub mod serialization;

pub struct ProofResult {
    pub proof: Vec<u8>,
    pub pub_inputs: Vec<u8>,
}

pub enum ProofLib {
    Arkworks,
    RapidSnark,
}

pub fn prove(lib: ProofLib, zkey_path: String, witnesses: Vec<BigUint>) -> Result<ProofResult> {
    match lib {
        ProofLib::Arkworks => arkworks::generate_circom_proof(zkey_path, witnesses),
        ProofLib::RapidSnark => panic!("Not supported yet."),
    }
}
