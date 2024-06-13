use ark_bn254::{Fq, Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use ark_std::{cfg_into_iter, vec::Vec};
// For benchmarking
use std::time::{Duration, Instant};

use crate::middleware::gpu_explorations::metal::abstraction::{
    errors::MetalError,
    limbs_conversion::{FromLimbs, ToLimbs},
    state::*,
};
use crate::middleware::gpu_explorations::utils::{benchmark::BenchmarkResult, preprocess};

use metal::*;
use objc::rc::autoreleasepool;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub struct MetalMsmData {
    pub window_size_buffer: Buffer,
    pub instances_size_buffer: Buffer,
    pub window_starts_buffer: Buffer,
    pub scalar_buffer: Buffer,
    pub base_buffer: Buffer,
    pub num_windows_buffer: Buffer,
    pub buckets_indices_buffer: Buffer,
    pub buckets_matrix_buffer: Buffer,
    pub res_buffer: Buffer,
    pub result_buffer: Buffer,
    // pub debug_buffer: Buffer,
}

pub struct MetalMsmParams {
    pub instances_size: u32,
    pub buckets_size: u32,
    pub window_size: u32,
    pub num_window: u64,
}

pub struct MetalMsmPipeline {
    pub init_buckets: ComputePipelineState,
    pub accumulation_and_reduction: ComputePipelineState,
    pub final_accumulation: ComputePipelineState,
    pub prepare_buckets_indices: ComputePipelineState,
    pub bucket_wise_accumulation: ComputePipelineState,
    pub sum_reduction: ComputePipelineState,
}

pub struct MetalMsmConfig {
    pub state: MetalState,
    pub pipelines: MetalMsmPipeline,
}

pub struct MetalMsmInstance {
    pub data: MetalMsmData,
    pub params: MetalMsmParams,
}

// Helper function for getting the windows size
fn ln_without_floats(a: usize) -> usize {
    // log2(a) * ln(2)
    (ark_std::log2(a) * 69 / 100) as usize
}

fn sort_buckets_indices(buckets_indices: &mut Vec<u32>) -> () {
    // parse the buckets_indices to a Vec<(u32, u32)>
    let mut buckets_indices_pairs: Vec<(u32, u32)> = Vec::new();
    for i in 0..buckets_indices.len() / 2 {
        // skip empty indices (0, 0)
        if buckets_indices[2 * i] == 0 && buckets_indices[2 * i + 1] == 0 {
            continue;
        }
        buckets_indices_pairs.push((buckets_indices[2 * i], buckets_indices[2 * i + 1]));
    }
    // parallel sort the buckets_indices_pairs by the first element
    buckets_indices_pairs.par_sort_by(|a, b| a.0.cmp(&b.0));

    // flatten the sorted pairs to a Vec<u32>
    buckets_indices.clear();
    for (start, end) in buckets_indices_pairs {
        buckets_indices.push(start);
        buckets_indices.push(end);
    }
}

pub fn setup_metal_state() -> MetalMsmConfig {
    let state = MetalState::new(None).unwrap();
    let init_buckets = state.setup_pipeline("initialize_buckets").unwrap();
    let accumulation_and_reduction = state
        .setup_pipeline("accumulation_and_reduction_phase")
        .unwrap();
    let final_accumulation = state.setup_pipeline("final_accumulation").unwrap();

    // TODO:
    let prepare_buckets_indices = state.setup_pipeline("prepare_buckets_indices").unwrap();
    let bucket_wise_accumulation = state.setup_pipeline("bucket_wise_accumulation").unwrap();
    let sum_reduction = state.setup_pipeline("sum_reduction").unwrap();
    // let make_histogram_uint32 = state.setup_pipeline("make_histogram_uint32").unwrap();
    // let reorder_uint32 = state.setup_pipeline("reorder_uint32").unwrap();

    // let make_histogram_uint32_raw = state.library.get_function("reorder_uint32", None).unwrap();
    // let tmp = state.setup_pipeline("reorder_uint32").unwrap();
    // println!("tmp: {:?}", tmp);
    // state.library.function_names().iter().for_each(|name| {
    //     println!("Function name: {:?}", name);
    // });
    // let compute_descriptor = ComputePipelineDescriptor::new();
    // compute_descriptor.set_compute_function(Some(&make_histogram_uint32_raw));
    // println!("make_histogram_uint32: {:?}", compute_descriptor.compute_function().unwrap());
    // println!("make_histogram_uint32: {:?}", result);

    MetalMsmConfig {
        state,
        pipelines: MetalMsmPipeline {
            init_buckets,
            accumulation_and_reduction,
            final_accumulation,
            prepare_buckets_indices,
            bucket_wise_accumulation,
            sum_reduction,
        },
    }
}

pub fn encode_instances(
    points: &[GAffine],
    scalars: &[ScalarField],
    config: &mut MetalMsmConfig,
) -> MetalMsmInstance {
    let modulus_bit_size = ScalarField::MODULUS_BIT_SIZE as usize;

    let instances_size = ark_std::cmp::min(points.len(), scalars.len());
    let window_size = if instances_size < 32 {
        3
    } else {
        ln_without_floats(instances_size) + 2
    };
    let buckets_size = (1 << window_size) - 1;
    let window_starts: Vec<usize> = (0..modulus_bit_size).step_by(window_size).collect();
    let num_windows = window_starts.len();

    // flatten scalar and base to Vec<u32> for GPU usage
    let scalars_limbs = cfg_into_iter!(scalars)
        .map(|s| s.into_bigint().to_u32_limbs())
        .flatten()
        .collect::<Vec<u32>>();
    let bases_limbs = cfg_into_iter!(points)
        .map(|b| {
            let b = b.into_group();
            b.x.to_u32_limbs()
                .into_iter()
                .chain(b.y.to_u32_limbs())
                .chain(b.z.to_u32_limbs())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<u32>>();

    // store params to GPU shared memory
    let window_size_buffer = config.state.alloc_buffer_data(&[window_size as u32]);
    let instances_size_buffer = config.state.alloc_buffer_data(&[instances_size as u32]);
    let scalar_buffer = config.state.alloc_buffer_data(&scalars_limbs);
    let base_buffer = config.state.alloc_buffer_data(&bases_limbs);
    let num_windows_buffer = config.state.alloc_buffer_data(&[num_windows as u32]);
    let buckets_matrix_buffer = config
        .state
        .alloc_buffer::<u32>(buckets_size * num_windows * 8 * 3);
    let res_buffer = config.state.alloc_buffer::<u32>(num_windows * 8 * 3);
    let result_buffer = config.state.alloc_buffer::<u32>(8 * 3);
    // convert window_starts to u32 to give the exact storage need for GPU
    let window_starts_buffer = config.state.alloc_buffer_data(
        &(window_starts
            .iter()
            .map(|x| *x as u32)
            .collect::<Vec<u32>>()),
    );
    // prepare bucket_size * num_windows * 2
    let buckets_indices_buffer = config
        .state
        .alloc_buffer::<u32>(instances_size * num_windows * 2);

    // // debug
    // let debug_buffer = config.state.alloc_buffer::<u32>(2048);

    MetalMsmInstance {
        data: MetalMsmData {
            window_size_buffer,
            instances_size_buffer,
            window_starts_buffer,
            scalar_buffer,
            base_buffer,
            num_windows_buffer,
            buckets_matrix_buffer,
            buckets_indices_buffer,
            res_buffer,
            result_buffer,
            // debug_buffer,
        },
        params: MetalMsmParams {
            instances_size: instances_size as u32,
            buckets_size: buckets_size as u32,
            window_size: window_size as u32,
            num_window: num_windows as u64,
        },
    }
}

pub fn exec_metal_commands(
    config: &MetalMsmConfig,
    instance: MetalMsmInstance,
) -> Result<G, MetalError> {
    let data = instance.data;
    let params = instance.params;

    // Init the pipleline for MSM
    let init_time = Instant::now();
    autoreleasepool(|| {
        let (command_buffer, command_encoder) = config.state.setup_command(
            &config.pipelines.init_buckets,
            Some(&[
                (0, &data.window_size_buffer),
                (1, &data.window_starts_buffer),
                (2, &data.buckets_matrix_buffer),
            ]),
        );
        command_encoder
            .dispatch_thread_groups(MTLSize::new(params.num_window, 1, 1), MTLSize::new(1, 1, 1));
        command_encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();
    });
    println!("Init buckets time: {:?}", init_time.elapsed());

    // // Accumulate and reduction
    // let acc_time = Instant::now();
    // autoreleasepool(|| {
    //     let (command_buffer, command_encoder) = config.state.setup_command(
    //         &config.pipelines.accumulation_and_reduction,
    //         Some(&[
    //             (0, &data.window_size_buffer),
    //             (1, &data.instances_size_buffer),
    //             (2, &data.window_starts_buffer),
    //             (3, &data.scalar_buffer),
    //             (4, &data.base_buffer),
    //             (5, &data.buckets_matrix_buffer),
    //             (6, &data.res_buffer),
    //         ]),
    //     );
    //     command_encoder.dispatch_threads(
    //         MTLSize::new(params.num_window, 1, 1),
    //         config.threads_per_threadgroup,
    //     );
    //     command_encoder.end_encoding();
    //     command_buffer.commit();
    //     command_buffer.wait_until_completed();
    // });
    // println!("Accumulation and Reduction time: {:?}", acc_time.elapsed());

    let prepare_time = Instant::now();
    autoreleasepool(|| {
        let (command_buffer, command_encoder) = config.state.setup_command(
            &config.pipelines.prepare_buckets_indices,
            Some(&[
                (0, &data.window_size_buffer),
                (1, &data.window_starts_buffer),
                (2, &data.num_windows_buffer),
                (3, &data.scalar_buffer),
                (4, &data.buckets_indices_buffer),
            ]),
        );
        command_encoder.dispatch_thread_groups(
            MTLSize::new(params.instances_size as u64, 1, 1),
            MTLSize::new(1, 1, 1),
        );
        command_encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();
    });
    println!("Prepare buckets indices time: {:?}", prepare_time.elapsed());

    // sort the buckets_indices in CPU parallelly
    let sort_start = Instant::now();
    let mut buckets_indices = MetalState::retrieve_contents::<u32>(&data.buckets_indices_buffer);
    sort_buckets_indices(&mut buckets_indices);

    // send the sorted buckets back to GPU
    let sorted_buckets_indices_buffer = config.state.alloc_buffer_data(&buckets_indices);
    println!("Sort buckets indices time: {:?}", sort_start.elapsed());

    // accumulate the buckets_matrix using sorted bucket indices on GPU
    let max_threads_per_group = MTLSize::new(
        config
            .pipelines
            .bucket_wise_accumulation
            .thread_execution_width(),
        config
            .pipelines
            .bucket_wise_accumulation
            .max_total_threads_per_threadgroup()
            / config
                .pipelines
                .bucket_wise_accumulation
                .thread_execution_width(),
        1,
    );
    let max_thread_size = params.buckets_size as u64 * params.num_window;
    let opt_threadgroups_amount = max_thread_size
        / config
            .pipelines
            .bucket_wise_accumulation
            .max_total_threads_per_threadgroup()
        + 1;
    let opt_threadgroups = MTLSize::new(opt_threadgroups_amount, 1, 1);
    println!(
        "(accumulation) max thread per threadgroup: {:?}",
        max_threads_per_group
    );
    println!("(accumulation) opt threadgroups: {:?}", opt_threadgroups);

    let max_thread_size_accu_buffer = config.state.alloc_buffer_data(&[max_thread_size as u32]);
    let bucket_wise_time = Instant::now();
    autoreleasepool(|| {
        let (command_buffer, command_encoder) = config.state.setup_command(
            &config.pipelines.bucket_wise_accumulation,
            Some(&[
                (0, &data.instances_size_buffer),
                (1, &data.num_windows_buffer),
                (2, &data.base_buffer),
                (3, &sorted_buckets_indices_buffer),
                (4, &data.buckets_matrix_buffer),
                (5, &max_thread_size_accu_buffer),
                // (6, &data.debug_buffer),
            ]),
        );
        // command_encoder.dispatch_thread_groups(
        //     MTLSize::new(params.buckets_size as u64 * params.num_window, 1, 1),
        //     MTLSize::new(1, 1, 1),
        // );
        command_encoder.dispatch_thread_groups(opt_threadgroups, max_threads_per_group);
        command_encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();
    });
    let bucket_wise_elapsed = bucket_wise_time.elapsed();
    println!(
        "Bucket wise accumulation time (using {:?} threads): {:?}",
        params.buckets_size as u64 * params.num_window,
        bucket_wise_elapsed
    );

    // // debug
    // let debug_data = MetalState::retrieve_contents::<u32>(&data.debug_buffer);
    // println!("Debug data: {:?}", debug_data);

    // Reduce the buckets_matrix on GPU
    let max_thread_size = params.num_window;
    let opt_threadgroups_amount = max_thread_size
        / config
            .pipelines
            .bucket_wise_accumulation
            .max_total_threads_per_threadgroup()
        + 1;
    let opt_threadgroups = MTLSize::new(opt_threadgroups_amount, 1, 1);
    let max_thread_size_reduc_buffer = config.state.alloc_buffer_data(&[max_thread_size as u32]);
    let reduction_time = Instant::now();
    autoreleasepool(|| {
        let (command_buffer, command_encoder) = config.state.setup_command(
            &config.pipelines.sum_reduction,
            Some(&[
                (0, &data.window_size_buffer),
                (1, &data.scalar_buffer),
                (2, &data.base_buffer),
                (3, &data.buckets_matrix_buffer),
                (4, &data.res_buffer),
                (5, &max_thread_size_reduc_buffer),
            ]),
        );
        // command_encoder
        //     .dispatch_thread_groups(MTLSize::new(params.num_window, 1, 1), MTLSize::new(1, 1, 1));
        command_encoder.dispatch_thread_groups(opt_threadgroups, max_threads_per_group);
        command_encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();
    });
    println!("Reduction time: {:?}", reduction_time.elapsed());

    // Sequentially accumulate the msm results on GPU
    let final_time = Instant::now();
    autoreleasepool(|| {
        let (command_buffer, command_encoder) = config.state.setup_command(
            &config.pipelines.final_accumulation,
            Some(&[
                (0, &data.window_size_buffer),
                (1, &data.window_starts_buffer),
                (2, &data.num_windows_buffer),
                (3, &data.res_buffer),
                (4, &data.result_buffer),
            ]),
        );
        command_encoder.dispatch_thread_groups(MTLSize::new(1, 1, 1), MTLSize::new(1, 1, 1));
        command_encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();
    });
    println!("Final accumulation time: {:?}", final_time.elapsed());

    // retrieve and parse the result from GPU
    let msm_result = {
        let raw_limbs = MetalState::retrieve_contents::<u32>(&data.result_buffer);
        G::new_unchecked(
            Fq::from_u32_limbs(&raw_limbs[0..8]),
            Fq::from_u32_limbs(&raw_limbs[8..16]),
            Fq::from_u32_limbs(&raw_limbs[16..24]),
        )
    };

    Ok(msm_result)
}

pub fn metal_msm(
    points: &[GAffine],
    scalars: &[ScalarField],
    config: &mut MetalMsmConfig,
) -> Result<G, MetalError> {
    let instance = encode_instances(points, scalars, config);
    exec_metal_commands(config, instance)
}

pub fn benchmark_msm<I>(
    instances: I,
    iterations: u32,
) -> Result<Vec<Duration>, preprocess::HarnessError>
where
    I: Iterator<Item = preprocess::Instance>,
{
    println!("Init metal (GPU) state...");
    let init_start = Instant::now();
    let mut metal_config = setup_metal_state();
    let init_duration = init_start.elapsed();
    println!("Done initializing metal (GPU) state in {:?}", init_duration);

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
            let encoding_data_start = Instant::now();
            println!("Encoding instance to GPU memory...");
            let metal_instance = encode_instances(&points[..], &scalars[..], &mut metal_config);
            let encoding_data_duration = encoding_data_start.elapsed();
            println!("Done encoding data in {:?}", encoding_data_duration);

            let msm_start = Instant::now();
            let _result = exec_metal_commands(&metal_config, metal_instance).unwrap();
            instance_total_duration += msm_start.elapsed();
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

    use ark_ec::{CurveGroup, VariableBaseMSM};
    use ark_serialize::Write;
    use ark_std::UniformRand;
    use std::fs::File;

    const INSTANCE_SIZE: u32 = 16;
    const NUM_INSTANCE: u32 = 5;
    const UTILSPATH: &str = "mopro-core/src/middleware/gpu_explorations/utils/vectors";
    const BENCHMARKSPATH: &str = "mopro-core/gpu_explorations/benchmarks";

    #[test]
    fn test_msm_correctness_small_sample() {
        let mut rng = ark_std::rand::thread_rng();
        let p1 = G::rand(&mut rng);
        let p2 = G::rand(&mut rng);

        let s1 = ScalarField::rand(&mut rng);
        let s2 = ScalarField::rand(&mut rng);

        // compare with msm correctness with arkworks ec arithmetics
        let target_msm = p1 * s1 + p2 * s2;

        // Init metal (GPU) state
        let mut metal_config = setup_metal_state();
        let this_msm = metal_msm(
            &[p1.into_affine(), p2.into_affine()],
            &[s1, s2],
            &mut metal_config,
        )
        .unwrap();

        assert_eq!(target_msm, this_msm, "This msm is wrongly computed");
    }

    #[test]
    fn test_msm_correctness_medium_sample() {
        let dir = format!("{}/{}/{}x{}", preprocess::get_root_path(), UTILSPATH, 8, 5);
        // Init metal (GPU) state
        let mut metal_config = setup_metal_state();

        // Check if the vectors have been generated
        match preprocess::FileInputIterator::open(&dir) {
            Ok(_) => {
                println!("Vectors already generated");
            }
            Err(_) => {
                preprocess::gen_vectors(INSTANCE_SIZE, NUM_INSTANCE, &dir);
            }
        }

        let instances = preprocess::FileInputIterator::open(&dir).unwrap();

        for (i, instance) in instances.enumerate() {
            let points = &instance.0;
            // map each scalar to a ScalarField
            let scalars = &instance
                .1
                .iter()
                .map(|s| ScalarField::new(*s))
                .collect::<Vec<ScalarField>>();
            let arkworks_msm = G::msm(&points[..], &scalars[..]).unwrap();
            let metal_msm = metal_msm(&points[..], &scalars[..], &mut metal_config).unwrap();
            assert_eq!(metal_msm, arkworks_msm, "This msm is wrongly computed");
            println!(
                "(pass) {}th instance of size 2^{} is correctly computed",
                i, 8
            );
        }
    }

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

        let benchmark_data = preprocess::FileInputIterator::open(&dir).unwrap();
        let result = benchmark_msm(benchmark_data, 1);
        println!("Done running benchmark: {:?}", result);
    }

    #[test]
    fn test_run_benchmark() {
        let utils_path = format!(
            "{}/{}/{}x{}",
            preprocess::get_root_path(),
            &UTILSPATH,
            INSTANCE_SIZE,
            NUM_INSTANCE
        );
        let result = run_benchmark(INSTANCE_SIZE, NUM_INSTANCE, &utils_path).unwrap();
        println!("Benchmark result: {:#?}", result);
    }

    #[test]
    fn test_run_multi_benchmarks() {
        let output_path = format!(
            "{}/{}/{}_benchmark.txt",
            preprocess::get_root_path(),
            &BENCHMARKSPATH,
            "metal_msm"
        );
        let mut output_file = File::create(output_path).expect("output file creation failed");
        writeln!(output_file, "msm_size,num_msm,avg_processing_time(ms)").unwrap();

        let instance_size = vec![8, 12, 16, 18, 20, 22];
        let num_instance = vec![10];
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
