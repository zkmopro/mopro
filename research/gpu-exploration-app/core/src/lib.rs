// This file contains tests calling MSM functions from `mopro-core`
#[cfg(test)]
mod tests {
    use ark_serialize::Write;
    use mopro_core::middleware::gpu_explorations::{
        arkworks_pippenger, trapdoortech_zprize_msm, utils::benchmark::BenchmarkResult, utils::preprocess::HarnessError,
    };
    use std::fs::File;

    const UTILSPATH: &str = "benchmarks/vectors";
    const BENCHMARKSPATH: &str = "benchmarks/results";

    fn run_benchmarks<F>(algorithm: &str, instance_size: &[usize], num_instance: &[usize], benchmark_fn: F)
    where
        F: Fn(usize, usize, &str) -> Result<BenchmarkResult, HarnessError> + Sync,
    {
        let output_path = format!("{}/{}_benchmark.txt", BENCHMARKSPATH, algorithm);
        let mut output_file = File::create(output_path).expect("output file creation failed");
        writeln!(output_file, "msm_size,num_msm,avg_processing_time(ms)").unwrap();

        let results: Vec<Vec<(u32, u32, f64)>> = instance_size
            .iter()
            .map(|size| {
                num_instance
                    .iter()
                    .map(|num| {
                        let utils_path = format!("{}/{}x{}", UTILSPATH, *size, *num);
                        let result = benchmark_fn(*size, *num, &utils_path).unwrap();
                        println!("{}x{} result: {:#?}", *size, *num, result);
                        (result.instance_size, result.num_instance, result.avg_processing_time)
                    })
                    .collect()
            })
            .collect();

        for outer_result in results {
            for (instance_size, num_instance, avg_processing_time) in outer_result {
                writeln!(
                    output_file,
                    "{},{},{}",
                    instance_size, num_instance, avg_processing_time
                )
                .unwrap();
            }
        }
    }

    #[test]
    fn test_arkworks_msm() {
        let instance_size = &[8, 12, 16];
        let num_instance = &[5, 10];
        run_benchmarks(
            "arkworks_pippenger",
            instance_size,
            num_instance,
            |size: usize, num: usize, path: &str| arkworks_pippenger::run_benchmark(size as u32, num as u32, path),
        );
    }

    #[test]
    fn test_trapdoortech_msm() {
        let instance_size = &[8, 12, 16];
        let num_instance = &[5, 10];
        run_benchmarks(
            "trapdoortech_zprize_msm",
            instance_size,
            num_instance,
            |size: usize, num: usize, path: &str| trapdoortech_zprize_msm::run_benchmark(size as u32, num as u32, path),
        );
    }
}
