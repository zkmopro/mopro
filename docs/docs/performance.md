---
sidebar_position: 5
---

# Performance and Benchmarks

In summary: <br/>
Both native witness generation and proof generation are generally faster than `snarkjs` in the browser, with potential speed improvements of up to **20 times**. <br/>
However, performance varies across different circuits.
We _recommend_ developers benchmark their custom circuits before selecting tools for app development.

:::warning
- [witnesscalc](https://github.com/0xPolygonID/witnesscalc) hasn't been integrated in mopro. See [zkmopro/mopro#110](https://github.com/zkmopro/mopro/issues/110).
- [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) is not fully compatible with circom circuits. See: [zkmopro/mopro#32](https://github.com/zkmopro/mopro/issues/32).
- [wasmer](https://github.com/arkworks-rs/circom-compat) doesn't work in iOS. See: [zkmopro/mopro#109](https://github.com/zkmopro/mopro/issues/109).
:::

## iOS

Benchmarks on an iPhone 12 mini (2020).

<details>
  <summary>Witness generation</summary>

| SHA256 | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | :--: |
| Average | 22.3 ms | 36.1 ms | 476.1 ms | 90.3 ms | 163.5 ms |
| Stdev | 1.2 ms | 0.3 ms | 27.8 ms | 1.2 ms | 6.7 ms |
| Comparing to snarkjs | <font color="FFB546">**~7x**</font> | ~4.5x | ~(-3)x | ~1.8 | - |

<iframe width="552" height="257" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vTbLHpEzT1ybhta5NVQrDQuOLwgGzLGpm2amiWgLRu0l9ZvXMNtNZ-DWIlySL0zO30UWn_nZvkfaQWY/pubchart?oid=745835114&amp;format=interactive"></iframe>

| Keccak256 | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | :--: |
| Average | 144.7 ms | 26.2 ms | 440.7 ms | 160.7 ms | 257.1 ms |
| Stdev | 1.8 ms | 4.5 ms | 10.4 ms | 3.3 ms | 4.1 ms |
| Comparing to snarkjs | ~1.8x | <font color="FFB546">**~10x**</font> | ~(-1.7)x | ~1.6x | - |

<iframe width="541" height="259" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vTbLHpEzT1ybhta5NVQrDQuOLwgGzLGpm2amiWgLRu0l9ZvXMNtNZ-DWIlySL0zO30UWn_nZvkfaQWY/pubchart?oid=1466340585&amp;format=interactive"></iframe>

| RSA | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | :--: |
| Average | 208.5 ms | 513.1 ms | 5488 ms | 3861 ms | 5421 ms |
| Stdev | 5.8 ms | 11.2 ms | 47.8 ms | 10.8 ms | 9.9 ms |
| Comparing to snarkjs | <font color="FFB546">**~26x**</font> | ~10x | ~(-1)x | ~1.4x | - |

<iframe width="537" height="256" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vTbLHpEzT1ybhta5NVQrDQuOLwgGzLGpm2amiWgLRu0l9ZvXMNtNZ-DWIlySL0zO30UWn_nZvkfaQWY/pubchart?oid=1593466084&amp;format=interactive"></iframe>
</details>

<details>
  <summary>Proof generation</summary>

| SHA256 | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | 
| Average | 795.2 ms | 550.4 ms | 2374.1 ms |
| Stdev | 17.2 ms | 27.2 ms | 62.9 ms |
| Comparing to snarkjs | ~3x | <font color="FFB546">**~4.3x**</font> | - |

<iframe width="467" height="269" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vTbLHpEzT1ybhta5NVQrDQuOLwgGzLGpm2amiWgLRu0l9ZvXMNtNZ-DWIlySL0zO30UWn_nZvkfaQWY/pubchart?oid=127729877&amp;format=interactive"></iframe>

| Keccak256 | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | 
| Average | 2647.9 ms | 1221.1 ms | 8149.1 ms |
| Stdev | 14.4 ms | 42.7 ms | 283.575 ms |
| Comparing to snarkjs | ~3x | <font color="FFB546">**~6.7x**</font> | - |

<iframe width="495" height="252" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vTbLHpEzT1ybhta5NVQrDQuOLwgGzLGpm2amiWgLRu0l9ZvXMNtNZ-DWIlySL0zO30UWn_nZvkfaQWY/pubchart?oid=333122430&amp;format=interactive"></iframe>

| RSA | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | 
| Average | 2908.6 ms | 2324.4 ms | 10304.8 ms |
| Stdev | 112.9 ms | 67.1 ms | 605.5 ms |
| Comparing to snarkjs | ~3.5x | <font color="FFB546">**~4.4x**</font> | - |

<iframe width="484" height="266" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vTbLHpEzT1ybhta5NVQrDQuOLwgGzLGpm2amiWgLRu0l9ZvXMNtNZ-DWIlySL0zO30UWn_nZvkfaQWY/pubchart?oid=1171109874&amp;format=interactive"></iframe>
</details>

**Details:** [Spreadsheet of Circom benchmark (iOS)](https://docs.google.com/spreadsheets/d/1MFABmsYSUsWDmhbjleqhBXk7nkYwhu589yK-CHtRkNI/edit?usp=sharing)

:::note
- [Tachyon](https://github.com/kroma-network/tachyon) performs well in [macOS](#macos), but we haven't integrated it in mobile. See [zkmopro/mopro#143](https://github.com/zkmopro/mopro/issues/143)
:::

## Android

TBD

## macOS

Benchmarks on an Macbook Pro M1 Max (2021).

<details>
  <summary>Witness generation</summary>

| SHA256 | [Tachyon](https://github.com/kroma-network/tachyon) | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Average | 3.2 ms | 22.2 ms | 42.8 ms | 454.5 ms | 88.8 ms | 132.8 ms | 
| Stdev | 0.2 ms | 5.2 ms | 2.2 ms | 26.7 ms | 1.0 ms | 1.3 ms |
| Comparing to snarkjs | <font color="FFB546">**~41x**</font> | ~6x | ~3x | ~(-3.4)x | ~1.5x | - |

<iframe width="466" height="254" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=2079338651&amp;format=interactive"></iframe>

| Keccak256 | [Tachyon](https://github.com/kroma-network/tachyon) | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Average | 9.8 ms | 72.3 ms | 14.1 ms | 447.1 ms | 169 ms | 234.6 ms | 
| Stdev | 0.4 ms | 7.7 ms | 0.8 ms | 5.9 ms | 2.0 ms | 3.2 ms |
| Comparing to snarkjs | <font color="FFB546">**~23x**</font> | ~3x | ~16x | ~(-1.9)x | x1.4x | - |

<iframe width="530" height="245" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=129836159&amp;format=interactive"></iframe>

| RSA | [Tachyon](https://github.com/kroma-network/tachyon) | [witnesscalc](https://github.com/0xPolygonID/witnesscalc) | [circom-witness-rs](https://github.com/philsippl/circom-witness-rs) | [wasmer](https://github.com/arkworks-rs/circom-compat) | [w2c](https://github.com/vimwitch/rust-witness) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Average | 4.3 ms | 167.6 ms | 522.9 ms | 5109 ms | 3847.2 ms | 4638.8 ms | 
| Stdev | 0.2 ms | 7.5 ms | 7.7 ms | 25.1 ms | 61.3 ms | 32.4 ms |
| Comparing to snarkjs | <font color="FFB546">**~1078x**</font> | ~27x | ~8.8x | ~(-1.1)x | ~1.2x | - |

<iframe width="552" height="259" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=1347845461&amp;format=interactive"></iframe>
</details>


<details>
  <summary>Proof generation</summary>
| SHA256 | [Tachyon](https://github.com/kroma-network/tachyon) | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | 
| Average | 385.2 ms | 773.8 ms | 1137.3 ms | 1350.4 ms |
| Stdev | 3.6 ms | 17 ms | 127 ms | 26 ms |
| Comparing to snarkjs | <font color="FFB546">**~3.5x**</font> | ~1.7x | ~1.1 | - |

<iframe width="432" height="267" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=160655426&amp;format=interactive"></iframe>

| Keccak256 | [Tachyon](https://github.com/kroma-network/tachyon) | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | 
| Average | 1365 ms | 2514 ms | 1133 ms | 3791 ms |
| Stdev | 11.6 ms | 75.2 ms | 168 ms | 58.6 ms |
| Comparing to snarkjs | **~2.7x** | ~1.5x | <font color="FFB546">**~3.3**</font> | - |

<iframe width="472" height="263" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=222721960&amp;format=interactive"></iframe>

| RSA | [Tachyon](https://github.com/kroma-network/tachyon) | [rapidsnark](https://github.com/iden3/rapidsnark) | [ark-works](https://github.com/arkworks-rs/circom-compat) | [snarkjs](https://github.com/iden3/snarkjs) |
| :--: | :--: | :--: | :--: | :--: | 
| Average | 1665 ms | 2560 ms | 2530 ms | 5504 ms |
| Stdev | 11.2 ms | 21.3 ms | 266.1 ms | 69.3 ms |
| Comparing to snarkjs | <font color="FFB546">**~3.3**</font> | ~2.1x | ~2.1x | - |

<iframe width="472" height="279" seamless frameborder="0" scrolling="no" src="https://docs.google.com/spreadsheets/d/e/2PACX-1vQOuS5abdzvh6znXORvSo7M85ubmDpSmE3C1Zs_l56wd25lMK4FZPEWOaCp7WrOlIjc3jEcWa0lfiy9/pubchart?oid=1443184132&amp;format=interactive"></iframe>
</details>

**Details:** [Spreadsheet of Circom benchmark (macOS)](https://docs.google.com/spreadsheets/d/1irKg_TOP-yXms8igwCN_3OjVrtFe5gTHkuF0RbrVuho/edit?usp=sharing)
