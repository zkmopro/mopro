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

<table>
  <tr>
    <th>Witness generation</th>
    <th>[witnesscalc](https://github.com/0xPolygonID/witnesscalc)</th>
    <th>[circom-witnesscalc](https://github.com/iden3/circom-witnesscalc)</th>
    <th>[wasmer](https://github.com/arkworks-rs/circom-compat)</th>
    <th>[w2c](https://github.com/vimwitch/rust-witness)</th>
    <th>[snarkjs](https://github.com/iden3/snarkjs)</th>
  </tr>
  <tr>
    <td>Keccak256</td>
    <td>142.1 ms (~1x)</td>
    <td>75.4 ms (<font color="FFB546">**~2x**</font>)</td>
    <td>287.7 ms (slower)</td>
    <td>140 ms (~1x)</td>
    <td>147.1 ms </td>
  </tr>
  <tr>
    <td>SHA256</td>
    <td>41 ms (<font color="FFB546">**~2x**</font>)</td>
    <td>51.3 ms (~1.7x)</td>
    <td>171.3  ms (slower)</td>
    <td>93.9 ms (~1x)</td>
    <td>91.8 ms </td>
  </tr>
  <tr>
    <td>RSA</td>
    <td>153 ms (<font color="FFB546">**~19x**</font>)</td>
    <td>-</td>
    <td>2937.5 ms (~1x)</td>
    <td>2312.3 ms (~1.2x)</td>
    <td>2979.5 ms </td>
  </tr>
  <tr>
    <td>Semaphore</td>
    <td>22 ms (~3.5x)</td>
    <td>14.6 ms (<font color="FFB546">**~5.3x**</font>)</td>
    <td>266.5 ms (slower)</td>
    <td>38.9 ms (~2x)</td>
    <td>77.6 ms</td>
  </tr>
  <tr>
    <td>Anon Aadhaar</td>
    <td>285.1 ms</td>
    <td>-</td>
    <td>3284.7 ms</td>
    <td>1490.8 ms</td>
    <td>-</td>
  </tr>
</table>

<table>
  <tr>
    <th>Proof generation</th>
    <th>[rapidsnark](https://github.com/iden3/rapidsnark)</th>
    <th>[ark-works](https://github.com/arkworks-rs/circom-compat)</th>
    <th>[snarkjs](https://github.com/iden3/snarkjs)</th>
  </tr>
  <tr>
    <td>Keccak256</td>
    <td>630.3 ms (<font color="FFB546">**~8.2x**</font>)</td>
    <td>956.9 ms (~5.4x)</td>
    <td>5182.1 ms</td>
  </tr>
  <tr>
    <td>SHA256</td>
    <td>186.7 ms (<font color="FFB546">**~8.2x**</font>)</td>
    <td>498.6 ms (~3x)</td>
    <td>1487  ms</td>
  </tr>
  <tr>
    <td>RSA</td>
    <td>749.1 ms (<font color="FFB546">**~8.8x**</font>)</td>
    <td>2250.8 ms (~3x)</td>
    <td>6604.5 ms</td>
  </tr>
  <tr>
    <td>Semaphore</td>
    <td>143.3 ms (<font color="FFB546">**~6.9x**</font>)</td>
    <td>151.4 ms (~6.6x)</td>
    <td>1001.6 ms</td>
  </tr>
  <tr>
    <td>Anon Aadhaar</td>
    <td>3131.7 ms</td>
    <td>10681.6 ms</td>
    <td>-</td>
  </tr>
</table>

### Android

Benchmarks on an Samsung S23 Ultra (2023).

<table>
  <tr>
    <th>Witness generation</th>
    <th>[witnesscalc](https://github.com/0xPolygonID/witnesscalc)</th>
    <th>[circom-witnesscalc](https://github.com/iden3/circom-witnesscalc)</th>
    <th>[wasmer](https://github.com/arkworks-rs/circom-compat)</th>
    <th>[w2c](https://github.com/vimwitch/rust-witness)</th>
    <th>[snarkjs](https://github.com/iden3/snarkjs)</th>
  </tr>
  <tr>
    <td>Keccak256</td>
    <td>101.4 ms (~3x)</td>
    <td>71 ms (<font color="FFB546">**~4x**</font>)</td>
    <td>507.3 ms (slower)</td>
    <td>210.5 ms (~1.3x)</td>
    <td>292.3 ms</td>
  </tr>
  <tr>
    <td>SHA256</td>
    <td>29 ms (<font color="FFB546">**~5x**</font>)</td>
    <td>44 ms (~3.5x)</td>
    <td>271.6 ms (slower)</td>
    <td>106.9 ms (~1.4x)</td>
    <td>157.9 ms</td>
  </tr>
  <tr>
    <td>RSA</td>
    <td>155 ms (<font color="FFB546">**~25x**</font>)</td>
    <td>-</td>
    <td>4723 ms (slower)</td>
    <td>3751 ms (~1x)</td>
    <td>3958 ms</td>
  </tr>
  <tr>
    <td>Semaphore</td>
    <td>10.3 ms (<font color="FFB546">**~7x**</font>)</td>
    <td>14.7 ms (~5x)</td>
    <td>416.9 ms (slower)</td>
    <td>32.8 ms (~2x)</td>
    <td>74.1 ms</td>
  </tr>
  <tr>
    <td>Anon Aadhaar</td>
    <td>365.1 ms (<font color="FFB546">**~8.7x**</font>)</td>
    <td>-</td>
    <td>5359.6 ms (slower)</td>
    <td>2716.4 ms (~1.1x)</td>
    <td>3207.5 ms</td>
  </tr>
</table>

<table>
  <tr>
    <th>Proof generation</th>
    <th>[rapidsnark](https://github.com/iden3/rapidsnark)</th>
    <th>[ark-works](https://github.com/arkworks-rs/circom-compat)</th>
    <th>[snarkjs](https://github.com/iden3/snarkjs)</th>
  </tr>
  <tr>
    <td>Keccak256</td>
    <td>743.7 ms (<font color="FFB546">**~14x**</font>)</td>
    <td>2330.4 ms (~4.7x)</td>
    <td>11096.4 ms</td>
  </tr>
  <tr>
    <td>SHA256</td>
    <td>228.4 ms (<font color="FFB546">**~15x**</font>) </td>
    <td>1575.2 ms (~2x)</td>
    <td>3514.8 ms</td>
  </tr>
  <tr>
    <td>RSA</td>
    <td>950 ms (<font color="FFB546">**~14x**</font>)</td>
    <td>5839 ms (~2.3x)</td>
    <td>13442 ms</td>
  </tr>
  <tr>
    <td>Semaphore</td>
    <td>165.8 ms (<font color="FFB546">**~13x**</font>)</td>
    <td>276.9 ms (~7.7x)</td>
    <td>2146 ms</td>
  </tr>
  <tr>
    <td>Anon Aadhaar</td>
    <td>3394.5 ms (<font color="FFB546">**~15x**</font>)</td>
    <td>33239.2 ms (~1.5x)</td>
    <td>51546.3 ms</td>
  </tr>
</table>

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

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
