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

-   [witnesscalc](https://github.com/0xPolygonID/witnesscalc) hasn't been integrated in mopro. See [zkmopro/mopro#110](https://github.com/zkmopro/mopro/issues/110).
-   [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) is not fully compatible with circom circuits. See: [zkmopro/mopro#32](https://github.com/zkmopro/mopro/issues/32).
-   [wasmer](https://github.com/arkworks-rs/circom-compat) doesn't work in iOS. See: [zkmopro/mopro#109](https://github.com/zkmopro/mopro/issues/109).
-   [Tachyon](https://github.com/kroma-network/tachyon) performs well in [macOS](#macos), but we haven't integrated it in
    mobile. See [zkmopro/mopro#143](https://github.com/zkmopro/mopro/issues/143)
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
:::info
**Details:** [Spreadsheet of Circom benchmark (Android)](https://docs.google.com/spreadsheets/d/1TDgL2NXxYl8UH-RZPWfWdawY0tjxf3c6l8B11_FG7Kg/edit?usp=sharing)
:::

### macOS

Benchmarks on an Macbook Pro M1 Max (2021).

  <summary>Witness generation</summary>

|        SHA256        | [Tachyon](https://github.com/kroma-network/tachyon) | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| :------------------: | :-------------------------------------------------: | :-------------------------------------------------------: | :-----------------------------------------------------------------: | :----------------------------------------------------: | :---------------------------------------------: | :-----------------------------------------: |
|       Average        |                       32.7 ms                       |                          22.2 ms                          |                               42.8 ms                               |                        454.5 ms                        |                     88.8 ms                     |                  132.8 ms                   |
|        Stdev         |                       0.7 ms                        |                          5.2 ms                           |                               2.2 ms                                |                        26.7 ms                         |                     1.0 ms                      |                   1.3 ms                    |
| Comparing to snarkjs |                         ~4x                         |            <font color="FFB546">**~6x**</font>            |                                 ~3x                                 |                        ~(-3.4)x                        |                      ~1.5x                      |                      -                      |

<iframe width="466" height="254" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=2079338651&amp;format=interactive"></iframe>

|      Keccak256       | [Tachyon](https://github.com/kroma-network/tachyon) | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| :------------------: | :-------------------------------------------------: | :-------------------------------------------------------: | :-----------------------------------------------------------------: | :----------------------------------------------------: | :---------------------------------------------: | :-----------------------------------------: |
|       Average        |                       82.9 ms                       |                          72.3 ms                          |                               14.1 ms                               |                        447.1 ms                        |                     169 ms                      |                  234.6 ms                   |
|        Stdev         |                       0.2 ms                        |                          7.7 ms                           |                               0.8 ms                                |                         5.9 ms                         |                     2.0 ms                      |                   3.2 ms                    |
| Comparing to snarkjs |                        ~2.8x                        |                            ~3x                            |                <font color="FFB546">**~16x**</font>                 |                        ~(-1.9)x                        |                      x1.4x                      |                      -                      |

<iframe width="530" height="245" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=129836159&amp;format=interactive"></iframe>

|         RSA          | [Tachyon](https://github.com/kroma-network/tachyon) | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| :------------------: | :-------------------------------------------------: | :-------------------------------------------------------: | :-----------------------------------------------------------------: | :----------------------------------------------------: | :---------------------------------------------: | :-----------------------------------------: |
|       Average        |                      218.4 ms                       |                         167.6 ms                          |                              522.9 ms                               |                        5109 ms                         |                    3847.2 ms                    |                  4638.8 ms                  |
|        Stdev         |                       16.8 ms                       |                          7.5 ms                           |                               7.7 ms                                |                        25.1 ms                         |                     61.3 ms                     |                   32.4 ms                   |
| Comparing to snarkjs |                        ~21.2                        |           <font color="FFB546">**~27x**</font>            |                                ~8.8x                                |                        ~(-1.1)x                        |                      ~1.2x                      |                      -                      |

<iframe width="552" height="259" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=1347845461&amp;format=interactive"></iframe>

  <summary>Proof generation</summary>
| SHA256 | [Tachyon](https://github.com/kroma-network/tachyon) | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | 
| Average | 536.1 ms | 773.8 ms | 1137.3 ms | 1350.4 ms |
| Stdev | 10 ms | 17 ms | 127 ms | 26 ms |
| Comparing to snarkjs | <font color="FFB546">**~2.5x**</font> | ~1.7x | ~1.1 | - |

<iframe width="432" height="267" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=160655426&amp;format=interactive"></iframe>

|      Keccak256       | [Tachyon](https://github.com/kroma-network/tachyon) | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| :------------------: | :-------------------------------------------------: | :-----------------------------------------------: | :-------------------------------------------------------: | :-----------------------------------------: |
|       Average        |                       1931 ms                       |                      2514 ms                      |                          1133 ms                          |                   3791 ms                   |
|        Stdev         |                       31.9 ms                       |                      75.2 ms                      |                          168 ms                           |                   58.6 ms                   |
| Comparing to snarkjs |                        ~1.9x                        |                       ~1.5x                       |           <font color="FFB546">**~3.3**</font>            |                      -                      |

<iframe width="472" height="263" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=222721960&amp;format=interactive"></iframe>

|         RSA          | [Tachyon](https://github.com/kroma-network/tachyon) | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| :------------------: | :-------------------------------------------------: | :-----------------------------------------------: | :-------------------------------------------------------: | :-----------------------------------------: |
|       Average        |                       2307 ms                       |                      2560 ms                      |                          2530 ms                          |                   5504 ms                   |
|        Stdev         |                       18.7 ms                       |                      21.3 ms                      |                         266.1 ms                          |                   69.3 ms                   |
| Comparing to snarkjs |        <font color="FFB546">**~2.3**</font>         |                       ~2.1x                       |                           ~2.1x                           |                      -                      |

<iframe width="472" height="279" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=1443184132&amp;format=interactive"></iframe>

**Details:** [Spreadsheet of Circom benchmark (macOS)](https://docs.google.com/spreadsheets/d/1irKg_TOP-yXms8igwCN_3OjVrtFe5gTHkuF0RbrVuho/edit?usp=sharing)

## Halo2

In summary: <br/>
The performance of the Mopro build is comparable to the native Halo2 build. <br/>

The bellow tests were run on a Macbook Pro M1 Pro (2021) as well as an iPhone 15 Pro (2023).

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
