mod local_msm;

use ark_bls12_377_3::G1Affine;
use ark_ec_3::AffineCurve;
use ark_ff::BigInt;
use ark_serialize_3::Write;
use ark_std::{One, Zero};
use local_msm::{
    edwards_from_neg_one_a, edwards_proj_to_affine, edwards_to_neg_one_a, edwards_to_sw,
    multi_scalar_mul, sw_to_edwards, EdwardsAffine, ExEdwardsAffine,
};
use std::fs::File;
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::middleware::gpu_explorations::utils::{benchmark::BenchmarkResult, preprocess};

pub fn benchmark_msm<I>(
    benchmark_dir: &str,
    instances: I,
    iterations: u32,
) -> Result<Vec<Duration>, preprocess::HarnessError>
where
    I: Iterator<Item = preprocess::Instance>,
{
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

            total_duration += start.elapsed();
        }
        let avg_duration = total_duration / iterations;

        println!(
            "Average time to execute MSM with {} points and scalars in {} iterations is: {:?}",
            points.len(),
            iterations,
            avg_duration
        );
        result_vec.push(avg_duration);
    }
    Ok(result_vec)
}

pub fn run_benchmark(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
    benchmark_dir: &str,
) -> Result<BenchmarkResult, preprocess::HarnessError> {
    let benchmark_data_dir = format!("{}/vectors/{}x{}", &utils_dir, instance_size, num_instance);

    // Check if the vectors have been generated
    match preprocess::FileInputIterator::open(&benchmark_data_dir) {
        Ok(_) => {
            println!("Vectors already generated");
        }
        Err(_) => {
            preprocess::gen_vectors(instance_size, num_instance, &benchmark_data_dir);
        }
    }

    let benchmark_data = preprocess::FileInputIterator::open(&benchmark_data_dir).unwrap();
    let instance_durations = benchmark_msm(benchmark_dir, benchmark_data, 1).unwrap();
    // in milliseconds
    let avg_processing_time: f64 = instance_durations
        .iter()
        .map(|d| d.as_secs_f64() * 1000.0)
        .sum::<f64>()
        / instance_durations.len() as f64;

    println!(
        "Done running benchmark. Check the result at: mopro-core/{:?}",
        benchmark_dir
    );
    Ok(BenchmarkResult {
        instance_size: instance_size,
        num_instance: num_instance,
        avg_processing_time: avg_processing_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INSTANCE_SIZE: u32 = 16;
    const NUM_INSTANCE: u32 = 10;
    const UTILSPATH: &str = "src/middleware/gpu_explorations/utils";
    const BENCHMARKSPATH: &str = "benchmarks/gpu_explorations";

    #[test]
    fn test_benchmark_msm() {
        let dir = format!("{}/vectors/{}x{}", UTILSPATH, INSTANCE_SIZE, NUM_INSTANCE);

        // Check if the vectors have been generated
        match preprocess::FileInputIterator::open(&dir) {
            Ok(_) => {
                println!("Vectors already generated");
            }
            Err(_) => {
                preprocess::gen_vectors(INSTANCE_SIZE, NUM_INSTANCE, &dir);
            }
        }

        let benchmark_data = preprocess::FileInputIterator::open(&dir).unwrap();
        let result = benchmark_msm(&BENCHMARKSPATH, benchmark_data, 1);
        println!("Done running benchmark: {:?}", result);
    }

    #[test]
    fn test_run_benchmark() {
        let result =
            run_benchmark(INSTANCE_SIZE, NUM_INSTANCE, &UTILSPATH, &BENCHMARKSPATH).unwrap();
        println!("Benchmark result: {:#?}", result);
    }

    #[test]
    fn test_run_multi_benchmarks() {
        let output_path = format!("{}/{}_benchmark.txt", &BENCHMARKSPATH, "trapdoor");
        let mut output_file = File::create(output_path).expect("output file creation failed");
        writeln!(output_file, "msm_size,num_msm,avg_processing_time(ms)");
        let instance_size = vec![1, 4, 8, 12, 16];
        let num_instance = vec![1, 5, 10];
        for size in &instance_size {
            for num in &num_instance {
                let result = run_benchmark(*size, *num, &UTILSPATH, &BENCHMARKSPATH).unwrap();
                println!("{}x{} result: {:#?}", *size, *num, result);
                writeln!(
                    output_file,
                    "{},{},{}",
                    result.instance_size, result.num_instance, result.avg_processing_time
                );
            }
        }
    }
}
