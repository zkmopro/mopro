use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Halo2CircuitBindings)]
pub fn halo2_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let struct_name_str = name.to_string();
    let new_struct_name = syn::Ident::new(&format!("{}Halo2Mopro", struct_name_str), name.span());

    let expanded = quote! {
        // Struct to define the prove and verify methods
        // as well as enforce the trait bound on MoproHalo2
        #[derive(uniffi::Object)]
        struct #new_struct_name where #name: mopro_ffi::MoproHalo2;

        #[uniffi::export]
        impl #new_struct_name {
            #[uniffi::constructor]
            fn new() -> Self {
                #new_struct_name
            }

            pub fn prove(&self, in0: HashMap<String, Vec<String>>) -> Result<mopro_ffi::GenerateProofResult, crate::MoproErrorExternal> {
                #name::prove(in0).map_err(|e| crate::MoproErrorExternal::from(e))
            }

            pub fn verify(&self, in0: Vec<u8>, in1: Vec<u8>) -> Result<bool, crate::MoproErrorExternal> {
            #name::verify(in0, in1).map_err(|e| crate::MoproErrorExternal::from(e))
            }
        }
    };

    TokenStream::from(expanded)
}

// #[proc_macro_derive(CircomCircuitBindings)]
