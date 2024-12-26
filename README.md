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

<table>
  <tr>
    <th>Android (Samsung S23U)</th>
    <th>snarkjs groth16 fullprove</th>
    <th>mopro (rust-witness + ark-works)</th>
    <th>witnesscalc + rapidsnark</th>
  </tr>
  <tr>
    <td>Keccak256</td>
    <td>6.584 s</td>
    <td>2.55 s (~2.5x)</td>
    <td>0.867 s (~7.6x)</td>
  </tr>
  <tr>
    <td>SHA256</td>
    <td>2.499 s</td>
    <td>1.73 s (~1.4x)</td>
    <td>0.252 s (~9.9x)</td>
  </tr>
  <tr>
    <td>RSA</td>
    <td>11.684 s</td>
    <td>9.53 s (~1.2x)</td>
    <td>1.1 s (~10.6x)</td>
  </tr>
  <tr>
    <td>Semaphore</td>
    <td>1.162 s</td>
    <td>0.331 s (~3.5x)</td>
    <td>0.188 ms (~6x)</td>
  </tr>
  <tr>
    <td>Anon Aadhaar</td>
    <td>36.08 s</td>
    <td>30.29 s (~1x)</td>
    <td>3.883 s (~9.3x)</td>
  </tr>
</table>

<table>
  <tr>
    <th>iOS (iPhone 16 Pro)</th>
    <th>snarkjs groth16 fullprove</th>
    <th>mopro (rust-witness + ark-works)</th>
    <th>witnesscalc + rapidsnark</th>
  </tr>
  <tr>
    <td>Keccak256</td>
    <td>5.696 s</td>
    <td>1.167 s (~5x)</td>
    <td>0.751 s (~7.5x)</td>
  </tr>
  <tr>
    <td>SHA256</td>
    <td>1.694 s</td>
    <td>0.665 s (~2.5x)</td>
    <td>0.233 s (~7.2x)</td>
  </tr>
  <tr>
    <td>RSA</td>
    <td>9.794 s</td>
    <td>4.551 s (~2.1x)</td>
    <td>0.9 s (~10.8x)</td>
  </tr>
  <tr>
    <td>Semaphore</td>
    <td>1.18 s</td>
    <td>0.162 s (~7.2x)</td>
    <td>0.163 ms (~7.2x)</td>
  </tr>
  <tr>
    <td>Anon Aadhaar</td>
    <td>28.679 s</td>
    <td>8.369 s (~3.4x)</td>
    <td>3.432 s (~8.3x)</td>
  </tr>
</table>

### Halo2

The performance of the Mopro build is comparable to the native Halo2 build.

<table>
  <tr>
    <th>Circuits</th>
    <th>Native (M1 Pro)</th>
    <th>iPhone 15 Pro	</th>
  </tr>
  <tr>
    <td>Keccak256</td>
    <td>10.3 s</td>
    <td>11.0 s</td>
  </tr>
  <tr>
    <td>RSA</td>
    <td>76.5 s	</td>
    <td>crashes</td>
  </tr>
</table>

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.
