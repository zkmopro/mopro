use crate::middleware::gpu_explorations::utils::{
    benchmark::BenchmarkResult,
    precomputation::{
        deserialize_precomputed_points, precompute_points_from_instances,
        serialize_precomputed_points,
    },
    preprocess,
};
use ark_bn254::{Fr as ScalarField, G1Projective as G};
use ark_ec::VariableBaseMSM;
use ark_ff::{BigInteger, PrimeField};
use ark_std::{self, cfg_into_iter, One};
use std::time::{Duration, Instant};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

// Helper function for getting the windows size
fn ln_without_floats(a: usize) -> usize {
    // log2(a) * ln(2)
    (ark_std::log2(a) * 69 / 100) as usize
}

fn precompute_msm<V: VariableBaseMSM>(
    bases: &[V::MulBase], // precomputed_bases
    scalars: &[V::ScalarField],
) -> V {
    println!("bases length: {:?}", bases.len());
    let bigints = cfg_into_iter!(scalars)
        .map(|s| s.into_bigint())
        .collect::<Vec<_>>();

    let size = ark_std::cmp::min(bases.len(), bigints.len());
    let scalars = &bigints[..size];

    // let scalars_and_bases_iter = scalars.iter().zip(bases).filter(|(s, _)| !s.is_zero());
    // we can filter with scalar and precomputed bases as well

    let c = if size < 32 {
        3
    } else {
        ln_without_floats(size) + 2
    };

    let num_bits = V::ScalarField::MODULUS_BIT_SIZE as usize;
    let one = V::ScalarField::one().into_bigint();
    let zero = V::zero();

    let window_starts: Vec<_> = (0..num_bits).step_by(c).collect();
    let num_window = window_starts.len();
    println!("num_window: {:?}", num_window);

    let window_sums: Vec<_> = ark_std::cfg_into_iter!(window_starts)
        .enumerate()
        .map(|(j, w_start)| {
            let mut res = zero;
            let mut buckets = vec![zero; (1 << c) - 1];

            for (i, &scalar) in scalars.iter().clone().enumerate() {
                let scalar = scalar;
                if scalar == one {
                    if w_start == 0 {
                        res += bases[i];
                    }
                } else {
                    let mut scalar = scalar;
                    scalar.divn(w_start as u32);
                    let scalar = scalar.as_ref()[0] % (1 << c);
                    if scalar != 0 {
                        buckets[(scalar - 1) as usize] += bases[i + j * size];
                        // j == ((w_start + c - 1) / c) == window index
                    }
                }
            }

            let mut running_sum = V::zero();
            buckets.into_iter().rev().for_each(|b| {
                running_sum += &b;
                res += &running_sum;
            });
            res
        })
        .collect();

    let total_sums = window_sums.iter().sum();
    total_sums
    // We store the sum for the lowest window.
    // let lowest = *window_sums.first().unwrap();

    // We're traversing windows from high to low.
    // lowest
    //     + &window_sums[1..]
    //         .iter()
    //         .rev()
    //         .fold(zero, |mut total, sum_i| {
    //             total += sum_i;
    //             for _ in 0..c {
    //                 total.double_in_place();
    //             }
    //             total
    // })
}

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
        // map each scalar to a ScalarField
        let scalars = &instance
            .1
            .iter()
            .map(|s| ScalarField::new(*s))
            .collect::<Vec<ScalarField>>();
        let mut instance_total_duration = Duration::ZERO;
        for _i in 0..iterations {
            let start = Instant::now();
            let _ = precompute_msm::<G>(&points[..], &scalars[..]);
            instance_total_duration += start.elapsed();
        }
        let instance_avg_duration = instance_total_duration / iterations;

        println!(
            "Average time to execute MSM with {} points and {} scalars in {} iterations is: {:?}",
            points.len(),
            scalars.len(),
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

    let size = 1 << instance_size;
    let c = if size < 32 {
        3
    } else {
        ln_without_floats(size) + 2
    };
    let num_bit_of_scalar = ScalarField::MODULUS_BIT_SIZE as usize;
    let precompute_factor = (num_bit_of_scalar + c - 1) / c;
    let benchmark_data = preprocess::FileInputIterator::open(&utils_dir).unwrap();
    let instance_vec: Vec<_> = benchmark_data.collect();
    match preprocess::FileInputIterator::open_precomputed_point(&utils_dir) {
        Ok(_) => {
            println!("Precomputed points already generated");
        }
        Err(_) => {
            let _ = precompute_points_from_instances(
                instance_vec.clone().into_iter(),
                instance_size,
                num_instance,
                precompute_factor as u32,
                c as u32,
                &utils_dir,
            );
        }
    }
    let benchmark_data = preprocess::FileInputIterator::open_precomputed_point(&utils_dir).unwrap();
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

    use crate::middleware::gpu_explorations::utils::precomputation::{
        deserialize_precomputed_points, precompute_points_from_instances,
        serialize_precomputed_points,
    };
    use ark_serialize::Write;
    use std::fs::File;

    const INSTANCE_SIZE: u32 = 16;
    const NUM_INSTANCE: u32 = 5;
    const UTILSPATH: &str = "mopro-core/src/middleware/gpu_explorations/utils/vectors";
    const BENCHMARKSPATH: &str = "mopro-core/gpu_explorations/benchmarks";

    #[test]
    fn test_benchmark_msm() {
        let dir = format!(
            "{}/{}/{}x{}",
            preprocess::get_root_path(),
            UTILSPATH,
            INSTANCE_SIZE,
            NUM_INSTANCE
        );
        // Check if the vectors have been generated
        match preprocess::FileInputIterator::open(&dir) {
            Ok(_) => {
                println!("Vectors already generated");
            }
            Err(_) => {
                preprocess::gen_vectors(INSTANCE_SIZE, NUM_INSTANCE, &dir);
            }
        }

        let size = 1 << INSTANCE_SIZE;
        let c = if size < 32 {
            3
        } else {
            ln_without_floats(size) + 2
        };
        let num_bit_of_scalar = ScalarField::MODULUS_BIT_SIZE as usize;
        let precompute_factor = (num_bit_of_scalar + c - 1) / c;
        let benchmark_data = preprocess::FileInputIterator::open(&dir).unwrap();
        let instance_vec: Vec<_> = benchmark_data.collect();
        match preprocess::FileInputIterator::open_precomputed_point(&dir) {
            Ok(_) => {
                println!("Precomputed points already generated");
            }
            Err(_) => {
                let _ = precompute_points_from_instances(
                    instance_vec.clone().into_iter(),
                    INSTANCE_SIZE,
                    NUM_INSTANCE,
                    precompute_factor as u32,
                    c as u32,
                    &dir,
                );
            }
        }
        let benchmark_data = preprocess::FileInputIterator::open_precomputed_point(&dir).unwrap();

        let result = benchmark_msm(benchmark_data, 1);
        println!("Done running benchmark: {:?}", result);
    }

    #[test]
    fn test_run_benchmark() {
        let dir = format!(
            "{}/{}/{}x{}",
            preprocess::get_root_path(),
            &UTILSPATH,
            INSTANCE_SIZE,
            NUM_INSTANCE
        );

        let result = run_benchmark(INSTANCE_SIZE, NUM_INSTANCE, &dir).unwrap();
        println!("Benchmark result: {:#?}", result);
    }

    #[test]
    fn test_run_multi_benchmarks() {
        let output_path = format!(
            "{}/{}/{}_benchmark.txt",
            preprocess::get_root_path(),
            &BENCHMARKSPATH,
            "bucket_wise_msm"
        );
        let mut output_file = File::create(output_path).expect("output file creation failed");
        writeln!(output_file, "msm_size,num_msm,avg_processing_time(ms)").unwrap();

        let instance_size = vec![16, 18, 20, 22, 24, 26];
        let num_instance = vec![5];
        for size in &instance_size {
            for num in &num_instance {
                let utils_path = format!(
                    "{}/{}/{}x{}",
                    preprocess::get_root_path(),
                    &UTILSPATH,
                    *size,
                    *num
                );
                let result = run_benchmark(*size, *num, &utils_path).unwrap();
                println!("{}x{} result: {:#?}", *size, *num, result);
                writeln!(
                    output_file,
                    "{},{},{}",
                    result.instance_size, result.num_instance, result.avg_processing_time
                )
                .unwrap();
            }
        }
    }
}
