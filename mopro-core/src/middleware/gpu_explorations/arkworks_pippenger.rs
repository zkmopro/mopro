use ark_bls12_377::{Fr as ScalarField, G1Affine, G1Projective};
// use ark_bn254::{Fr as ScalarField, FrConfig, G1Affine as GAffine, G1Projective as G};
use ark_ec::{AffineRepr, VariableBaseMSM};
use ark_ff::{BigInt, Field, FpConfig};
use ark_serialize::Write;

use crate::middleware::gpu_explorations::utils::{benchmark::BenchmarkResult, preprocess};

use std::fs::File;
use std::time::{Duration, Instant};

pub fn benchmark_msm<I>(
    instances: I,
    iterations: u32,
) -> Result<Vec<Duration>, preprocess::HarnessError>
where
    I: Iterator<Item = preprocess::Instance>,
{
    let mut instance_durations = Vec::new();

    for instance in instances {
        let points = &instance.0;
        let scalars = &instance.1;
        let mut parsed_points = Vec::<G1Affine>::new();
        let mut parsed_scalars = Vec::<ScalarField>::new();

        // parse points and scalars from arkworks 0.3 compatible format to 0.4 compatible
        for p in points {
            let new_p =
                G1Affine::new_unchecked(BigInt::new(p.x.0 .0).into(), BigInt::new(p.y.0 .0).into());
            parsed_points.push(new_p);
        }

        for s in scalars {
            let new_s = ScalarField::new(BigInt::new(s.0));
            parsed_scalars.push(new_s);
        }

        let mut instance_total_duration = Duration::ZERO;
        for i in 0..iterations {
            let start = Instant::now();
            let result =
                <G1Projective as VariableBaseMSM>::msm(&parsed_points[..], &parsed_scalars[..])
                    .unwrap();

            instance_total_duration += start.elapsed();
        }
        let instance_avg_duration = instance_total_duration / iterations;

        println!(
            "Average time to execute MSM with {} points and scalars in {} iterations is: {:?}",
            points.len(),
            iterations,
            instance_avg_duration,
        );
        instance_durations.push(instance_avg_duration);
    }
    Ok(instance_durations)
}

pub fn run_benchmark(
    instance_size: u32,
    num_instance: u32,
    utils_dir: &str,
) -> Result<BenchmarkResult, preprocess::HarnessError> {
    // Check if the vectors have been generated
    match preprocess::FileInputIterator::open(&utils_dir) {
        Ok(_) => {
            println!("Vectors already generated");
        }
        Err(_) => {
            preprocess::gen_vectors(instance_size, num_instance, &utils_dir);
        }
    }

    let benchmark_data = preprocess::FileInputIterator::open(&utils_dir).unwrap();
    let instance_durations = benchmark_msm(benchmark_data, 1).unwrap();
    // in milliseconds
    let avg_processing_time: f64 = instance_durations
        .iter()
        .map(|d| d.as_secs_f64() * 1000.0)
        .sum::<f64>()
        / instance_durations.len() as f64;

    println!("Done running benchmark.");
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
    const UTILSPATH: &str = "../mopro-core/src/middleware/gpu_explorations/utils/vectors";
    const BENCHMARKSPATH: &str = "../mopro-core/benchmarks/gpu_explorations";

    #[test]
    fn test_benchmark_msm() {
        let dir = format!("{}/{}x{}", UTILSPATH, INSTANCE_SIZE, NUM_INSTANCE);

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
        let result = benchmark_msm(benchmark_data, 1);
        println!("Done running benchmark: {:?}", result);
    }

    #[test]
    fn test_run_benchmark() {
        let utils_path = format!("{}/{}x{}", &UTILSPATH, INSTANCE_SIZE, NUM_INSTANCE);
        let result = run_benchmark(INSTANCE_SIZE, NUM_INSTANCE, &utils_path).unwrap();
        println!("Benchmark result: {:#?}", result);
    }

    #[test]
    fn test_run_multi_benchmarks() {
        let output_path = format!("{}/{}_benchmark.txt", &BENCHMARKSPATH, "arkworks_pippenger");
        let mut output_file = File::create(output_path).expect("output file creation failed");
        writeln!(output_file, "msm_size,num_msm,avg_processing_time(ms)");
        let instance_size = vec![8, 12, 16, 18];
        let num_instance = vec![5, 10];
        for size in &instance_size {
            for num in &num_instance {
                let utils_path = format!("{}/{}x{}", &UTILSPATH, *size, *num);
                let result = run_benchmark(*size, *num, &utils_path).unwrap();
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
