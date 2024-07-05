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
        mopro_ffi::reexports::paste! {
            #[derive(uniffi::Object)]
            pub struct [<$struct_name Halo2Mopro>] where [<$struct_name>]: mopro_ffi::MoproHalo2 {}

            /// A separate module to avoid duplicate imports for `MoproHalo2`
            mod [<$struct_name _tmp_impl_mod>] {
                use mopro_ffi::MoproHalo2;
                use super::{[<$struct_name Halo2Mopro>], [<$struct_name>]};

                #[uniffi::export]
                impl [<$struct_name Halo2Mopro>] {

                    #[uniffi::constructor]
                    pub fn new() -> Self {
                        Self {}
                    }

                    pub fn prove(&self, circuit_inputs: std::collections::HashMap<String, Vec<String>>) -> Result<mopro_ffi::GenerateProofResult, crate::MoproErrorExternal> {
                        [<$struct_name>]::prove(circuit_inputs).map_err(|e| e.into())
                    }

                    pub fn verify(&self, proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, crate::MoproErrorExternal> {
                        [<$struct_name>]::verify(proof, public_input).map_err(|e| e.into())
                    }
                }
            }
        }
    };
}
