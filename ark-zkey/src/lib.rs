use ark_bn254::{Bn254, Fr};
use ark_circom::read_zkey;
use ark_ff::Field;
use ark_groth16::ProvingKey;
use ark_relations::r1cs::ConstraintMatrices;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use color_eyre::eyre::{Result, WrapErr};
use memmap2::Mmap;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::time::Instant;

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug, PartialEq)]
pub struct SerializableProvingKey(pub ProvingKey<Bn254>);

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug, PartialEq)]
pub struct SerializableMatrix<F: Field> {
    pub data: Vec<Vec<(F, usize)>>,
}

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug, PartialEq)]
pub struct SerializableConstraintMatrices<F: Field> {
    pub num_instance_variables: usize,
    pub num_witness_variables: usize,
    pub num_constraints: usize,
    pub a_num_non_zero: usize,
    pub b_num_non_zero: usize,
    pub c_num_non_zero: usize,
    pub a: SerializableMatrix<F>,
    pub b: SerializableMatrix<F>,
    pub c: SerializableMatrix<F>,
}

impl<F: Field> From<Vec<Vec<(F, usize)>>> for SerializableMatrix<F> {
    fn from(matrix: Vec<Vec<(F, usize)>>) -> Self {
        SerializableMatrix { data: matrix }
    }
}

impl<F: Field> From<SerializableMatrix<F>> for Vec<Vec<(F, usize)>> {
    fn from(serializable_matrix: SerializableMatrix<F>) -> Self {
        serializable_matrix.data
    }
}

pub fn serialize_proving_key(pk: &SerializableProvingKey) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    pk.serialize_compressed(&mut serialized_data)
        .expect("Serialization failed");
    serialized_data
}

pub fn deserialize_proving_key(data: Vec<u8>) -> SerializableProvingKey {
    SerializableProvingKey::deserialize_compressed(&mut &data[..]).expect("Deserialization failed")
}

const ZKEY_DATA: &[u8] =
    include_bytes!("../../mopro-core/examples/circom/multiplier2/target/multiplier2_final.zkey");

pub fn read_proving_key_and_matrices(
) -> Result<(SerializableProvingKey, SerializableConstraintMatrices<Fr>)> {
    println!("Processing zkey data...");
    let now = Instant::now();

    let mut cursor = Cursor::new(ZKEY_DATA);

    let (proving_key, matrices) = read_zkey(&mut cursor).wrap_err("Failed to process zkey data")?;
    println!("Time to process zkey data: {:?}", now.elapsed());

    println!("Serializing proving key and constraint matrices");
    let now = Instant::now();
    let serializable_proving_key = SerializableProvingKey(proving_key);
    let serializable_constrain_matrices = SerializableConstraintMatrices {
        num_instance_variables: matrices.num_instance_variables,
        num_witness_variables: matrices.num_witness_variables,
        num_constraints: matrices.num_constraints,
        a_num_non_zero: matrices.a_num_non_zero,
        b_num_non_zero: matrices.b_num_non_zero,
        c_num_non_zero: matrices.c_num_non_zero,
        a: SerializableMatrix { data: matrices.a },
        b: SerializableMatrix { data: matrices.b },
        c: SerializableMatrix { data: matrices.c },
    };
    println!(
        "Time to serialize proving key and constraint matrices: {:?}",
        now.elapsed()
    );

    Ok((serializable_proving_key, serializable_constrain_matrices))
}

pub fn read_arkzkey(
    arkzkey_path: &str,
) -> Result<(SerializableProvingKey, SerializableConstraintMatrices<Fr>)> {
    let now = std::time::Instant::now();
    let arkzkey_file_path = PathBuf::from(arkzkey_path);
    let arkzkey_file = File::open(arkzkey_file_path).wrap_err("Failed to open arkzkey file")?;
    println!("Time to open arkzkey file: {:?}", now.elapsed());

    //let mut buf_reader = BufReader::new(arkzkey_file);

    // Using mmap
    let now = std::time::Instant::now();
    let mmap = unsafe { Mmap::map(&arkzkey_file)? };
    let mut cursor = std::io::Cursor::new(mmap);
    println!("Time to mmap arkzkey: {:?}", now.elapsed());

    // Was &mut buf_reader
    let now = std::time::Instant::now();
    let proving_key = SerializableProvingKey::deserialize_compressed(&mut cursor)
        .wrap_err("Failed to deserialize proving key")?;
    println!("Time to deserialize proving key: {:?}", now.elapsed());

    let now = std::time::Instant::now();
    let constraint_matrices = SerializableConstraintMatrices::deserialize_compressed(&mut cursor)
        .wrap_err("Failed to deserialize constraint matrices")?;
    println!("Time to deserialize matrices: {:?}", now.elapsed());

    Ok((proving_key, constraint_matrices))
}

pub fn read_proving_key_and_matrices_from_zkey(
    zkey_path: &str,
) -> Result<(SerializableProvingKey, SerializableConstraintMatrices<Fr>)> {
    println!("Reading zkey from: {}", zkey_path);
    let now = Instant::now();
    let zkey_file_path = PathBuf::from(zkey_path);
    let mut zkey_file = File::open(zkey_file_path).wrap_err("Failed to open zkey file")?;

    let mut buf_reader = BufReader::new(zkey_file);

    let (proving_key, matrices) =
        read_zkey(&mut buf_reader).wrap_err("Failed to read zkey file")?;
    println!("Time to read zkey: {:?}", now.elapsed());

    println!("Serializing proving key and constraint matrices");
    let now = Instant::now();
    let serializable_proving_key = SerializableProvingKey(proving_key);
    let serializable_constrain_matrices = SerializableConstraintMatrices {
        num_instance_variables: matrices.num_instance_variables,
        num_witness_variables: matrices.num_witness_variables,
        num_constraints: matrices.num_constraints,
        a_num_non_zero: matrices.a_num_non_zero,
        b_num_non_zero: matrices.b_num_non_zero,
        c_num_non_zero: matrices.c_num_non_zero,
        a: SerializableMatrix { data: matrices.a },
        b: SerializableMatrix { data: matrices.b },
        c: SerializableMatrix { data: matrices.c },
    };
    println!(
        "Time to serialize proving key and constraint matrices: {:?}",
        now.elapsed()
    );

    Ok((serializable_proving_key, serializable_constrain_matrices))
}

pub fn convert_zkey(
    proving_key: SerializableProvingKey,
    constraint_matrices: SerializableConstraintMatrices<Fr>,
    arkzkey_path: &str,
) -> Result<()> {
    let arkzkey_file_path = PathBuf::from(arkzkey_path);

    let serialized_path = PathBuf::from(arkzkey_file_path);

    let mut file =
        File::create(&serialized_path).wrap_err("Failed to create serialized proving key file")?;

    proving_key
        .serialize_compressed(&mut file)
        .wrap_err("Failed to serialize proving key")?;

    constraint_matrices
        .serialize_compressed(&mut file)
        .wrap_err("Failed to serialize constraint matrices")?;

    Ok(())
}

fn read_zkey_with_mmap(zkey_path: &str) -> Result<(ProvingKey<Bn254>, ConstraintMatrices<Fr>)> {
    let file = File::open(zkey_path)?;

    let mmap = unsafe { Mmap::map(&file)? };

    let cursor = Cursor::new(&mmap);
    let (proving_key, matrices) = read_zkey(&mut cursor.clone())?;

    Ok((proving_key, matrices))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_serialization_deserialization() -> Result<()> {
        // multiplier
        let dir = "../mopro-core/examples/circom/multiplier2";
        let circuit = "multiplier2";

        //keccak256
        // let dir = "../mopro-core/examples/circom/keccak256";
        // let circuit = "keccak256_256_test";

        let zkey_path = format!("{}/target/{}_final.zkey", dir, circuit);
        let arkzkey_path = format!("{}/target/{}_final.arkzkey", dir, circuit);

        // Not much faster
        // println!("Reading mmaped zkey from: {}", zkey_path);
        // let now = Instant::now();
        // let (original_proving_key, original_constraint_matrices) = read_zkey_with_mmap(&zkey_path)?;
        // println!("Time to read mmaped zkey: {:?}", now.elapsed());

        let (original_proving_key, original_constraint_matrices) =
            read_proving_key_and_matrices_from_zkey(&zkey_path)?;

        let (_proving_key2, _constraint_matrices2) = read_proving_key_and_matrices()?;

        println!("Writing arkzkey to: {}", arkzkey_path);
        let now = Instant::now();
        convert_zkey(
            original_proving_key.clone(),
            original_constraint_matrices.clone(),
            &arkzkey_path,
        )?;
        println!("Time to write zkey: {:?}", now.elapsed());

        println!("Reading arkzkey from: {}", arkzkey_path);
        let now = Instant::now();
        let (deserialized_proving_key, deserialized_constraint_matrices) =
            read_arkzkey(&arkzkey_path)?;
        println!("Time to read arkzkey: {:?}", now.elapsed());

        assert_eq!(
            original_proving_key, deserialized_proving_key,
            "Original and deserialized proving keys do not match"
        );

        assert_eq!(
            original_constraint_matrices, deserialized_constraint_matrices,
            "Original and deserialized constraint matrices do not match"
        );

        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();

        Ok(())
    }
}
