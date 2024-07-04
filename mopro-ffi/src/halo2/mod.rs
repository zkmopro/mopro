extern crate proc_macro;
use crate::{GenerateProofResult, MoproError};
use std::collections::HashMap;

pub trait MoproHalo2 {
    // TODO - may be we can switch to using the Halo2 API types directly
    fn prove(input: HashMap<String, Vec<String>>) -> Result<GenerateProofResult, MoproError>;

    fn verify(proof: Vec<u8>, public_inputs: Vec<u8>) -> Result<bool, MoproError>;
}

#[macro_export]
macro_rules! mopro_halo2_circuit {
    ($struct_name:ident) => {
        paste::paste! {
            #[derive(uniffi::Object)]
            pub struct [<$struct_name Halo2Mopro>] where [<$struct_name>]: mopro_ffi::MoproHalo2 {}

            #[uniffi::export]
            impl [<$struct_name Halo2Mopro>] {

                #[uniffi::constructor]
                pub fn new() -> Self {
                    Self {}
                }

                pub fn prove(&self, in1: std::collections::HashMap<String, Vec<String>>) -> Result<mopro_ffi::GenerateProofResult, crate::MoproErrorExternal> {
                    [<$struct_name>]::prove(in1).map_err(|e| crate::MoproErrorExternal::from(e))
                }

                pub fn verify(&self, in1: Vec<u8>, in2: Vec<u8>) -> Result<bool, crate::MoproErrorExternal> {
                    [<$struct_name>]::verify(in1, in2).map_err(|e| crate::MoproErrorExternal::from(e))
                }
            }
        }
    };
}
