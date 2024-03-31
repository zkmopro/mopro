use ark_bn254::g1::G1Affine;
use ark_bn254::{Fr as ScalarField, FrConfig, G1Affine as GAffine, G1Projective as G};
use ark_ec::{scalar_mul, AffineRepr, ScalarMul, VariableBaseMSM};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::{Rng, RngCore};
use ark_std::{error::Error, UniformRand};

// New

use std::collections::VecDeque;
use std::fs::File;
use std::result;
use thiserror::Error;

// For benchmarking
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub num_msm: u32,
    pub avg_processing_time: f64,
    pub total_processing_time: f64,
}

const INSTANCE_SIZE: usize = 16;
const NUM_INSTANCES: usize = 10;
const PATH: &str = "src/middleware/gpu_explorations/trapdoorTech_zprize_msm";

type Point = ark_bn254::G1Affine;
type Scalar = ScalarField;
type Instance = (Vec<Point>, Vec<Scalar>);

#[derive(Debug, Error)]
pub enum HarnessError {
    #[error("could not serialize")]
    SerializationError(#[from] ark_serialize::SerializationError),

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

// TODO: refactor fn name and add more benchmarks in the future
fn single_msm() -> Result<(), Box<dyn Error>> {
    let mut rng = ark_std::test_rng();

    /*
    // We use the BN254 curve to match Groth16 proving system
    let a = GAffine::rand(&mut rng);
    let b = GAffine::rand(&mut rng);
    let s1 = ScalarField::rand(&mut rng);
    let s2 = ScalarField::rand(&mut rng);

    println!("a len: {:?}", a.compressed_size());
    println!("b len: {:?}", b.compressed_size());
    println!("s1 len: {:?}", s1.0.0.len());
    println!("s2 len: {:?}", s2.0.0.len());
    */

    // let num_bytes = ark_bn254::G1Affine::zero().compressed_size();
    // let mut points = Vec::<ark_bn254::G1Affine>::new();
    // let mut scalars = Vec::<ScalarField>::new();
    // let mut bytes = vec![0; num_bytes];
    // let mut scalar ;
    // let num_elements = 1 << INSTANCE_SIZE;
    // for _i in 0..num_elements {
    //     // scalar = ark_bn254::G1Affine::from_random_bytes(&bytes);
    //     scalar = ScalarField::rand(&mut rng);
    //     // println!("scalar: {:?}", scalar);
    //     scalars.push(scalar);

    //     let point: ark_bn254::G1Projective = rng.gen();
    //     points.push(point.into());
    // }
    // let result = G::msm(&points, &scalars);

    // TODO: read point and scalar from file then perform msm
    let dir = format!("{}/vectors/{}x{}", PATH, NUM_INSTANCES, INSTANCE_SIZE);
    // let input_iter = FileInputIterator::open(&dir).unwrap();

    let points_path = format!("{}{}", dir, "/points");
    let scalars_path = format!("{}{}", dir, "/scalars");
    let points_file = File::open(&points_path)?;
    let scalars_file = File::open(&scalars_path)?;
    let mut points_result = Vec::new();
    let mut scalars_result = Vec::new();

    loop {
        let points = Vec::<Point>::deserialize_uncompressed_unchecked(&points_file);
        let scalars = Vec::<Scalar>::deserialize_uncompressed_unchecked(&scalars_file);

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

    println!("points: {:?}", points_result.len());
    println!("scalars: {:?}", scalars_result.len());

    // let result = G::msm(&points, &scalars);

    // for instance in input_iter {
    //     let point = &instance.0;
    //     let scalar = &instance.1;

    //     let result = G::msm(point, scalar);
    // }
    // let points = &input_iter.points_file;
    // let scalars = &input_iter.scalars_file;
    // println!("points: {:?}", points);
    // println!("scalars: {:?}", scalars);

    // let r = G::msm(&[a, b], &[s1, s2]).unwrap();
    // assert_eq!(r, a * s1 + b * s2);
    Ok(())
}

// TODO: figure out a way to configure the algorithm fn used
// Run the msm benchmark with timing
pub fn run_msm_benchmark(num_msm: Option<u32>) -> Result<BenchmarkResult, Box<dyn Error>> {
    let num_msm = num_msm.unwrap_or(1000); // default to 1000 msm operations

    let mut total_msm = Duration::new(0, 0);
    for _ in 0..num_msm {
        let start = Instant::now();
        single_msm()?;
        total_msm += start.elapsed();
    }

    let msm_avg = (total_msm.as_secs_f64() / num_msm as f64) * 1_000.0; // in ms

    Ok(BenchmarkResult {
        num_msm,
        avg_processing_time: msm_avg,
        total_processing_time: total_msm.as_secs_f64() * 1_000.0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_msm() {
        assert!(single_msm().is_ok());
    }

    #[test]
    fn test_run_msm_benchmark() {
        let benchmarks = run_msm_benchmark(None).unwrap();
        println!("\nBenchmarking {:?} msm on BN254 curve", benchmarks.num_msm);
        println!(
            "└─ Average msm time: {:.5} ms\n└─ Overall processing time: {:.5} ms",
            benchmarks.avg_processing_time, benchmarks.total_processing_time
        );
    }
}
