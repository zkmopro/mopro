use anyhow::Result;
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use circom::Proof;
use num::BigUint;
use serialization::SerializableInputs;
use std::{str::FromStr, thread::JoinHandle};

pub mod ark_circom;
pub mod circom;
pub mod serialization;

#[cfg(feature = "arkworks")]
pub mod arkworks;
#[cfg(feature = "rapidsnark")]
pub mod rapidsnark;

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

pub fn verify(lib: ProofLib, zkey_path: String, proof: CircomProof) -> Result<bool> {
    match lib {
        #[cfg(feature = "arkworks")]
        ProofLib::Arkworks => arkworks::verify_circom_proof(zkey_path, proof),
        #[cfg(feature = "rapidsnark")]
        ProofLib::RapidSnark => rapidsnark::verify_circom_proof(zkey_path, proof),
        #[allow(unreachable_patterns)]
        _ => panic!("Unsupported proof library"),
    }
}

//
// Helper functions to convert PublicInputs to other types we need
//
impl From<Vec<String>> for PublicInputs {
    fn from(src: Vec<String>) -> Self {
        let pi = src
            .iter()
            .map(|str| BigUint::from_str(str).unwrap())
            .collect();
        PublicInputs(pi)
    }
}

impl From<PublicInputs> for Vec<String> {
    fn from(src: PublicInputs) -> Self {
        src.0.iter().map(|p| p.to_string()).collect()
    }
}

impl<T: Pairing> From<PublicInputs> for SerializableInputs<T> {
    fn from(src: PublicInputs) -> SerializableInputs<T> {
        let si = src
            .0
            .iter()
            .map(|n| {
                if *n > BigUint::from_bytes_le(T::ScalarField::MODULUS.to_bytes_le().as_ref()) {
                    panic!("Invalid input, lager than MODULUS")
                }
                T::ScalarField::from_le_bytes_mod_order(n.to_bytes_le().as_ref())
            })
            .collect();
        SerializableInputs(si)
    }
}

impl<T: Pairing> From<SerializableInputs<T>> for PublicInputs {
    fn from(src: SerializableInputs<T>) -> PublicInputs {
        let pi = src
            .0
            .iter()
            .map(|&si| BigUint::from_bytes_le(si.into_bigint().to_bytes_le().as_ref()))
            .collect();
        PublicInputs(pi)
    }
}
