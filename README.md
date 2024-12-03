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
    <th>Circuits</th>
    <th>snarkjs groth16 fullprove</th>
    <th>mopro (rust-witness + ark-works)</th>
    <th>witnesscalc + rapidsnark</th>
  </tr>
  <tr>
    <td>Keccak256</td>
    <td>8406.2 ms</td>
    <td>1381.8 ms (~6x)</td>
    <td>2792.6 ms (~3x)</td>
  </tr>
  <tr>
    <td>SHA256</td>
    <td>2537.6 ms</td>
    <td>640.7 ms (~4x)</td>
    <td>817.5 ms (~3.1x)</td>
  </tr>
  <tr>
    <td>RSA</td>
    <td>15.7 s</td>
    <td>6.1 s (~2.5x)</td>
    <td>3.1 s (~5x)</td>
  </tr>
  <tr>
    <td>Semaphore</td>
    <td>902 ms</td>
    <td>257 ms (~3.5x)</td>
    <td>347 ms (~2.5x)</td>
  </tr>
  <tr>
    <td>Anon Aadhaar</td>
    <td>26 s</td>
    <td>17 s (~1.5x)</td>
    <td>11 s (~2.3x)</td>
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
