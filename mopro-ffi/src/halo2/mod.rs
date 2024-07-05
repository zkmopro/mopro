extern crate proc_macro;
use crate::{GenerateProofResult, MoproError};
use std::collections::HashMap;

pub trait MoproHalo2 {
    // TODO - may be we can switch to using the Halo2 API types directly
    fn prove(input: HashMap<String, Vec<String>>) -> Result<GenerateProofResult, MoproError>;

    fn verify(proof: Vec<u8>, public_inputs: Vec<u8>) -> Result<bool, MoproError>;
}

/// This marco will generate a Halo2 circuit struct named: `<your_struct>Halo2Circuit`.
/// You can then call the `prove` and `verify` methods on this struct to interact with the circuit.
/// This macro **must** be called from the same module as where the struct is defined,
/// Or you can re-export the struct as long as it is available in this module
/// Warning: Make sure that there are no other circuits with the same name that you export
#[macro_export]
macro_rules! mopro_halo2_circuit {
    ($struct_name:ident) => {
        mopro_ffi::reexports::paste! {

            /// A separate module to avoid duplicate imports for `MoproHalo2`
            /// As well as duplicated struct names
            mod [<$struct_name _tmp_impl_module_halo2>] {
                use mopro_ffi::MoproHalo2;

                #[derive(uniffi::Object)]
                pub struct [<$struct_name Halo2Circuit>] where super::[<$struct_name>]: mopro_ffi::MoproHalo2 {}


                #[uniffi::export]
                impl [<$struct_name Halo2Circuit>] {

                    #[uniffi::constructor]
                    pub fn new() -> Self {
                        Self {}
                    }

                    pub fn prove(&self, circuit_inputs: std::collections::HashMap<String, Vec<String>>) -> Result<mopro_ffi::GenerateProofResult, crate::MoproErrorExternal> {
                        super::[<$struct_name>]::prove(circuit_inputs).map_err(|e| e.into())
                    }

                    pub fn verify(&self, proof: Vec<u8>, public_input: Vec<u8>) -> Result<bool, crate::MoproErrorExternal> {
                        super::[<$struct_name>]::verify(proof, public_input).map_err(|e| e.into())
                    }
                }
            }
        }
    };
}
