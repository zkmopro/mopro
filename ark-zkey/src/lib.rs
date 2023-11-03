use ark_bn254::Bn254;
use ark_circom::read_zkey;
use ark_groth16::ProvingKey;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use color_eyre::eyre::{self, Result, WrapErr};
use std::env;
use std::fs::File;
use std::io::{self, Cursor, Read, Write};
use std::path::PathBuf;

// NOTE: Starting with ProvingKey
// TODO: Add ConstraintMatrices

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug, PartialEq)]
pub struct SerializableProvingKey(pub ProvingKey<Bn254>);

pub fn serialize_proving_key(pk: &SerializableProvingKey) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    pk.serialize_uncompressed(&mut serialized_data)
        .expect("Serialization failed");
    serialized_data
}

pub fn deserialize_proving_key(data: Vec<u8>) -> SerializableProvingKey {
    SerializableProvingKey::deserialize_uncompressed(&mut &data[..])
        .expect("Deserialization failed")
}

pub fn convert_zkey(zkey_path: &str, arkzkey_path: &str) -> Result<()> {
    let zkey_file_path = PathBuf::from(zkey_path);
    let arkzkey_file_path = PathBuf::from(arkzkey_path);
    println!("zkey_file_path: {:?}", zkey_file_path);
    println!("arkzkey_file_path: {:?}", arkzkey_file_path);

    // Read the zkey file and get a SerializableProvingKey
    let mut zkey_file = File::open(zkey_file_path).wrap_err("Failed to open zkey file")?;

    // TODO: Add ConstraintMatrices
    let (proving_key, _) = read_zkey(&mut zkey_file).wrap_err("Failed to read zkey file")?;

    let serialized_path = PathBuf::from(arkzkey_file_path);

    let mut file =
        File::create(&serialized_path).wrap_err("Failed to create serialized proving key file")?;
    SerializableProvingKey(proving_key)
        .serialize_uncompressed(&mut file)
        .wrap_err("Failed to serialize proving key")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() -> Result<()> {
        // multiplier
        let dir = "../mopro-core/examples/circom/multiplier2";
        let circuit = "multiplier2";

        // keccak256
        // let dir = "../mopro-core/examples/circom/keccak256";
        // let circuit = "keccak256_256_test";

        let zkey_path = format!("{}/target/{}_final.zkey", dir, circuit);
        let arkzkey_path = format!("{}/target/{}_final.arkzkey", dir, circuit);

        // TODO: Also read it back and compare
        convert_zkey(&zkey_path, &arkzkey_path)?;

        Ok(())
    }
}
