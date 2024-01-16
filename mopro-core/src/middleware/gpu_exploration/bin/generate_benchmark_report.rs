// This file is used to generate the benchmark report for the GPU exploration middleware.


#[cfg(feature = "gpu-benchmarks")]
use {
    std::{env, fs::File, io::Write},
    mopro_core::middleware::gpu_exploration::run_msm_benchmark
};


#[cfg(feature = "gpu-benchmarks")]
fn main() {
    let path = env::current_dir()
        .unwrap()
        .join("benchmarks/gpu_explorations/msm_bench.csv");
    let mut file = File::create(path).unwrap();
    writeln!(
        file,
        "num_msm,avg_processing_time(sec),total_processing_time(sec),memory_allocated(MiB)"
    )
    .unwrap();
    // generate 30 figures to run (range from 1 to 1000)
    let trials = (1..1000).step_by(30);

    for each in trials {
        let bench_data = run_msm_benchmark(Some(each)).unwrap();
        writeln!(
            file,
            "{},{},{},{}",
            bench_data.num_msm,
            bench_data.avg_processing_time,
            bench_data.total_processing_time,
            bench_data.allocated_memory,
        )
        .unwrap();
    }
}
