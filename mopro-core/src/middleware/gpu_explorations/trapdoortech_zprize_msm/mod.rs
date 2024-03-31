mod local_msm;

use ark_bls12_377::G1Affine;
use ark_ec_3::AffineCurve;
use ark_serialize_3::Write;
use ark_std::{One, Zero};
use local_msm::{
    edwards_from_neg_one_a, edwards_proj_to_affine, edwards_to_neg_one_a, edwards_to_sw,
    multi_scalar_mul, sw_to_edwards, EdwardsAffine, ExEdwardsAffine,
};
use std::fs::File;
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::middleware::gpu_explorations::utils::preprocess;

const INSTANCE_SIZE: usize = 16;
const NUM_INSTANCES: usize = 10;
const UTILSPATH: &str = "src/middleware/gpu_explorations/utils";
const BENCHMARKSPATH: &str = "benchmarks/gpu_explorations";

pub fn benchmark_msm<I>(
    output_dir: &str,
    instances: I,
    iterations: u32,
) -> Result<Vec<Duration>, preprocess::HarnessError>
where
    I: Iterator<Item = preprocess::Instance>,
{
    let output_path = format!("{}{}", output_dir, "/trapdoor_benchmark_time.txt");
    let mut output_file = File::create(output_path).expect("output file creation failed");
    let mut result_vec = Vec::new();

    let mut ed_instances = vec![];
    for instance in instances {
        let points = &instance.0;

        let mut ed_points = Vec::<ExEdwardsAffine>::new();
        for p in points {
            let ed_p = sw_to_edwards(EdwardsAffine {
                x: p.x.clone(),
                y: p.y.clone(),
            });
            let ed_p = edwards_to_neg_one_a(ed_p);
            ed_points.push(ed_p);
        }
        ed_instances.push((ed_points, instance.1));
    }

    for instance in ed_instances {
        let points = &instance.0;
        let scalars = &instance.1;

        let mut total_duration = Duration::ZERO;
        for i in 0..iterations {
            let start = Instant::now();
            let result = multi_scalar_mul(&points[..], &scalars[..]);
            let result = edwards_from_neg_one_a(edwards_proj_to_affine(result));
            let result = edwards_to_sw(result);

            let result = if result.x == <G1Affine as AffineCurve>::BaseField::zero()
                && result.y == <G1Affine as AffineCurve>::BaseField::one()
            {
                G1Affine::new(result.x, result.y, true)
            } else {
                G1Affine::new(result.x, result.y, false)
            };

            let time = start.elapsed();
            writeln!(output_file, "iteration {}: {:?}", i + 1, time)?;
            total_duration += time;
        }
        let mean = total_duration / iterations;
        writeln!(output_file, "Mean across all iterations: {:?}", mean)?;
        println!(
            "Average time to execute MSM with {} points and {} scalars and {} iterations is: {:?}",
            points.len(),
            scalars.len(),
            iterations,
            mean
        );
        result_vec.push(mean);
    }
    Ok(result_vec)
}

pub fn run_benchmark(dir: &str) {
    let input_iter = preprocess::FileInputIterator::open(dir).unwrap();
    let result = benchmark_msm(dir, input_iter, 1);
    println!("Done running benchmark: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_msm() {
        // Run this benchmark after the points and scalars have been generated
        let dir = format!("{}/vectors/{}x{}", UTILSPATH, NUM_INSTANCES, INSTANCE_SIZE);

        // Check if the vectors have been generated
        match preprocess::FileInputIterator::open(&dir) {
            Ok(_) => {
                println!("Vectors already generated");
            }
            Err(_) => {
                preprocess::gen_vectors(&dir);
            }
        }

        println!("Running benchmark for baseline result");
        let input_iter = preprocess::FileInputIterator::open(&dir).unwrap();
        let result = benchmark_msm(&BENCHMARKSPATH, input_iter, 1);
        println!("Done running benchmark: {:?}", result);
    }
}
