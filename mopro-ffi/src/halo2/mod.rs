extern crate proc_macro;
use crate::{GenerateProofResult, MoproError};
use std::collections::HashMap;

pub trait MoproHalo2 {
    // TODO - we can switch to using the Halo2 API types directly
    fn prove(input: HashMap<String, Vec<String>>) -> Result<GenerateProofResult, MoproError>;

    fn verify(proof: Vec<u8>, public_inputs: Vec<u8>) -> Result<bool, MoproError>;
}
