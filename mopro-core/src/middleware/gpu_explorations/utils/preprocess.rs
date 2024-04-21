// use ark_bls12_377_3;
use ark_bn254::{Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ff::{Field, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::{
    rand::{Rng, RngCore},
    UniformRand,
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

pub type Point = GAffine;
pub type Scalar = <ScalarField as PrimeField>::BigInt;
pub type Instance = (Vec<Point>, Vec<Scalar>);

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
            FileInputIteratorMode::Checked => {
                Vec::<Point>::deserialize_compressed(&self.points_file)
            }
            FileInputIteratorMode::Unchecked => {
                Vec::<Point>::deserialize_compressed_unchecked(&self.points_file)
            }
        };

        let points = match points {
            Ok(x) => Some(x),
            Err(_) => None,
        }?;

        let scalars = Vec::<Scalar>::deserialize_compressed(&self.scalars_file);
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
    let mut points = Vec::<Point>::new();
    let mut scalars = Vec::<Scalar>::new();

    // Generate instances with each having instance_size points and scalars
    for _i in 0..instance_size {
        let scalar = ScalarField::rand(rng); // the size of scalar is at most 32 Bytes
        scalars.push(scalar.into());

        let point = GAffine::rand(rng);
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
    points.serialize_compressed(&f1).unwrap();
    scalars.serialize_compressed(&f2).unwrap();
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
        let points = Vec::<Point>::deserialize_compressed_unchecked(&f1);
        let scalars = Vec::<Scalar>::deserialize_compressed_unchecked(&f2);

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

pub fn get_root_path() -> String {
    let current_dir = std::env::current_dir().unwrap();
    let mut current_dir = current_dir;
    loop {
        if current_dir.ends_with("mopro") {
            break;
        }
        current_dir = current_dir.parent().unwrap().to_path_buf();
    }
    current_dir.display().to_string()
}

mod tests {
    use super::*;

    const INSTANCE_SIZE: u32 = 16;
    const NUM_INSTANCE: u32 = 10;
    const PATH: &str = "mopro-core/src/middleware/gpu_explorations/utils";

    #[test]
    fn test_gen_random_vector() {
        let mut rng = ark_std::rand::thread_rng();
        let instance_size = 1 << INSTANCE_SIZE;
        let (points, scalars) = gen_random_vectors(instance_size, &mut rng);

        assert_eq!(points.len() as u32, instance_size);
        assert_eq!(scalars.len() as u32, instance_size);
        assert_eq!(scalars[0].compressed_size(), 32); // scalar size is 32 Bytes
    }

    #[test]
    fn test_gen_vectors() {
        let dir = format!(
            "{}/{}/vectors/{}x{}",
            get_root_path(),
            PATH,
            INSTANCE_SIZE,
            NUM_INSTANCE
        );
        gen_vectors(INSTANCE_SIZE, NUM_INSTANCE, &dir);
    }
}
