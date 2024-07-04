use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[cfg(feature = "halo2")]
#[proc_macro_derive(Halo2Mopro)]
pub fn halo2_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let struct_name_str = name.to_string();
    let prove_fn_name = syn::Ident::new(
        &format!("prove_halo2_{}", struct_name_str.to_lowercase()),
        name.span(),
    );
    let verify_fn_name = syn::Ident::new(
        &format!("verify_halo2_{}", struct_name_str.to_lowercase()),
        name.span(),
    );
    let new_struct_name = syn::Ident::new(&format!("{}Halo2Mopro", struct_name_str), name.span());

    let expanded = quote! {
        // Helper struct to enforce the trait bound
        #[derive(uniffi::Object)]
        struct #new_struct_name where #name: Halo2Mopro;

        #[uniffi::export]
        impl #new_struct_name {
            #[uniffi::constructor]
            fn new() -> Self {
                #new_struct_name
            }

            pub fn prove(&self, in0: HashMap<String, Vec<String>>) -> Result<GenerateProofResult, MoproError1> {
                #name::prove(in0).map_err(|e| MoproError1::Halo2Error(e.to_string()))
            }

            pub fn verify(&self, in0: Vec<u8>, in1: Vec<u8>) -> Result<bool, MoproError1> {
            #name::verify(in0, in1).map_err(|e| MoproError1::Halo2Error(e.to_string()))
            }
        }

        #[uniffi::export]
        pub fn #prove_fn_name(
            in0: HashMap<String, Vec<String>>
        ) -> Result<GenerateProofResult, MoproError1> {
                #name::prove(in0).map_err(|e| MoproError1::Halo2Error(e.to_string()))
        }

        #[uniffi::export]
        pub fn #verify_fn_name(in0: Vec<u8>, in1: Vec<u8>) -> Result<bool, MoproError1> {
            #name::verify(in0, in1).map_err(|e| MoproError1::Halo2Error(e.to_string()))
        }

    };

    TokenStream::from(expanded)
}
