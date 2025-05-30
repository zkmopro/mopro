---
sidebar_position: 5
---

# Performance and Benchmarks

## Circom

In summary: <br/>
Both native witness generation and proof generation are generally faster than `snarkjs` in the browser, with potential speed improvements of up to **20 times**. <br/>
However, performance varies across different circuits.
We _recommend_ developers benchmark their custom circuits before selecting tools for app development.

:::warning

- [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) is not fully compatible with circom circuits. See: [zkmopro/mopro#32](https://github.com/zkmopro/mopro/issues/32).
- [wasmer](https://github.com/arkworks-rs/circom-compat) doesn't work in iOS. See: [zkmopro/mopro#109](https://github.com/zkmopro/mopro/issues/109).

:::

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
      <td>[Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256)</td>
      <td>142.1 ms (~1x)</td>
      <td>75.4 ms (<font color="FFB546">**~2x**</font>)</td>
      <td>287.7 ms (slower)</td>
      <td>140 ms (~1x)</td>
      <td>147.1 ms </td>
    </tr>
    <tr>
      <td>[SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256)</td>
      <td>41 ms (<font color="FFB546">**~2x**</font>)</td>
      <td>51.3 ms (~1.7x)</td>
      <td>171.3  ms (slower)</td>
      <td>93.9 ms (~1x)</td>
      <td>91.8 ms </td>
    </tr>
    <tr>
      <td>[RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa)</td>
      <td>153 ms (<font color="FFB546">**~19x**</font>)</td>
      <td>-</td>
      <td>2937.5 ms (~1x)</td>
      <td>2312.3 ms (~1.2x)</td>
      <td>2979.5 ms </td>
    </tr>
    <tr>
      <td>[Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32)</td>
      <td>22 ms (~3.5x)</td>
      <td>14.6 ms (<font color="FFB546">**~5.3x**</font>)</td>
      <td>266.5 ms (slower)</td>
      <td>38.9 ms (~2x)</td>
      <td>77.6 ms</td>
    </tr>
    <tr>
      <td>[Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar)</td>
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
      <td>[Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256)</td>
      <td>630.3 ms (<font color="FFB546">**~8.2x**</font>)</td>
      <td>956.9 ms (~5.4x)</td>
      <td>5182.1 ms</td>
    </tr>
    <tr>
      <td>[SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256)</td>
      <td>186.7 ms (<font color="FFB546">**~8.2x**</font>)</td>
      <td>498.6 ms (~3x)</td>
      <td>1487  ms</td>
    </tr>
    <tr>
      <td>[RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa)</td>
      <td>749.1 ms (<font color="FFB546">**~8.8x**</font>)</td>
      <td>2250.8 ms (~3x)</td>
      <td>6604.5 ms</td>
    </tr>
    <tr>
      <td>[Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32)</td>
      <td>143.3 ms (<font color="FFB546">**~6.9x**</font>)</td>
      <td>151.4 ms (~6.6x)</td>
      <td>1001.6 ms</td>
    </tr>
    <tr>
      <td>[Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar)</td>
      <td>3131.7 ms</td>
      <td>10681.6 ms</td>
      <td>-</td>
    </tr>
  </table>

:::info
**Details:** [Spreadsheet of Circom benchmark (iOS)](https://docs.google.com/spreadsheets/d/1MFABmsYSUsWDmhbjleqhBXk7nkYwhu589yK-CHtRkNI/edit?usp=sharing)
:::

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
      <td>[Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256)</td>
      <td>101.4 ms (~3x)</td>
      <td>71 ms (<font color="FFB546">**~4x**</font>)</td>
      <td>507.3 ms (slower)</td>
      <td>210.5 ms (~1.3x)</td>
      <td>292.3 ms</td>
    </tr>
    <tr>
      <td>[SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256)</td>
      <td>29 ms (<font color="FFB546">**~5x**</font>)</td>
      <td>44 ms (~3.5x)</td>
      <td>271.6 ms (slower)</td>
      <td>106.9 ms (~1.4x)</td>
      <td>157.9 ms</td>
    </tr>
    <tr>
      <td>[RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa)</td>
      <td>155 ms (<font color="FFB546">**~25x**</font>)</td>
      <td>-</td>
      <td>4723 ms (slower)</td>
      <td>3751 ms (~1x)</td>
      <td>3958 ms</td>
    </tr>
    <tr>
      <td>[Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32)</td>
      <td>10.3 ms (<font color="FFB546">**~7x**</font>)</td>
      <td>14.7 ms (~5x)</td>
      <td>416.9 ms (slower)</td>
      <td>32.8 ms (~2x)</td>
      <td>74.1 ms</td>
    </tr>
    <tr>
      <td>[Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar)</td>
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
      <td>[Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256)</td>
      <td>743.7 ms (<font color="FFB546">**~14x**</font>)</td>
      <td>2330.4 ms (~4.7x)</td>
      <td>11096.4 ms</td>
    </tr>
    <tr>
      <td>[SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256)</td>
      <td>228.4 ms (<font color="FFB546">**~15x**</font>) </td>
      <td>1575.2 ms (~2x)</td>
      <td>3514.8 ms</td>
    </tr>
    <tr>
      <td>[RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa)</td>
      <td>950 ms (<font color="FFB546">**~14x**</font>)</td>
      <td>5839 ms (~2.3x)</td>
      <td>13442 ms</td>
    </tr>
    <tr>
      <td>[Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32)</td>
      <td>165.8 ms (<font color="FFB546">**~13x**</font>)</td>
      <td>276.9 ms (~7.7x)</td>
      <td>2146 ms</td>
    </tr>
    <tr>
      <td>[Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar)</td>
      <td>3394.5 ms (<font color="FFB546">**~15x**</font>)</td>
      <td>33239.2 ms (~1.5x)</td>
      <td>51546.3 ms</td>
    </tr>
  </table>

:::info
**Details:** [Spreadsheet of Circom benchmark (Android)](https://docs.google.com/spreadsheets/d/1TDgL2NXxYl8UH-RZPWfWdawY0tjxf3c6l8B11_FG7Kg/edit?usp=sharing)
:::

### macOS

Benchmarks on an Macbook Pro M1 Max (2021).

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
      <td>[Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256)</td>
      <td>63.9 ms (<font color="FFB546">**~5x**</font>)</td>
      <td>69.6 ms (~5x)</td>
      <td>507.7 ms (slower)</td>
      <td>214.6 ms (~1.6x)</td>
      <td>348.7 ms</td>
    </tr>
    <tr>
      <td>[SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256)</td>
      <td>22 ms (<font color="FFB546">**~10x**</font>)</td>
      <td>32 ms (~7x)</td>
      <td>272 ms (slower)</td>
      <td>112 ms (~2x)</td>
      <td>225 ms</td>
    </tr>
    <tr>
      <td>[RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa)</td>
      <td>164 ms (<font color="FFB546">**~29x**</font>)</td>
      <td>-</td>
      <td>5326 ms (slower)</td>
      <td>4796 ms (slower)</td>
      <td>4777 ms</td>
    </tr>
    <tr>
      <td>[Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32)</td>
      <td>7.1 ms (~18x)</td>
      <td>5 ms (<font color="FFB546">**~26x**</font>)</td>
      <td>287 ms (slower)</td>
      <td>34.9 ms (~3.7x)</td>
      <td>130 ms</td>
    </tr>
    <tr>
      <td>[Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar)</td>
      <td>324 ms (<font color="FFB546">**~10x**</font>)</td>
      <td>-</td>
      <td>5369 ms (slower)</td>
      <td>2902 ms (~1.2x)</td>
      <td>3437 ms</td>
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
      <td>[Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256)</td>
      <td>528 ms (<font color="FFB546">**~7x**</font>)</td>
      <td>1161 ms (~3.3x)</td>
      <td>3873 ms</td>
    </tr>
    <tr>
      <td>[SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256)</td>
      <td>158 ms (<font color="FFB546">**~9x**</font>) </td>
      <td>779 ms (~2x)</td>
      <td>1462 ms</td>
    </tr>
    <tr>
      <td>[RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa)</td>
      <td>676 ms (<font color="FFB546">**~8x**</font>)</td>
      <td>3005 ms (~1.8x)</td>
      <td>5553 ms</td>
    </tr>
    <tr>
      <td>[Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32)</td>
      <td>112 ms (~7.8x)</td>
      <td>84 ms (<font color="FFB546">**~10x**</font>)</td>
      <td>877 ms</td>
    </tr>
    <tr>
      <td>[Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar)</td>
      <td>2421 ms (<font color="FFB546">**~8x**</font>)</td>
      <td>14142 ms (~1.3x)</td>
      <td>19794 ms</td>
    </tr>
  </table>

:::info
**Details:** [Spreadsheet of Circom benchmark (host)](https://docs.google.com/spreadsheets/d/1irKg_TOP-yXms8igwCN_3OjVrtFe5gTHkuF0RbrVuho/edit?usp=sharing)
:::

### Web

We have enabled `wasm-bindgen-rayon` for multithreading in the browser. Below is a benchmark comparing arkworks with `wasm-bindgen-rayon` against `snarkjs`.

- **iPhone 16 Pro**

  <table>
    <tr>
      <th>Proof generation</th>
      <th>[ark-works with `rayon`](https://github.com/RReverser/wasm-bindgen-rayon)</th>
      <th>[snarkjs](https://github.com/iden3/snarkjs)</th>
    </tr>
    <tr>
      <td>[Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256)</td>
      <td>1717.81 ms	 (<font color="FFB546">**~3x**</font>)</td>
      <td>5166.02 ms</td>
    </tr>
    <tr>
      <td>[SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256)</td>
      <td>601.32 ms	</td>
      <td>380.61 ms (<font color="FFB546">**~1.5x**</font>) </td>
    </tr>
    <tr>
      <td>[RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa)</td>
      <td>7152.85 ms (<font color="FFB546">**~1.1x**</font>)</td>
      <td>8473.58 ms</td>
    </tr>
    <tr>
      <td>[Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32)</td>
      <td>652.07 ms (<font color="FFB546">**~1.4x**</font>)</td>
      <td>919.54 ms</td>
    </tr>
    <tr>
      <td>[Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar)</td>
      <td>- ms</td>
      <td>- ms</td>
    </tr>
  </table>

- **Android Samsung S23U**

  <table>
    <tr>
      <th>Proof generation</th>
      <th>[ark-works with `rayon`](https://github.com/RReverser/wasm-bindgen-rayon)</th>
      <th>[snarkjs](https://github.com/iden3/snarkjs)</th>
    </tr>
    <tr>
      <td>[Keccak256](https://github.com/zkmopro/circuit-registry/tree/main/keccak256)</td>
      <td>2733.15 ms	 (<font color="FFB546">**~2.7x**</font>)</td>
      <td>7412.39 ms</td>
    </tr>
    <tr>
      <td>[SHA256](https://github.com/zkmopro/circuit-registry/tree/main/sha256)</td>
      <td>980.64 ms	(<font color="FFB546">**~2.4x**</font>) </td>
      <td>2379.58 ms</td>
    </tr>
    <tr>
      <td>[RSA](https://github.com/zkmopro/circuit-registry/tree/main/rsa)</td>
      <td>9313.07 ms (<font color="FFB546">**~1.1x**</font>)</td>
      <td>10725.49 ms</td>
    </tr>
    <tr>
      <td>[Semaphore-32](https://github.com/zkmopro/circuit-registry/tree/main/semaphore-32)</td>
      <td>792.87 ms	(<font color="FFB546">**~1.3x**</font>)</td>
      <td>1045.34 ms</td>
    </tr>
    <tr>
      <td>[Anon Aadhaar](https://github.com/zkmopro/circuit-registry/tree/main/anonAadhaar)</td>
      <td>- ms</td>
      <td>- ms</td>
    </tr>
  </table>

:::info
**Details:** [zkmopro/mopro#290](https://github.com/zkmopro/mopro/issues/290)
:::

## Halo2

In summary: <br/>
The performance of the Mopro build is comparable to that of native Halo2 build. <br/>

The below tests were run on a Macbook Pro M1 Pro (2021) as well as an iPhone 15 Pro (2023).

| [Keccak256](https://github.com/ElusAegis/halo2-keccak-stable.git) | Prove Time (s) | Verify Time (s) |
| :---------------------------------------------------------------: | :------------: | :-------------: |
|                          Native (M1 Pro)                          |     10.3 s     |     0.15 s      |
|                           iPhone 15 Pro                           |     11.0 s     |     0.12 s      |
|                            Pixel 6 Pro                            |     23.9 s     |     0.51 s      |
|                                Web                                |     26.6 s     |     0.55 s      |

| [RSA](https://github.com/KimiWu123/halo2-rsa-mopro.git) | Prove Time (s) | Verify Time (s) |
| :-----------------------------------------------------: | :------------: | :-------------: |
|                     Native (M1 Pro)                     |     26.9 s     |      4.0 s      |
|                      iPhone 15 Pro                      |    crashes     |     crashes     |
|                       Pixel 6 Pro                       |    crashes     |     crashes     |
|                           Web                           |  Not Support   |        x        |

Note that the iPhone 15 Pro and Pixel 6 Pro crash when running the RSA circuit due to the large memory requirements. The circuit needs
around 5GB of memory to run, while the iPhone 15 Pro and Pixel 6 Pro usually limits the application memory usage to 3GB.
Note that the RSA circuit is built on top of former version of Halo2. When it generates a proof on web, it crashes inside Halo2 module.

## Noir

We target [`nargo v1.0.0-beta.3`](https://noir-lang.org/docs/getting_started/quick_start#nargo) and [`bb v0.82.2`](https://noir-lang.org/docs/getting_started/quick_start#proving-backend-1), ensuring all related dependencies are aligned (e.g., [@noir-lang/noir_js@1.0.0-beta.3](https://www.npmjs.com/package/@noir-lang/noir_js/v/1.0.0-beta.3), [@aztec/bb.js@0.82.2](https://www.npmjs.com/package/@aztec/bb.js/v/0.82.2)). The benchmarks below were run on a MacBook Air M3 (2024) and a Pixel 6 (2021). We report the fastest result from multiple runs to reduce cold-start effects.

On iOS, the benchmark runs using the "My Mac (Designed for iPad)" target. On Android, it runs on a Pixel 6 in release mode. On bb CLI, we measure wall-clock time with the `time bb prove` command. On the website, the benchmark runs in a browser-based (WASM) environment.

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

:::info
**Details:** [zkmopro/mopro#414](https://github.com/zkmopro/mopro/issues/414)
:::
