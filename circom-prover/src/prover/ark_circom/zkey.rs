//! ZKey Parsing
//!
//! Each ZKey file is broken into sections:
//!  Header(1)
//!       Prover Type 1 Groth
//!  HeaderGroth(2)
//!       n8q
//!       q
//!       n8r
//!       r
//!       NVars
//!       NPub
//!       DomainSize  (multiple of 2
//!       alpha1
//!       beta1
//!       delta1
//!       beta2
//!       gamma2
//!       delta2
//!  IC(3)
//!  Coefs(4)
//!  PointsA(5)
//!  PointsB1(6)
//!  PointsB2(7)
//!  PointsC(8)
//!  PointsH(9)
//!  Contributions(10)
use ark_bls12_381::Bls12_381;
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger256, BigInteger384, Field, PrimeField};
use ark_relations::r1cs::ConstraintMatrices;
use ark_serialize::{CanonicalDeserialize, SerializationError};
use ark_std::log2;
use byteorder::{LittleEndian, ReadBytesExt};

use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    io::{Read, Seek, SeekFrom},
    marker::PhantomData,
};

use ark_bn254::Bn254;
use ark_groth16::{ProvingKey, VerifyingKey};
use num_bigint::BigUint;
use num_traits::Zero;
use std::fs::File;

type IoResult<T> = Result<T, SerializationError>;

pub trait FieldSerialization: Pairing {
    type Fr: Field;
    type Fq;
    type Fq2;

    fn deserialize_field_fr<R: Read>(reader: &mut R) -> IoResult<Self::Fr>;
    fn deserialize_field<R: Read>(reader: &mut R) -> IoResult<Self::Fq>;
    fn deserialize_field2<R: Read>(reader: &mut R) -> IoResult<Self::Fq2>;
    fn deserialize_g1<R: Read>(reader: &mut R) -> IoResult<Self::G1Affine>;
    fn deserialize_g2<R: Read>(reader: &mut R) -> IoResult<Self::G2Affine>;
    fn deserialize_g1_vec<R: Read>(reader: &mut R, n_vars: u32) -> IoResult<Vec<Self::G1Affine>>;
    fn deserialize_g2_vec<R: Read>(reader: &mut R, n_vars: u32) -> IoResult<Vec<Self::G2Affine>>;
}

impl FieldSerialization for Bn254 {
    type Fr = ark_bn254::Fr;
    type Fq = ark_bn254::Fq;
    type Fq2 = ark_bn254::Fq2;

    // need to divide by R, since snarkjs outputs the zkey with coefficients
    // multiplieid by R^2
    fn deserialize_field_fr<R: Read>(reader: &mut R) -> IoResult<Self::Fr> {
        let bigint = BigInteger256::deserialize_uncompressed(reader)?;
        Ok(Self::Fr::new_unchecked(
            Self::Fr::new_unchecked(bigint).into_bigint(),
        ))
    }

    // skips the multiplication by R because Circom points are already in Montgomery form
    fn deserialize_field<R: Read>(reader: &mut R) -> IoResult<Self::Fq> {
        let bigint = BigInteger256::deserialize_uncompressed_unchecked(reader)?;
        // if you use Fq::new it multiplies by R
        Ok(Self::Fq::new_unchecked(bigint))
    }

    fn deserialize_field2<R: Read>(reader: &mut R) -> IoResult<Self::Fq2> {
        let c0 = Self::deserialize_field(reader)?;
        let c1 = Self::deserialize_field(reader)?;
        Ok(Self::Fq2::new(c0, c1))
    }

    fn deserialize_g1<R: Read>(reader: &mut R) -> IoResult<Self::G1Affine> {
        let x = Self::deserialize_field(reader)?;
        let y = Self::deserialize_field(reader)?;
        let infinity = x.is_zero() && y.is_zero();
        if infinity {
            Ok(Self::G1Affine::identity())
        } else {
            Ok(Self::G1Affine::new_unchecked(x, y))
        }
    }

    fn deserialize_g2<R: Read>(reader: &mut R) -> IoResult<Self::G2Affine> {
        let f1 = Self::deserialize_field2(reader)?;
        let f2 = Self::deserialize_field2(reader)?;
        let infinity = f1.is_zero() && f2.is_zero();
        if infinity {
            Ok(Self::G2Affine::identity())
        } else {
            Ok(Self::G2Affine::new_unchecked(f1, f2))
        }
    }

    fn deserialize_g1_vec<R: Read>(reader: &mut R, n_vars: u32) -> IoResult<Vec<Self::G1Affine>> {
        (0..n_vars).map(|_| Self::deserialize_g1(reader)).collect()
    }

    fn deserialize_g2_vec<R: Read>(reader: &mut R, n_vars: u32) -> IoResult<Vec<Self::G2Affine>> {
        (0..n_vars).map(|_| Self::deserialize_g2(reader)).collect()
    }
}

impl FieldSerialization for Bls12_381 {
    type Fr = ark_bls12_381::Fr;
    type Fq = ark_bls12_381::Fq;
    type Fq2 = ark_bls12_381::Fq2;

    // need to divide by R, since snarkjs outputs the zkey with coefficients
    // multiplieid by R^2
    fn deserialize_field_fr<R: Read>(reader: &mut R) -> IoResult<Self::Fr> {
        let bigint = BigInteger256::deserialize_uncompressed(reader)?;
        Ok(Self::Fr::new_unchecked(
            Self::Fr::new_unchecked(bigint).into_bigint(),
        ))
    }

    // skips the multiplication by R because Circom points are already in Montgomery form
    fn deserialize_field<R: Read>(reader: &mut R) -> IoResult<Self::Fq> {
        let bigint = BigInteger384::deserialize_uncompressed(reader)?;
        // if you use Fq::new it multiplies by R
        Ok(Self::Fq::new_unchecked(bigint))
    }

    fn deserialize_field2<R: Read>(reader: &mut R) -> IoResult<Self::Fq2> {
        let c0 = Self::deserialize_field(reader)?;
        let c1 = Self::deserialize_field(reader)?;
        Ok(Self::Fq2::new(c0, c1))
    }

    fn deserialize_g1<R: Read>(reader: &mut R) -> IoResult<Self::G1Affine> {
        let x = Self::deserialize_field(reader)?;
        let y = Self::deserialize_field(reader)?;
        let infinity = x.is_zero() && y.is_zero();
        if infinity {
            Ok(Self::G1Affine::identity())
        } else {
            Ok(Self::G1Affine::new(x, y))
        }
    }

    fn deserialize_g2<R: Read>(reader: &mut R) -> IoResult<Self::G2Affine> {
        let f1 = Self::deserialize_field2(reader)?;
        let f2 = Self::deserialize_field2(reader)?;
        let infinity = f1.is_zero() && f2.is_zero();
        if infinity {
            Ok(Self::G2Affine::identity())
        } else {
            Ok(Self::G2Affine::new(f1, f2))
        }
    }

    fn deserialize_g1_vec<R: Read>(reader: &mut R, n_vars: u32) -> IoResult<Vec<Self::G1Affine>> {
        (0..n_vars).map(|_| Self::deserialize_g1(reader)).collect()
    }

    fn deserialize_g2_vec<R: Read>(reader: &mut R, n_vars: u32) -> IoResult<Vec<Self::G2Affine>> {
        (0..n_vars).map(|_| Self::deserialize_g2(reader)).collect()
    }
}

#[derive(Clone, Debug)]
struct Section {
    position: u64,
    #[allow(dead_code)]
    size: usize,
}

/// Reads a SnarkJS ZKey file into an Arkworks ProvingKey.
pub fn read_zkey<R: Read + Seek, P: Pairing + FieldSerialization>(
    reader: &mut R,
) -> IoResult<(ProvingKey<P>, ConstraintMatrices<P::Fr>)> {
    let mut binfile = BinFile::<R, P>::new(reader)?;
    let proving_key: ProvingKey<P> = binfile.proving_key()?;
    let matrices = binfile.matrices()?;
    Ok((proving_key, matrices))
}

/// Reads a SnarkJS ZKey file into an Arkworks ProvingKey.
pub fn read_proving_key<R: Read + Seek, P: Pairing + FieldSerialization>(
    reader: &mut R,
) -> IoResult<ProvingKey<P>> {
    let mut binfile = BinFile::<R, P>::new(reader)?;
    binfile.proving_key()
}

#[derive(Debug)]
struct BinFile<'a, R, P: Pairing + FieldSerialization> {
    #[allow(dead_code)]
    ftype: String,
    #[allow(dead_code)]
    version: u32,
    sections: HashMap<u32, Vec<Section>>,
    reader: &'a mut R,
    _p: PhantomData<P>,
}

impl<'a, R: Read + Seek, P: Pairing + FieldSerialization> BinFile<'a, R, P> {
    fn new(reader: &'a mut R) -> IoResult<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        let version = reader.read_u32::<LittleEndian>()?;

        let num_sections = reader.read_u32::<LittleEndian>()?;

        let mut sections = HashMap::new();
        for _ in 0..num_sections {
            let section_id = reader.read_u32::<LittleEndian>()?;
            let section_length = reader.read_u64::<LittleEndian>()?;

            let section = sections.entry(section_id).or_insert_with(Vec::new);
            section.push(Section {
                position: reader.stream_position()?,
                size: section_length as usize,
            });

            reader.seek(SeekFrom::Current(section_length as i64))?;
        }

        Ok(Self {
            ftype: std::str::from_utf8(&magic[..]).unwrap().to_string(),
            version,
            sections,
            reader,
            _p: PhantomData,
        })
    }

    fn proving_key(&mut self) -> IoResult<ProvingKey<P>> {
        let header = self.groth_header()?;
        let ic = self.ic(header.n_public)?;

        let a_query = self.a_query(header.n_vars)?;
        let b_g1_query = self.b_g1_query(header.n_vars)?;
        let b_g2_query = self.b_g2_query(header.n_vars)?;
        let l_query = self.l_query(header.n_vars - header.n_public - 1)?;
        let h_query = self.h_query(header.domain_size as usize)?;

        let vk = VerifyingKey::<P> {
            alpha_g1: header.verifying_key.alpha_g1,
            beta_g2: header.verifying_key.beta_g2,
            gamma_g2: header.verifying_key.gamma_g2,
            delta_g2: header.verifying_key.delta_g2,
            gamma_abc_g1: ic,
        };

        let pk = ProvingKey::<P> {
            vk,
            beta_g1: header.verifying_key.beta_g1,
            delta_g1: header.verifying_key.delta_g1,
            a_query,
            b_g1_query,
            b_g2_query,
            h_query,
            l_query,
        };

        Ok(pk)
    }

    fn get_section(&self, id: u32) -> Section {
        self.sections.get(&id).unwrap()[0].clone()
    }

    fn groth_header(&mut self) -> IoResult<HeaderGroth<P>> {
        let section = self.get_section(2);
        let header = HeaderGroth::new(&mut self.reader, &section)?;
        Ok(header)
    }

    fn ic(&mut self, n_public: usize) -> IoResult<Vec<P::G1Affine>> {
        // the range is non-inclusive so we do +1 to get all inputs
        self.g1_section(n_public + 1, 3)
    }

    /// Returns the [`ConstraintMatrices`] corresponding to the zkey
    pub fn matrices(&mut self) -> IoResult<ConstraintMatrices<P::Fr>> {
        let header = self.groth_header()?;

        let section = self.get_section(4);
        self.reader.seek(SeekFrom::Start(section.position))?;
        let num_coeffs: u32 = self.reader.read_u32::<LittleEndian>()?;

        // insantiate AB
        let mut matrices = vec![vec![vec![]; header.domain_size as usize]; 2];
        let mut max_constraint_index = 0;
        for _ in 0..num_coeffs {
            let matrix: u32 = self.reader.read_u32::<LittleEndian>()?;
            let constraint: u32 = self.reader.read_u32::<LittleEndian>()?;
            let signal: u32 = self.reader.read_u32::<LittleEndian>()?;

            let value: P::Fr = P::deserialize_field_fr(&mut self.reader)?;
            max_constraint_index = std::cmp::max(max_constraint_index, constraint);
            matrices[matrix as usize][constraint as usize].push((value, signal as usize));
        }

        let num_constraints = max_constraint_index as usize - header.n_public;
        // Remove the public input constraints, Arkworks adds them later
        matrices.iter_mut().for_each(|m| {
            m.truncate(num_constraints);
        });
        // This is taken from Arkworks' to_matrices() function
        let a = matrices[0].clone();
        let b = matrices[1].clone();
        let a_num_non_zero: usize = a.iter().map(|lc| lc.len()).sum();
        let b_num_non_zero: usize = b.iter().map(|lc| lc.len()).sum();
        let matrices = ConstraintMatrices {
            num_instance_variables: header.n_public + 1,
            num_witness_variables: header.n_vars - header.n_public,
            num_constraints,

            a_num_non_zero,
            b_num_non_zero,
            c_num_non_zero: 0,

            a,
            b,
            c: vec![],
        };

        Ok(matrices)
    }

    fn a_query(&mut self, n_vars: usize) -> IoResult<Vec<P::G1Affine>> {
        self.g1_section(n_vars, 5)
    }

    fn b_g1_query(&mut self, n_vars: usize) -> IoResult<Vec<P::G1Affine>> {
        self.g1_section(n_vars, 6)
    }

    fn b_g2_query(&mut self, n_vars: usize) -> IoResult<Vec<P::G2Affine>> {
        self.g2_section(n_vars, 7)
    }

    fn l_query(&mut self, n_vars: usize) -> IoResult<Vec<P::G1Affine>> {
        self.g1_section(n_vars, 8)
    }

    fn h_query(&mut self, n_vars: usize) -> IoResult<Vec<P::G1Affine>> {
        self.g1_section(n_vars, 9)
    }

    fn g1_section(&mut self, num: usize, section_id: usize) -> IoResult<Vec<P::G1Affine>> {
        let section = self.get_section(section_id as u32);
        self.reader.seek(SeekFrom::Start(section.position))?;
        P::deserialize_g1_vec(self.reader, num as u32)
    }

    fn g2_section(&mut self, num: usize, section_id: usize) -> IoResult<Vec<P::G2Affine>> {
        let section = self.get_section(section_id as u32);
        self.reader.seek(SeekFrom::Start(section.position))?;
        P::deserialize_g2_vec(self.reader, num as u32)
    }
}

#[derive(Default, Clone, Debug, CanonicalDeserialize)]
pub struct ZVerifyingKey<F: FieldSerialization> {
    alpha_g1: F::G1Affine,
    beta_g1: F::G1Affine,
    beta_g2: F::G2Affine,
    gamma_g2: F::G2Affine,
    delta_g1: F::G1Affine,
    delta_g2: F::G2Affine,
}

impl<F: FieldSerialization> ZVerifyingKey<F> {
    fn new<R: Read>(reader: &mut R) -> IoResult<Self> {
        let alpha_g1 = F::deserialize_g1(reader)?;
        let beta_g1 = F::deserialize_g1(reader)?;
        let beta_g2 = F::deserialize_g2(reader)?;
        let gamma_g2 = F::deserialize_g2(reader)?;
        let delta_g1 = F::deserialize_g1(reader)?;
        let delta_g2 = F::deserialize_g2(reader)?;

        Ok(Self {
            alpha_g1,
            beta_g1,
            beta_g2,
            gamma_g2,
            delta_g1,
            delta_g2,
        })
    }
}

#[derive(Clone, Debug)]
struct HeaderGroth<F: FieldSerialization> {
    #[allow(dead_code)]
    n8q: u32,
    #[allow(dead_code)]
    pub q: F::Fq,
    #[allow(dead_code)]
    n8r: u32,
    #[allow(dead_code)]
    pub r: F::Fr,

    n_vars: usize,
    n_public: usize,

    domain_size: u32,
    #[allow(dead_code)]
    power: u32,

    verifying_key: ZVerifyingKey<F>,
}

impl<P: Pairing + FieldSerialization> HeaderGroth<P> {
    fn new<R: Read + Seek>(reader: &mut R, section: &Section) -> IoResult<Self> {
        reader.seek(SeekFrom::Start(section.position))?;
        Self::read(reader)
    }

    fn read<R: Read>(mut reader: &mut R) -> IoResult<Self> {
        // TODO: Impl From<u32> in Arkworks
        let n8q: u32 = u32::deserialize_uncompressed(&mut reader)?;
        // group order r of Bn254
        let q = P::deserialize_field(&mut reader)?;

        let n8r: u32 = u32::deserialize_uncompressed(&mut reader)?;
        // Prime field modulus
        let r = P::deserialize_field_fr(&mut reader)?;

        let n_vars = u32::deserialize_uncompressed(&mut reader)? as usize;
        let n_public = u32::deserialize_uncompressed(&mut reader)? as usize;

        let domain_size: u32 = u32::deserialize_uncompressed(&mut reader)?;
        let power = log2(domain_size as usize);

        let verifying_key = ZVerifyingKey::<P>::new(&mut reader)?;

        Ok(Self {
            n8q,
            q,
            n8r,
            r,
            n_vars,
            n_public,
            domain_size,
            power,
            verifying_key,
        })
    }
}

pub struct ZkeyHeaderReader {
    zkey_path: std::path::PathBuf,
    offset: usize,
    data: Option<Vec<u8>>,
    pub n8q: u32,
    pub n8r: u32,
    n_public: u32,
    n_vars: u32,
    domain_size: u32,
    pub q: BigUint,
    pub r: BigUint,
}

// This implementation loads only the first few bytes
// of the zkey file to get the groth16 header.
//
// This header tells us what curve the zkey was built for.
// This is difficult to do in zkey.rs because we define the
// size if integers based on the type at the rust level, while
// zkeys specify their integer sizes in the file.
//
// e.g. we need to use the integer size specified in the zkey to
// determine what type to use in rust
impl ZkeyHeaderReader {
    pub fn new(zkey_path: &str) -> Self {
        ZkeyHeaderReader {
            zkey_path: std::path::PathBuf::from(zkey_path),
            offset: 0,
            data: None,
            n8q: 0,
            n8r: 0,
            n_public: 0,
            n_vars: 0,
            domain_size: 0,
            q: BigUint::zero(),
            r: BigUint::zero(),
        }
    }

    pub fn read(&mut self) {
        let mut file = File::open(self.zkey_path.clone()).unwrap();
        let mut zkey_bytes = vec![0; 512];
        // file.read_to_end(&mut zkey_bytes).unwrap();
        file.read_exact(&mut zkey_bytes).unwrap();
        self.data = Some(zkey_bytes);
        self.parse();
    }

    fn parse(&mut self) {
        let _magic = self.read_u32();
        let _version = self.read_u32();
        let num_sections = self.read_u32();
        for i in 0..num_sections {
            if i > 1 {
                return;
            }
            let section_id = self.read_u32();
            let section_len = self.read_u64();
            self.read_section(section_id, section_len);
        }
    }

    fn read_section(&mut self, section_n: u32, section_len: u64) {
        match section_n {
            1 => self.read_header(section_len),
            2 => self.read_groth16_header(section_len),
            // 3 => self.read_ic(section_len),
            // 4 => self.read_ccoefs(section_len),
            // 5 => self.read_a(section_len),
            // 6 => self.read_b1(section_len),
            // 7 => self.read_b2(section_len),
            // 8 => self.read_c(section_len),
            // 9 => self.read_h(section_len),
            // 10 => (|| {})(), // ignore reading the contributions
            _ => panic!("unknown section index"),
        }
    }

    fn read_u32(&mut self) -> u32 {
        let v = u32::from_le_bytes(
            self.data.as_ref().unwrap()[self.offset..(self.offset + 4)]
                .try_into()
                .unwrap(),
        );
        self.offset += 4;
        v
    }

    fn read_u64(&mut self) -> u64 {
        let v = u64::from_le_bytes(
            self.data.as_ref().unwrap()[self.offset..(self.offset + 8)]
                .try_into()
                .unwrap(),
        );
        self.offset += 8;
        v
    }

    fn read_bigint(&mut self, n8: u32) -> BigUint {
        let usize_n8 = usize::try_from(n8).unwrap();
        // convert an array of LE bytes to an array of LE 64 bit words
        let bytes = &self.data.as_ref().unwrap()[self.offset..(self.offset + usize_n8)];
        // let mut words_64 = [0_u64; 4];
        // for x in 0..4 {
        //     for y in 0..8 {
        //         words_64[x] += u64::from(bytes[x * 8 + y]) << (8 * y);
        //     }
        // }
        self.offset += usize_n8;
        BigUint::from_bytes_le(bytes)
        // BigInteger256::new(words_64)
    }

    // we start at the offset after the section length
    fn read_header(&mut self, _section_len: u64) {
        let key_type = self.read_u32();
        if key_type != 1 {
            panic!("non-groth16 zkey detected");
        }
    }

    fn read_groth16_header(&mut self, _section_len: u64) {
        self.n8q = self.read_u32();
        // read the q
        self.q = self.read_bigint(self.n8q);

        self.n8r = self.read_u32();
        // read the r
        self.r = self.read_bigint(self.n8r);

        self.n_vars = self.read_u32();
        self.n_public = self.read_u32();
        self.domain_size = self.read_u32();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254;
    use num_bigint::BigUint;
    use num_traits::{One, Zero};
    use serde_json::Value;
    use std::convert::TryFrom;
    use std::fs::File;
    use std::str::FromStr;

    fn fq_from_str(s: &str) -> ark_bn254::Fq {
        BigInteger256::try_from(BigUint::from_str(s).unwrap())
            .unwrap()
            .into()
    }

    // Circom snarkjs code:
    // console.log(curve.G1.F.one)
    fn fq_buf() -> Vec<u8> {
        vec![
            157, 13, 143, 197, 141, 67, 93, 211, 61, 11, 199, 245, 40, 235, 120, 10, 44, 70, 121,
            120, 111, 163, 110, 102, 47, 223, 7, 154, 193, 119, 10, 14,
        ]
    }

    // Circom snarkjs code:
    // const buff = new Uint8Array(curve.G1.F.n8*2);
    // curve.G1.toRprLEM(buff, 0, curve.G1.one);
    // console.dir( buff, { 'maxArrayLength': null })
    fn g1_buf() -> Vec<u8> {
        vec![
            157, 13, 143, 197, 141, 67, 93, 211, 61, 11, 199, 245, 40, 235, 120, 10, 44, 70, 121,
            120, 111, 163, 110, 102, 47, 223, 7, 154, 193, 119, 10, 14, 58, 27, 30, 139, 27, 135,
            186, 166, 123, 22, 142, 235, 81, 214, 241, 20, 88, 140, 242, 240, 222, 70, 221, 204,
            94, 190, 15, 52, 131, 239, 20, 28,
        ]
    }

    // Circom snarkjs code:
    // const buff = new Uint8Array(curve.G2.F.n8*2);
    // curve.G2.toRprLEM(buff, 0, curve.G2.one);
    // console.dir( buff, { 'maxArrayLength': null })
    fn g2_buf() -> Vec<u8> {
        vec![
            38, 32, 188, 2, 209, 181, 131, 142, 114, 1, 123, 73, 53, 25, 235, 220, 223, 26, 129,
            151, 71, 38, 184, 251, 59, 80, 150, 175, 65, 56, 87, 25, 64, 97, 76, 168, 125, 115,
            180, 175, 196, 216, 2, 88, 90, 221, 67, 96, 134, 47, 160, 82, 252, 80, 233, 9, 107,
            123, 234, 58, 131, 240, 254, 20, 246, 233, 107, 136, 157, 250, 157, 97, 120, 155, 158,
            245, 151, 210, 127, 254, 254, 125, 27, 35, 98, 26, 158, 255, 6, 66, 158, 174, 235, 126,
            253, 40, 238, 86, 24, 199, 86, 91, 9, 100, 187, 60, 125, 50, 34, 249, 87, 220, 118, 16,
            53, 51, 190, 53, 249, 85, 130, 100, 253, 147, 230, 160, 164, 13,
        ]
    }

    // Circom logs in Projective coordinates: console.log(curve.G1.one)
    fn g1_one() -> ark_bn254::G1Affine {
        let x = ark_bn254::Fq::one();
        let y = ark_bn254::Fq::one() + ark_bn254::Fq::one();
        let z = ark_bn254::Fq::one();
        ark_bn254::G1Affine::from(ark_bn254::G1Projective::new(x, y, z))
    }

    // Circom logs in Projective coordinates: console.log(curve.G2.one)
    fn g2_one() -> ark_bn254::G2Affine {
        let x = ark_bn254::Fq2::new(
            fq_from_str(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            ),
            fq_from_str(
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            ),
        );

        let y = ark_bn254::Fq2::new(
            fq_from_str(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            ),
            fq_from_str(
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            ),
        );
        let z = ark_bn254::Fq2::new(ark_bn254::Fq::one(), ark_bn254::Fq::zero());
        ark_bn254::G2Affine::from(ark_bn254::G2Projective::new(x, y, z))
    }

    #[test]
    fn can_deser_fq() {
        let buf = fq_buf();
        let fq = Bn254::deserialize_field(&mut &buf[..]).unwrap();
        assert_eq!(fq, ark_bn254::Fq::one());
    }

    #[test]
    fn can_deser_g1() {
        let buf = g1_buf();
        assert_eq!(buf.len(), 64);
        let g1 = Bn254::deserialize_g1(&mut &buf[..]).unwrap();
        let expected = g1_one();
        assert_eq!(g1, expected);
    }

    #[test]
    fn can_deser_g1_vec() {
        let n_vars = 10;
        let buf = vec![g1_buf(); n_vars]
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        let expected = vec![g1_one(); n_vars];

        let de = Bn254::deserialize_g1_vec(&mut &buf[..], n_vars as u32).unwrap();
        assert_eq!(expected, de);
    }

    #[test]
    fn can_deser_g2() {
        let buf = g2_buf();
        assert_eq!(buf.len(), 128);
        let g2 = Bn254::deserialize_g2(&mut &buf[..]).unwrap();

        let expected = g2_one();
        assert_eq!(g2, expected);
    }

    #[test]
    fn can_deser_g2_vec() {
        let n_vars = 10;
        let buf = vec![g2_buf(); n_vars]
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        let expected = vec![g2_one(); n_vars];

        let de = Bn254::deserialize_g2_vec(&mut &buf[..], n_vars as u32).unwrap();
        assert_eq!(expected, de);
    }

    #[test]
    fn header() {
        // `circom --r1cs` using the below file:
        //
        //  template Multiplier() {
        //     signal private input a;
        //     signal private input b;
        //     signal output c;
        //
        //     c <== a*b;
        // }
        //
        // component main = Multiplier();
        //
        // Then:
        // `snarkjs zkey new circuit.r1cs powersOfTau28_hez_final_10.ptau test.zkey`
        let path = "./test-vectors/test.zkey";
        let mut file = File::open(path).unwrap();
        let mut binfile = BinFile::<_, Bn254>::new(&mut file).unwrap();
        let header = binfile.groth_header().unwrap();
        assert_eq!(header.n_vars, 4);
        assert_eq!(header.n_public, 1);
        assert_eq!(header.domain_size, 4);
        assert_eq!(header.power, 2);
    }

    #[test]
    fn bls_header() {
        // `circom --r1cs` using the below file:
        //
        //  template Multiplier() {
        //     signal private input a;
        //     signal private input b;
        //     signal output c;
        //
        //     c <== a*b;
        // }
        //
        // component main = Multiplier();
        //
        // Then:
        // `snarkjs zkey new circuit.r1cs powersOfTau28_hez_final_10.ptau test.zkey`
        let path = "./test-vectors/multiplier2_bls.zkey";
        let mut file = File::open(path).unwrap();
        let mut binfile = BinFile::<_, Bls12_381>::new(&mut file).unwrap();
        let header = binfile.groth_header().unwrap();
        assert_eq!(header.n_vars, 4);
        assert_eq!(header.n_public, 1);
        assert_eq!(header.domain_size, 4);
        assert_eq!(header.power, 2);
    }

    #[test]
    fn deser_key() {
        let path = "./test-vectors/test.zkey";
        let mut file = File::open(path).unwrap();
        let (params, _matrices) = read_zkey::<_, Bn254>(&mut file).unwrap();

        // Check IC
        let expected = vec![
            Bn254::deserialize_g1(
                &mut &[
                    11, 205, 205, 176, 2, 105, 129, 243, 153, 58, 137, 89, 61, 95, 99, 161, 133,
                    201, 153, 192, 119, 19, 113, 136, 43, 105, 47, 206, 166, 55, 81, 22, 154, 77,
                    58, 119, 28, 230, 160, 206, 134, 98, 4, 115, 112, 184, 46, 117, 61, 180, 103,
                    138, 141, 202, 110, 252, 199, 252, 141, 211, 5, 46, 244, 10,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    118, 135, 198, 156, 63, 190, 210, 98, 194, 59, 169, 168, 204, 168, 76, 208,
                    109, 170, 24, 193, 57, 31, 184, 88, 234, 218, 118, 58, 107, 129, 90, 36, 230,
                    98, 62, 243, 3, 55, 68, 227, 117, 64, 188, 81, 81, 247, 161, 68, 68, 210, 142,
                    191, 174, 43, 110, 194, 253, 128, 217, 4, 54, 196, 111, 43,
                ][..],
            )
            .unwrap(),
        ];
        assert_eq!(expected, params.vk.gamma_abc_g1);

        // Check A Query
        let expected = vec![
            Bn254::deserialize_g1(
                &mut &[
                    240, 165, 110, 187, 72, 39, 218, 59, 128, 85, 50, 174, 229, 1, 86, 58, 125,
                    244, 145, 205, 248, 253, 120, 2, 165, 140, 154, 55, 220, 253, 14, 19, 212, 106,
                    59, 19, 125, 198, 202, 4, 59, 74, 14, 62, 20, 248, 219, 47, 234, 205, 54, 183,
                    33, 119, 165, 84, 46, 75, 39, 17, 229, 42, 192, 2,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    93, 53, 177, 82, 50, 5, 123, 116, 91, 35, 14, 196, 43, 180, 54, 15, 88, 144,
                    197, 105, 57, 167, 54, 5, 188, 109, 17, 89, 9, 223, 80, 1, 39, 193, 211, 168,
                    203, 119, 169, 105, 17, 156, 53, 106, 11, 102, 44, 92, 123, 220, 158, 240, 97,
                    253, 30, 121, 4, 236, 171, 23, 100, 34, 133, 11,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    177, 47, 21, 237, 244, 73, 76, 98, 80, 10, 10, 142, 80, 145, 40, 254, 100, 214,
                    103, 33, 38, 84, 238, 248, 252, 181, 75, 32, 109, 16, 93, 23, 135, 157, 206,
                    122, 107, 105, 202, 164, 197, 124, 242, 100, 70, 108, 9, 180, 224, 102, 250,
                    149, 130, 14, 133, 185, 132, 189, 193, 230, 180, 143, 156, 30,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ][..],
            )
            .unwrap(),
        ];
        assert_eq!(expected, params.a_query);

        // B G1 Query
        let expected = vec![
            Bn254::deserialize_g1(
                &mut &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    177, 47, 21, 237, 244, 73, 76, 98, 80, 10, 10, 142, 80, 145, 40, 254, 100, 214,
                    103, 33, 38, 84, 238, 248, 252, 181, 75, 32, 109, 16, 93, 23, 192, 95, 174, 93,
                    171, 34, 86, 151, 199, 77, 127, 3, 75, 254, 119, 227, 124, 241, 134, 235, 51,
                    55, 203, 254, 164, 226, 111, 250, 189, 190, 199, 17,
                ][..],
            )
            .unwrap(),
        ];
        assert_eq!(expected, params.b_g1_query);

        // B G2 Query
        let expected = vec![
            Bn254::deserialize_g2(
                &mut &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g2(
                &mut &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g2(
                &mut &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g2(
                &mut &[
                    240, 25, 157, 232, 164, 49, 152, 204, 244, 190, 178, 178, 29, 133, 205, 175,
                    172, 28, 12, 123, 139, 202, 196, 13, 67, 165, 204, 42, 74, 40, 6, 36, 112, 104,
                    61, 67, 107, 112, 72, 41, 213, 210, 249, 75, 89, 144, 144, 34, 177, 228, 18,
                    70, 80, 195, 124, 82, 40, 122, 91, 21, 198, 100, 154, 1, 16, 235, 41, 4, 176,
                    106, 9, 113, 141, 251, 100, 233, 188, 128, 194, 173, 0, 100, 206, 110, 53, 223,
                    163, 47, 166, 235, 25, 12, 151, 238, 45, 0, 78, 210, 56, 53, 57, 212, 67, 189,
                    253, 132, 62, 62, 116, 20, 235, 15, 245, 113, 30, 182, 33, 127, 203, 231, 124,
                    149, 74, 223, 39, 190, 217, 41,
                ][..],
            )
            .unwrap(),
        ];
        assert_eq!(expected, params.b_g2_query);

        // Check L Query
        let expected = vec![
            Bn254::deserialize_g1(
                &mut &[
                    146, 142, 29, 235, 9, 162, 84, 255, 6, 119, 86, 214, 154, 18, 12, 190, 202, 19,
                    168, 45, 29, 76, 174, 130, 6, 59, 146, 15, 229, 82, 81, 40, 50, 25, 124, 247,
                    129, 12, 147, 35, 108, 119, 178, 116, 238, 145, 33, 184, 74, 201, 128, 41, 151,
                    6, 60, 84, 156, 225, 200, 14, 240, 171, 128, 20,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    26, 32, 112, 226, 161, 84, 188, 236, 141, 226, 119, 169, 235, 218, 253, 176,
                    157, 184, 108, 243, 73, 122, 239, 217, 39, 190, 239, 105, 147, 190, 80, 47,
                    211, 68, 155, 212, 139, 173, 229, 160, 123, 117, 243, 110, 162, 188, 217, 206,
                    102, 19, 36, 189, 87, 183, 113, 8, 164, 133, 43, 142, 138, 109, 66, 33,
                ][..],
            )
            .unwrap(),
        ];
        assert_eq!(expected, params.l_query);

        // Check H Query
        let expected = vec![
            Bn254::deserialize_g1(
                &mut &[
                    21, 76, 104, 34, 28, 236, 135, 204, 218, 16, 160, 115, 185, 44, 19, 62, 43, 24,
                    57, 99, 207, 105, 10, 139, 195, 60, 17, 57, 85, 244, 167, 10, 166, 166, 165,
                    55, 38, 75, 116, 116, 182, 87, 217, 112, 28, 237, 239, 123, 231, 180, 122, 109,
                    77, 116, 88, 67, 102, 48, 80, 214, 137, 47, 94, 30,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    144, 175, 205, 119, 119, 192, 11, 10, 148, 224, 87, 161, 157, 231, 101, 208,
                    55, 15, 13, 16, 24, 59, 9, 22, 63, 215, 255, 30, 77, 188, 71, 37, 84, 227, 59,
                    29, 159, 116, 101, 93, 212, 220, 159, 141, 204, 107, 131, 87, 174, 149, 175,
                    72, 199, 109, 64, 109, 180, 150, 160, 249, 246, 33, 212, 29,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    129, 169, 52, 179, 66, 88, 123, 199, 222, 69, 24, 17, 219, 235, 118, 195, 156,
                    210, 14, 21, 76, 155, 178, 210, 223, 4, 233, 5, 8, 18, 156, 24, 82, 68, 183,
                    186, 7, 126, 2, 201, 207, 207, 74, 45, 44, 199, 16, 165, 25, 65, 157, 199, 90,
                    159, 12, 150, 250, 17, 177, 193, 244, 93, 230, 41,
                ][..],
            )
            .unwrap(),
            Bn254::deserialize_g1(
                &mut &[
                    207, 61, 229, 214, 21, 61, 103, 165, 93, 145, 54, 138, 143, 214, 5, 83, 183,
                    22, 174, 87, 108, 59, 99, 96, 19, 20, 25, 139, 114, 238, 198, 40, 182, 88, 1,
                    255, 206, 132, 156, 165, 178, 171, 0, 226, 179, 30, 192, 4, 79, 198, 69, 43,
                    145, 133, 116, 86, 36, 144, 190, 119, 79, 241, 76, 16,
                ][..],
            )
            .unwrap(),
        ];
        assert_eq!(expected, params.h_query);
    }

    #[test]
    fn deser_vk() {
        let path = "./test-vectors/test.zkey";
        let mut file = File::open(path).unwrap();
        let (params, _matrices) = read_zkey::<_, Bn254>(&mut file).unwrap();

        let json = std::fs::read_to_string("./test-vectors/verification_key.json").unwrap();
        let json: Value = serde_json::from_str(&json).unwrap();

        assert_eq!(json_to_g1(&json, "vk_alpha_1"), params.vk.alpha_g1);
        assert_eq!(json_to_g2(&json, "vk_beta_2"), params.vk.beta_g2);
        assert_eq!(json_to_g2(&json, "vk_gamma_2"), params.vk.gamma_g2);
        assert_eq!(json_to_g2(&json, "vk_delta_2"), params.vk.delta_g2);
        assert_eq!(json_to_g1_vec(&json, "IC"), params.vk.gamma_abc_g1);
    }

    fn json_to_g1(json: &Value, key: &str) -> ark_bn254::G1Affine {
        let els: Vec<String> = json
            .get(key)
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|i| i.as_str().unwrap().to_string())
            .collect();
        ark_bn254::G1Affine::from(ark_bn254::G1Projective::new(
            fq_from_str(&els[0]),
            fq_from_str(&els[1]),
            fq_from_str(&els[2]),
        ))
    }

    fn json_to_g1_vec(json: &Value, key: &str) -> Vec<ark_bn254::G1Affine> {
        let els: Vec<Vec<String>> = json
            .get(key)
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|i| {
                i.as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect::<Vec<String>>()
            })
            .collect();

        els.iter()
            .map(|coords| {
                ark_bn254::G1Affine::from(ark_bn254::G1Projective::new(
                    fq_from_str(&coords[0]),
                    fq_from_str(&coords[1]),
                    fq_from_str(&coords[2]),
                ))
            })
            .collect()
    }

    fn json_to_g2(json: &Value, key: &str) -> ark_bn254::G2Affine {
        let els: Vec<Vec<String>> = json
            .get(key)
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|i| {
                i.as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect::<Vec<String>>()
            })
            .collect();

        let x = ark_bn254::Fq2::new(fq_from_str(&els[0][0]), fq_from_str(&els[0][1]));
        let y = ark_bn254::Fq2::new(fq_from_str(&els[1][0]), fq_from_str(&els[1][1]));
        let z = ark_bn254::Fq2::new(fq_from_str(&els[2][0]), fq_from_str(&els[2][1]));
        ark_bn254::G2Affine::from(ark_bn254::G2Projective::new(x, y, z))
    }

    #[test]
    fn test_zkey_parse_bn() {
        let zkey_path = "./test-vectors/circom2_multiplier2.zkey".to_string();

        let mut zkey_reader = ZkeyHeaderReader::new(&zkey_path);
        zkey_reader.read();

        assert_eq!(zkey_reader.n8q, 32);
        assert_eq!(zkey_reader.n8r, 32);
        assert_eq!(zkey_reader.q, BigUint::from(ark_bn254::Fq::MODULUS));
        assert_eq!(zkey_reader.r, BigUint::from(ark_bn254::Fr::MODULUS));
    }

    #[test]
    fn test_zkey_parse_bls() {
        let zkey_path = "./test-vectors/multiplier2_bls.zkey".to_string();

        let mut zkey_reader = ZkeyHeaderReader::new(&zkey_path);
        zkey_reader.read();

        assert_eq!(zkey_reader.n8q, 48);
        assert_eq!(zkey_reader.n8r, 32);
        assert_eq!(zkey_reader.q, BigUint::from(ark_bls12_381::Fq::MODULUS));
        assert_eq!(zkey_reader.r, BigUint::from(ark_bls12_381::Fr::MODULUS));
    }
}
