use ark_bls12_377::{Fr as ScalarField, G1Affine, G1Projective};
use ark_bls12_377_3;
use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ff::{BigInt, Field, PrimeField};
// use ark_bn254::g1::G1Affine;
// use ark_bn254::{Fr as ScalarField, FrConfig, G1Affine as GAffine, G1Projective as G};
use ark_ec::{AffineRepr, CurveGroup, VariableBaseMSM};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Write};
use ark_std::rand::{Rng, RngCore};
use ark_std::{error::Error, UniformRand, Zero};

use crate::middleware::gpu_explorations::utils::preprocess;

use std::fs::File;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub num_msm: u32,
    pub avg_processing_time: f64,
    pub total_processing_time: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MyAffine<P: SWCurveConfig>(pub Affine<P>);

impl<P: SWCurveConfig> From<([u8; 32], [u8; 32])> for MyAffine<P> {
    fn from(bytes: ([u8; 32], [u8; 32])) -> Self {
        // Your conversion logic here
        // Let's say you somehow convert bytes to x and y...
        let x = P::BaseField::from_random_bytes(&bytes.0).unwrap();
        let y = P::BaseField::from_random_bytes(&bytes.1).unwrap();

        MyAffine(Affine::new(x, y))
    }
}

pub fn benchmark_msm<I>(
    benchmark_dir: &str,
    instances: I,
    iterations: u32,
) -> Result<Vec<Duration>, preprocess::HarnessError>
where
    I: Iterator<Item = preprocess::Instance>,
{
    let output_path = format!("{}{}", benchmark_dir, "/arkworks_pippenger_time.txt");
    let mut output_file = File::create(output_path).expect("output file creation failed");
    let mut result_vec = Vec::new();

    for instance in instances {
        let points_slice: Vec<G1Affine> = instance
            .0
            .iter()
            .map(|&p| {
                let mut bytes = Vec::new();
                <SWCurveConfig as Into<MyAffine>>::into(p)
                    .serialize_uncompressed(&mut bytes)
                    .unwrap(); // use custom from implementation
                               // p.into().serialize_uncompressed(&mut bytes).unwrap();   // use custom from implementation
                println!("p: {:?}", p.to_string());
                G1Affine::deserialize_uncompressed_unchecked(&*bytes).unwrap()
            })
            .collect();

        let scalars_slice: Vec<ScalarField> = instance
            .1
            .iter()
            .map(|&s| {
                let mut bytes = Vec::new();
                s.to_string().serialize_uncompressed(&mut bytes).unwrap();

                ScalarField::deserialize_uncompressed_unchecked(&*bytes).unwrap()
            })
            .collect();

        let mut total_duration = Duration::ZERO;
        for i in 0..iterations {
            let start = Instant::now();
            let result = <G1Projective as VariableBaseMSM>::msm(&points_slice, &scalars_slice);
            total_duration += start.elapsed();
        }

        let avg_duration = total_duration / iterations;
        writeln!(
            output_file,
            "Average duration across all iterations: {:?}",
            avg_duration
        )?;
        println!(
            "Average time to execute MSM with {} points and {} scalars and {} iterations is: {:?}",
            points_slice.len(),
            scalars_slice.len(),
            iterations,
            avg_duration
        );
        result_vec.push(avg_duration);
    }
    Ok(result_vec)
}

pub fn run_benchmark(dir: &str) {
    println!("dir: {:?}", dir);
    let benchmark_data = preprocess::FileInputIterator::open(dir).unwrap();
    let result = benchmark_msm("benchmarks/gpu_explorations", benchmark_data, 1);
    println!("Done running benchmark: {:?}", result);
}

// Run the msm benchmark with timing
// pub fn run_benchmark(dir: &str) -> Result<BenchmarkResult, Box<dyn Error>> {
//     let num_msm = num_msm.unwrap_or(1000); // default to 1000 msm operations

//     let mut total_msm = Duration::new(0, 0);
//     for _ in 0..num_msm {
//         let start = Instant::now();
//         benchmark_msm()?;
//         total_msm += start.elapsed();
//     }

//     let msm_avg = (total_msm.as_secs_f64() / num_msm as f64) * 1_000.0; // in ms

//     Ok(BenchmarkResult {
//         num_msm,
//         avg_processing_time: msm_avg,
//         total_processing_time: total_msm.as_secs_f64() * 1_000.0,
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_benchmark() {
        const INSTANCE_SIZE: usize = 16;
        const NUM_INSTANCES: usize = 10;
        const PATH: &str = "src/middleware/gpu_explorations/utils";
        let dir = format!("{}/vectors/{}x{}", PATH, NUM_INSTANCES, INSTANCE_SIZE);
        run_benchmark(&dir);
    }

    // #[test]
    // fn test_single_msm() {
    //     assert!(benchmark_msm().is_ok());
    // }

    // #[test]
    // fn test_run_msm_benchmark() {
    //     let benchmarks = run_msm_benchmark(None).unwrap();
    //     println!("\nBenchmarking {:?} msm on BN254 curve", benchmarks.num_msm);
    //     println!(
    //         "└─ Average msm time: {:.5} ms\n└─ Overall processing time: {:.5} ms",
    //         benchmarks.avg_processing_time, benchmarks.total_processing_time
    //     );
    // }
}
