# GPU exploration

## Report links

- [Benchmarking TrapdoorTechZprize MSM & arkworks(0.4) MSM](https://hackmd.io/ZCxFpQ8AROyYGTl5GLqAGQ)

## Steps to run the benchmarks

There are currently 2 algorithms for benchmarking:

- `arkworks_pippenger`
- `trapdoortech_zprize_msm`

### mopro-core tests

To run the benchmarks for the benchmarking test in `mopro-core`:
`cargo test --release --features gpu-benchmarks --package mopro-core --lib -- middleware::gpu_explorations::<algorithm_you_want_to_test>::tests::test_run_multi_benchmarks --exact --nocapture`

replace `<algorithm_you_want_to_test>` with the algorithm name listed above.

it would start generating `points` (on `bls12-377`) and `scalars` (32-bytes) under `mopro-core/src/middlware/gpu-explorations/utils/vectors/`. And after that it will begin to benchmark.

The results are as below:

```bash
Vectors already generated
Average time to execute MSM with 4096 points and scalars in 1 iterations is: 47.739041ms
Average time to execute MSM with 4096 points and scalars in 1 iterations is: 48.719542ms
Average time to execute MSM with 4096 points and scalars in 1 iterations is: 49.344042ms
Average time to execute MSM with 4096 points and scalars in 1 iterations is: 48.545792ms
Average time to execute MSM with 4096 points and scalars in 1 iterations is: 48.26825ms
Average time to execute MSM with 4096 points and scalars in 1 iterations is: 48.587625ms
Average time to execute MSM with 4096 points and scalars in 1 iterations is: 48.371541ms
Average time to execute MSM with 4096 points and scalars in 1 iterations is: 49.861625ms
Average time to execute MSM with 4096 points and scalars in 1 iterations is: 48.5495ms
Done running benchmark.
12x5 result: BenchmarkResult {
    instance_size: 12,
    num_instance: 5,
    avg_processing_time: 48.66521755555556,
}
...
```

### mopro-ios benchmarking

1. cd to the root dir.
2. run `./scripts/build_ios.sh config-example.toml` (remember to change your ios_device_type `simulator`/`device`) to build and update the bindings.
3. open `mopro-ios/MoproKit/Example/MoproKit.xcworkspace` in Xcode.
4. choose your simulator/mobile device and use `cmd + R` to build.
5. choose `MSMBenchmark` and choose the algorithms and click the button below you want to start benchmark

### `ExampleGpuExploration` in templates

> The example project would be created soon.
