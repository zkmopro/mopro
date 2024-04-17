# GPU exploration

## Report links

- [Benchmarking TrapdoorTechZprize MSM & arkworks(0.4) MSM](https://hackmd.io/ZCxFpQ8AROyYGTl5GLqAGQ)

## Steps to run the benchmarks

There are currently 2 algorithms for benchmarking:

- `arkworks_pippenger`
- `trapdoortech_zprize_msm`

### mopro-core tests

To run the benchmarks of the instance size of $2^{16}$ on BLS12_377 curve in `mopro-core`, replace `<algorithm_you_want_to_test>` with the algorithm name listed above.

```bash
cargo test --release --features gpu-benchmarks --package mopro-core --lib -- middleware::gpu_explorations::<algorithm_you_want_to_test>::tests::test_run_benchmark --exact --nocapture
```

Run the benchmarks for multiple instance size. You can customize your own benchmark parameters on modular files in `mopro-core\src\middleware\gpu_explorations`
```bash
cargo test --release --features gpu-benchmarks --package mopro-core --lib -- middleware::gpu_explorations::<algorithm_you_want_to_test>::tests::test_run_multi_benchmarks --exact --nocapture
```

More context about the benchmark:
- It would generate instances size on BLS12_377 curve with scalar size of 32 bytes (i.e. can represent 0 to $2^{256}-1$ unsigned integer.) in `mopro-core/src/middlware/gpu-explorations/utils/vectors/`
- The instance size mean the amount of points and scalars.
- The msm time is linear to the size of instance.

The results are as below:

```bash
Vectors already generated
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 195.635083ms
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 206.639791ms
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 205.1675ms
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 197.742167ms
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 207.147166ms
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 199.729459ms
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 203.080416ms
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 198.15875ms
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 201.636916ms
Average time to execute MSM with 65536 points and scalars in 1 iterations is: 210.273792ms
Done running benchmark. Check the result at: "../mopro-core/benchmarks/gpu_explorations"
16x10 result: BenchmarkResult {
    instance_size: 16,
    num_instance: 10,
    avg_processing_time: 202.52110399999998,
}
...
```

### mopro-ios benchmarking

1. cd to the `mopro/` directory.
2. run `./scripts/build_ios.sh config-example.toml` (remember to change your ios_device_type `simulator`/`device`) to build and update the bindings.
3. open `mopro-ios/MoproKit/Example/MoproKit.xcworkspace` in Xcode.
4. choose your simulator/mobile device and build the project (can also use `cmd + R` as hot key).
5. choose `MSMBenchmark` and choose the algorithms and click the button below you want to start benchmark.

### `ExampleGpuExploration` in templates

> The example project would be created soon.
