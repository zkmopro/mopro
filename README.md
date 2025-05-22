# mopro

![Crates.io](https://img.shields.io/crates/v/mopro-ffi?label=mopro-ffi&style=flat-square)

Mopro is a toolkit for ZK app development on mobile. Mopro makes client-side proving on mobile simple.

To learn more about mopro, please refer to the documentation at [zkmopro](https://zkmopro.org/docs/intro).

## Getting started

-   Make sure you've installed the [prerequisites](https://zkmopro.org/docs/prerequisites).
-   Getting started with this [tutorial](https://zkmopro.org/docs/getting-started).

## Run tests

-   circom
    ```sh
    cd mopro-ffi
    cargo test --features circom
    ```
-   halo2
    ```sh
    cd mopro-ffi
    cargo test --features halo2
    ```
-   noir
    ```sh
    cd mopro-ffi
    cargo test --features noir --release
    ```
-   circom-e2e
    ```sh
    cd test-e2e
    curl -L https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar -o jna-5.13.0.jar
    CLASSPATH=jna-5.13.0.jar cargo test --test circom -- --nocapture
    ```
-   halo2-e2e
    ```sh
    cd test-e2e
    curl -L https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar -o jna-5.13.0.jar
    CLASSPATH=jna-5.13.0.jar cargo test --test halo2 -- --nocapture
    ```

## Performance

### Circom

Both native witness generation and proof generation are generally faster than `snarkjs` in the browser, with potential speed improvements of up to 20 times.
Check the details here: [performance](https://zkmopro.org/docs/performance).

### iOS

Benchmarks on an iPhone 16 Pro (2024).

| Witness generation | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witnesscalc](https://github.com/iden3/circom-witnesscalc) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
|-------------------|-----------------------------------------------------------|------------------------------------------------------------------|-----------------------------------------------------|------------------------------------------------|---------------------------------------------|
| Keccak256 | 142.1 ms (~1x) | 75.4 ms (**~2x**) | 287.7 ms (slower) | 140 ms (~1x) | 147.1 ms |
| SHA256 | 41 ms (**~2x**) | 51.3 ms (~1.7x) | 171.3 ms (slower) | 93.9 ms (~1x) | 91.8 ms |
| RSA | 153 ms (**~19x**) | - | 2937.5 ms (~1x) | 2312.3 ms (~1.2x) | 2979.5 ms |
| Semaphore | 22 ms (~3.5x) | 14.6 ms (**~5.3x**) | 266.5 ms (slower) | 38.9 ms (~2x) | 77.6 ms |
| Anon Aadhaar | 285.1 ms | - | 3284.7 ms | 1490.8 ms | - |

| Proof generation | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
|-----------------|---------------------------------------------------|-------------------------------------------------------|---------------------------------------------|
| Keccak256 | 630.3 ms (**~8.2x**) | 956.9 ms (~5.4x) | 5182.1 ms |
| SHA256 | 186.7 ms (**~8.2x**) | 498.6 ms (~3x) | 1487 ms |
| RSA | 749.1 ms (**~8.8x**) | 2250.8 ms (~3x) | 6604.5 ms |
| Semaphore | 143.3 ms (**~6.9x**) | 151.4 ms (~6.6x) | 1001.6 ms |
| Anon Aadhaar | 3131.7 ms | 10681.6 ms | - |

### Android

Benchmarks on an Samsung S23 Ultra (2023).

| Witness generation | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witnesscalc](https://github.com/iden3/circom-witnesscalc) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
|-------------------|-----------------------------------------------------------|------------------------------------------------------------------|-----------------------------------------------------|------------------------------------------------|---------------------------------------------|
| Keccak256 | 101.4 ms (~3x) | 71 ms (**~4x**) | 507.3 ms (slower) | 210.5 ms (~1.3x) | 292.3 ms |
| SHA256 | 29 ms (**~5x**) | 44 ms (~3.5x) | 271.6 ms (slower) | 106.9 ms (~1.4x) | 157.9 ms |
| RSA | 155 ms (**~25x**) | - | 4723 ms (slower) | 3751 ms (~1x) | 3958 ms |
| Semaphore | 10.3 ms (**~7x**) | 14.7 ms (~5x) | 416.9 ms (slower) | 32.8 ms (~2x) | 74.1 ms |
| Anon Aadhaar | 365.1 ms (**~8.7x**) | - | 5359.6 ms (slower) | 2716.4 ms (~1.1x) | 3207.5 ms |

| Proof generation | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
|-----------------|---------------------------------------------------|-------------------------------------------------------|---------------------------------------------|
| Keccak256 | 743.7 ms (**~14x**) | 2330.4 ms (~4.7x) | 11096.4 ms |
| SHA256 | 228.4 ms (**~15x**) | 1575.2 ms (~2x) | 3514.8 ms |
| RSA | 950 ms (**~14x**) | 5839 ms (~2.3x) | 13442 ms |
| Semaphore | 165.8 ms (**~13x**) | 276.9 ms (~7.7x) | 2146 ms |
| Anon Aadhaar | 3394.5 ms (**~15x**) | 33239.2 ms (~1.5x) | 51546.3 ms |

### Halo2

In summary: <br/>
The performance of the Mopro build is comparable to that of native Halo2 build. <br/>

The below tests were run on a Macbook Pro M1 Pro (2021) as well as an iPhone 15 Pro (2023).

| [Keccak256](https://github.com/ElusAegis/halo2-keccak-stable.git) | Prove Time (s) | Verify Time (s) |
| :---------------------------------------------------------------: | :------------: | :-------------: |
|                          Native (M1 Pro)                          |     10.3 s     |     0.15 s      |
|                         Emulator (M1 Pro)                         |     10.1 s     |     0.13 s      |
|                           iPhone 15 Pro                           |     11.0 s     |     0.12 s      |

| [RSA](https://github.com/ElusAegis/halo2-rsa-mopro.git) | Prove Time (s) | Verify Time (s) |
| :-----------------------------------------------------: | :------------: | :-------------: |
|                     Native (M1 Pro)                     |     76.5 s     |     11.1 s      |
|                    Emulator (M1 Pro)                    |     64.5 s     |      9.0 s      |
|                      iPhone 15 Pro                      |    crashes     |     crashes     |

Note that the iPhone 15 Pro crashes when running the RSA circuit due to the large memory requirements. The circuit needs
around 5GB of memory to run, while the iPhone 15 Pro usually limits the application memory usage to 3GB.

### Noir

We target [`nargo v1.0.0-beta.3`](https://noir-lang.org/docs/getting_started/quick_start#nargo) and [`bb v0.82.2`](https://noir-lang.org/docs/getting_started/quick_start#proving-backend-1), ensuring all related dependencies are aligned (e.g., [@noir-lang/noir\_js@1.0.0-beta.3](https://www.npmjs.com/package/@noir-lang/noir_js/v/1.0.0-beta.3), [@aztec/bb.js@0.82.2](https://www.npmjs.com/package/@aztec/bb.js/v/0.82.2)). The benchmarks below were run on a MacBook Air M3 (2024) and a Pixel 6 (2021). We report the fastest result from multiple runs to reduce cold-start effects.

| Circuit | iOS  | Android |  bb CLI<br/>(Laptop) |  Web<br/>(Laptop) |
| :-------------------------------------------------------------------------------: | :---------------: | :----------------: | :----------------: | :------------: |
| [Keccak256](https://github.com/moven0831/mopro-example-app-keccak256) | 349 ms (\~11.8×) |  1303 ms (\~3.2×) |  345 ms (\~12.0×) |  4122 ms |
| [RSA](https://github.com/moven0831/mopro-example-app-rsa) |  312 ms (\~6.6×) |  1091 ms (\~1.9×) |   221 ms (\~9.4×) |  2068 ms |
| [zkemail](https://github.com/Mach-34/zkemail.nr_header_demo) | 1309 ms (\~5.0×) |  4757 ms (\~1.4×) |   804 ms (\~8.2×) |  6590 ms |
| [anon aadhaar](https://github.com/moven0831/mopro-example-app-anon-aadhaar-noir) | 2225 ms (\~7.7×) |  8179 ms (\~2.1×) | 1366 ms (\~12.5×) | 17030 ms |
| [semaphore](https://github.com/moven0831/mopro-example-app-semaphore-noir) |  828 ms (\~5.6×) |  3990 ms (\~1.2×) |   436 ms (\~10.6×) |  4638 ms |

**Benchmark circuit details:**
- Keccak256: From [Noir Stdlib](https://noir-lang.org/docs/noir/standard_library/cryptographic_primitives/hashes#keccak256).
- RSA: pkcs1v15 signature verification (SHA-256). (see details in [zkpassport/noir_rsa](https://github.com/zkpassport/noir_rsa/blob/acb50389b79dbf38b1828f4fa82c28d742b140fc/src/rsa.nr#L286-L309))
- zkEmail: Header extraction circuit ([`Mach-34/zkemail.nr_header_demo`](https://github.com/Mach-34/zkemail.nr_header_demo)).
- Anon Aadhaar: Updated circuit (nargo v1.0.0-beta.3) in [`anon-aadhaar/anon-aadhaar-noir#6`](https://github.com/anon-aadhaar/anon-aadhaar-noir/pull/6).
- Semaphore: From [`hashcloak/semaphore-noir`](https://github.com/hashcloak/semaphore-noir).

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.


> [!IMPORTANT]
> We do not accept minor grammatical fixes (e.g., correcting typos, rewording sentences) unless they significantly improve clarity in technical documentation. These contributions, while appreciated, are not a priority for merging. If there is a grammatical error feel free to message the team.

