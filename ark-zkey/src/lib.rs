use ark_bn254::Bn254;
use ark_circom::read_zkey;
use ark_groth16::ProvingKey;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use color_eyre::eyre::{Result, WrapErr};
use std::fs::File;
use std::io::Read;
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

pub fn read_arkzkey(arkzkey_path: &str) -> Result<SerializableProvingKey> {
    let arkzkey_file_path = PathBuf::from(arkzkey_path);
    let mut arkzkey_file = File::open(arkzkey_file_path).wrap_err("Failed to open arkzkey file")?;
    let mut serialized_data = Vec::new();
    arkzkey_file
        .read_to_end(&mut serialized_data)
        .wrap_err("Failed to read arkzkey file")?;
    Ok(
        SerializableProvingKey::deserialize_uncompressed(&mut &serialized_data[..])
            .wrap_err("Failed to deserialize proving key")?,
    )
}

pub fn read_proving_key_from_zkey(zkey_path: &str) -> Result<SerializableProvingKey> {
    let zkey_file_path = PathBuf::from(zkey_path);
    let mut zkey_file = File::open(zkey_file_path).wrap_err("Failed to open zkey file")?;
    let (proving_key, _) = read_zkey(&mut zkey_file).wrap_err("Failed to read zkey file")?;
    Ok(SerializableProvingKey(proving_key))
}

// TODO: Add ConstraintMatrices
pub fn convert_zkey(proving_key: SerializableProvingKey, arkzkey_path: &str) -> Result<()> {
    let arkzkey_file_path = PathBuf::from(arkzkey_path);
    println!("arkzkey_file_path: {:?}", arkzkey_file_path);

    let serialized_path = PathBuf::from(arkzkey_file_path);

    let mut file =
        File::create(&serialized_path).wrap_err("Failed to create serialized proving key file")?;
    proving_key
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

        // Read the original proving key
        let original_proving_key = read_proving_key_from_zkey(&zkey_path)?;
        convert_zkey(original_proving_key.clone(), &arkzkey_path)?;

        // Read the serialized and then deserialized proving key
        let deserialized_proving_key = read_arkzkey(&arkzkey_path)?;

        // Compare the original and deserialized proving keys
        assert_eq!(
            original_proving_key, deserialized_proving_key,
            "Original and deserialized proving keys do not match"
        );

        Ok(())
    }
}
