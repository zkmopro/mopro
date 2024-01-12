// This file is used to generate the benchmark report for the GPU exploration middleware.

use std::{env, fs::File, io::Write};

use mopro_core::middleware::gpu_exploration::run_msm_benchmark;

fn main() {
    let path = env::current_dir()
        .unwrap()
        .join("src/middleware/gpu_exploration/msm_bench.csv");
    let mut file = File::create(path).unwrap();
    writeln!(
        file,
        "num_msm,avg_processing_time(sec),total_processing_time(sec),memory_allocated(MiB)"
    )
    .unwrap();
    let trials = vec![1, 10, 50, 100, 250, 500, 750, 1_000];
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
