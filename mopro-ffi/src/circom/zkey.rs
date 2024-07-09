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
    io::{Read, Seek, SeekFrom},
    marker::PhantomData,
};

use ark_bn254::Bn254;
use ark_groth16::{ProvingKey, VerifyingKey};
use num_traits::Zero;

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

#[derive(Debug)]
pub struct BinFile<'a, R, P: Pairing + FieldSerialization> {
    #[allow(dead_code)]
    ftype: String,
    #[allow(dead_code)]
    version: u32,
    sections: HashMap<u32, Vec<Section>>,
    reader: &'a mut R,
    _p: PhantomData<P>,
}

pub struct HeaderGroth<F: FieldSerialization> {
    #[allow(dead_code)]
    pub n8q: u32,
    #[allow(dead_code)]
    pub q: F::Fq,
    #[allow(dead_code)]
    pub n8r: u32,
    #[allow(dead_code)]
    pub r: F::Fr,

    n_vars: usize,
    n_public: usize,

    domain_size: u32,
    #[allow(dead_code)]
    power: u32,

    alpha_g1: F::G1Affine,
    beta_g1: F::G1Affine,
    beta_g2: F::G2Affine,
    gamma_g2: F::G2Affine,
    delta_g1: F::G1Affine,
    delta_g2: F::G2Affine,
}

impl<'a, R: Read + Seek, P: Pairing + FieldSerialization> BinFile<'a, R, P> {
    pub fn new(reader: &'a mut R) -> IoResult<Self> {
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

    pub fn proving_key(&mut self) -> IoResult<ProvingKey<P>> {
        let header = self.groth_header()?;
        let ic = self.ic(header.n_public)?;

        let a_query = self.a_query(header.n_vars)?;
        let b_g1_query = self.b_g1_query(header.n_vars)?;
        let b_g2_query = self.b_g2_query(header.n_vars)?;
        let l_query = self.l_query(header.n_vars - header.n_public - 1)?;
        let h_query = self.h_query(header.domain_size as usize)?;

        let vk = VerifyingKey::<P> {
            alpha_g1: header.alpha_g1,
            beta_g2: header.beta_g2,
            gamma_g2: header.gamma_g2,
            delta_g2: header.delta_g2,
            gamma_abc_g1: ic,
        };

        let pk = ProvingKey::<P> {
            vk,
            beta_g1: header.beta_g1,
            delta_g1: header.delta_g1,
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

    pub fn groth_header(&mut self) -> IoResult<HeaderGroth<P>> {
        let section = self.get_section(2);
        self.reader.seek(SeekFrom::Start(section.position))?;

        // TODO: Impl From<u32> in Arkworks
        // Number of bytes per q element
        let n8q: u32 = u32::deserialize_uncompressed_unchecked(&mut self.reader)?;
        let q = P::deserialize_field(&mut self.reader)?;

        // Number of bytes per r element
        let n8r: u32 = u32::deserialize_uncompressed_unchecked(&mut self.reader)?;
        // Prime field modulus
        let r = P::deserialize_field_fr(&mut self.reader)?;

        let n_vars = u32::deserialize_uncompressed_unchecked(&mut self.reader)? as usize;
        let n_public = u32::deserialize_uncompressed_unchecked(&mut self.reader)? as usize;

        let domain_size: u32 = u32::deserialize_uncompressed_unchecked(&mut self.reader)?;
        let power = log2(domain_size as usize);

        let alpha_g1 = P::deserialize_g1(&mut self.reader)?;
        let beta_g1 = P::deserialize_g1(&mut self.reader)?;
        let beta_g2 = P::deserialize_g2(&mut self.reader)?;
        let gamma_g2 = P::deserialize_g2(&mut self.reader)?;
        let delta_g1 = P::deserialize_g1(&mut self.reader)?;
        let delta_g2 = P::deserialize_g2(&mut self.reader)?;

        Ok(HeaderGroth {
            n8q,
            q,
            n8r,
            r,
            n_vars,
            n_public,
            domain_size,
            power,
            alpha_g1,
            beta_g1,
            beta_g2,
            gamma_g2,
            delta_g1,
            delta_g2,
        })
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
