use ark_bls12_377_3;
use ark_ff_3::{fields::Field, BigInteger, PrimeField};
use ark_serialize_3::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::{
    rand::{Rng, RngCore},
    Zero,
};
use std::collections::VecDeque;
use std::fs::File;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HarnessError {
    #[error("could not serialize")]
    SerializationError(#[from] SerializationError),

    #[error("could not open file")]
    FileOpenError(#[from] std::io::Error),

    #[error("failed to read at least one instance from file")]
    DeserializationError,
}

pub enum FileInputIteratorMode {
    Checked,
    Unchecked,
}

pub struct FileInputIterator {
    points_file: File,
    scalars_file: File,
    mode: FileInputIteratorMode,
    cached: Option<Instance>,
}

pub type Point = ark_bls12_377_3::G1Affine;
pub type Scalar = <ark_bls12_377_3::Fr as PrimeField>::BigInt;
pub type Instance = (Vec<Point>, Vec<Scalar>);

const INSTANCE_SIZE: u32 = 16;
const NUM_INSTANCE: u32 = 10;
const PATH: &str = "mopro-core/src/middleware/gpu_explorations/utils";

impl FileInputIterator {
    pub fn open(dir: &str) -> Result<Self, HarnessError> {
        let points_path = format!("{}{}", dir, "/points");
        let scalars_path = format!("{}{}", dir, "/scalars");

        // Try to read an instance, first in uncheck, then check serialization modes.
        let mut iter = Self {
            points_file: File::open(&points_path)?,
            scalars_file: File::open(&scalars_path)?,
            mode: FileInputIteratorMode::Unchecked,
            cached: None,
        };

        // Read a first value and see if we get a result.
        iter.cached = iter.next();
        if iter.cached.is_some() {
            return Ok(iter);
        }

        let mut iter = Self {
            points_file: File::open(&points_path)?,
            scalars_file: File::open(&scalars_path)?,
            mode: FileInputIteratorMode::Checked,
            cached: None,
        };
        iter.cached = iter.next();
        if iter.cached.is_none() {
            return Err(HarnessError::DeserializationError);
        }
        Ok(iter)
    }
}

impl Iterator for FileInputIterator {
    type Item = Instance;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cached.is_some() {
            return self.cached.take();
        }

        let points = match self.mode {
            FileInputIteratorMode::Checked => Vec::<Point>::deserialize(&self.points_file),
            FileInputIteratorMode::Unchecked => {
                Vec::<Point>::deserialize_unchecked(&self.points_file)
            }
        };

        let points = match points {
            Ok(x) => Some(x),
            Err(_) => None,
        }?;

        let scalars = Vec::<Scalar>::deserialize(&self.scalars_file);
        let scalars = match scalars {
            Ok(x) => Some(x),
            Err(_) => None,
        }?;

        Some((points, scalars))
    }
}

pub struct VectorInputIterator {
    points: VecDeque<Vec<Point>>,
    scalars: VecDeque<Vec<Scalar>>,
}

impl Iterator for VectorInputIterator {
    type Item = Instance;

    fn next(&mut self) -> Option<Self::Item> {
        let points = self.points.pop_front()?;
        let scalars = self.scalars.pop_front()?;
        Some((points, scalars))
    }
}

impl From<Instance> for VectorInputIterator {
    fn from(other: Instance) -> Self {
        Self {
            points: vec![other.0].into(),
            scalars: vec![other.1].into(),
        }
    }
}

impl From<(Vec<Vec<Point>>, Vec<Vec<Scalar>>)> for VectorInputIterator {
    fn from(other: (Vec<Vec<Point>>, Vec<Vec<Scalar>>)) -> Self {
        Self {
            points: other.0.into(),
            scalars: other.1.into(),
        }
    }
}

fn gen_random_vectors<R: RngCore>(instance_size: u32, rng: &mut R) -> Instance {
    let num_bytes = ark_bls12_377_3::Fr::zero();
    let mut points = Vec::<Point>::new();
    let mut scalars = Vec::<Scalar>::new();
    let mut bytes = vec![0; instance_size as usize];
    let mut scalar;

    // Generate instances with each having instance_size points and scalars
    for _i in 0..instance_size {
        loop {
            rng.fill_bytes(&mut bytes[..]);
            scalar = ark_bls12_377_3::Fr::from_random_bytes(&bytes);
            if scalar.is_some() {
                break;
            }
        }
        scalars.push(scalar.unwrap().into_repr());

        let point: ark_bls12_377_3::G1Projective = rng.gen();
        points.push(point.into());
    }
    (points, scalars)
}

pub fn gen_vectors(instance_size: u32, num_instance: u32, dir: &str) {
    let mut rng = ark_std::rand::thread_rng();
    println!("Generating elements");
    let instance_size = 1 << instance_size;
    for i in 0..num_instance {
        let (points, scalars) = gen_random_vectors(instance_size, &mut rng);
        println!("Generated {}th instance", i);
        serialize_input(dir, &points, &scalars, i != 0).unwrap();
    }
    println!("Generated elements");
}

pub fn serialize_input(
    dir: &str,
    points: &[Point],
    scalars: &[Scalar],
    append: bool,
) -> Result<(), HarnessError> {
    // Check if dir exists
    std::fs::create_dir_all(dir).expect("Should create directory if it doesn't exist");

    let points_path = format!("{}{}", dir, "/points");
    let scalars_path = format!("{}{}", dir, "/scalars");

    let (f1, f2) = if append {
        let file1 = File::options()
            .append(true)
            .create(true)
            .open(points_path)?;
        let file2 = File::options()
            .append(true)
            .create(true)
            .open(scalars_path)?;
        (file1, file2)
    } else {
        let file1 = File::create(points_path)?;
        let file2 = File::create(scalars_path)?;
        (file1, file2)
    };
    points.serialize_unchecked(&f1).unwrap();
    scalars.serialize_unchecked(&f2).unwrap();
    Ok(())
}

pub fn deserialize_input(dir: &str) -> Result<(Vec<Vec<Point>>, Vec<Vec<Scalar>>), HarnessError> {
    let mut points_result = Vec::new();
    let mut scalars_result = Vec::new();
    let points_path = format!("{}{}", dir, "/points");
    let scalars_path = format!("{}{}", dir, "/scalars");
    let f1 = File::open(points_path)?;
    let f2 = File::open(scalars_path)?;

    loop {
        let points = Vec::<Point>::deserialize_unchecked(&f1);
        let scalars = Vec::<Scalar>::deserialize_unchecked(&f2);

        let points = match points {
            Ok(x) => x,
            _ => {
                break;
            }
        };

        let scalars = match scalars {
            Ok(x) => x,
            _ => {
                break;
            }
        };

        points_result.push(points);
        scalars_result.push(scalars);
    }

    Ok((points_result, scalars_result))
}

mod tests {
    use super::*;

    #[test]
    fn test_gen_random_vector() {
        let mut rng = ark_std::rand::thread_rng();
        let instance_size = 1 << INSTANCE_SIZE;
        let (points, scalars) = gen_random_vectors(instance_size, &mut rng);

        assert_eq!(points.len() as u32, instance_size);
        assert_eq!(scalars.len() as u32, instance_size);
        // assert_eq!(scalars[0], NUM_INSTANCE);
    }

    #[test]
    fn test_gen_vectors() {
        let dir = format!("{}/vectors/{}x{}", PATH, INSTANCE_SIZE, NUM_INSTANCE);
        gen_vectors(INSTANCE_SIZE, NUM_INSTANCE, &dir);
    }
}
