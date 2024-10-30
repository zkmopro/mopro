# mopro

![Crates.io](https://img.shields.io/crates/v/mopro-ffi?label=mopro-ffi&style=flat-square)

Mopro is a toolkit for ZK app development on mobile. Mopro makes client-side proving on mobile simple.

To learn more about mopro, please refer to the documentation at [zkmopro](https://zkmopro.org/docs/intro).

## Getting started

- Make sure you've installed the [prerequisites](https://zkmopro.org/docs/prerequisites).
- Getting started with this [tutorial](https://zkmopro.org/docs/getting-started).

## Run tests

- circom
  ```sh
  cd mopro-ffi
  cargo test --features circom
  ```
- halo2
  ```sh
  cd mopro-ffi
  cargo test --features halo2
  ```
- circom-e2e
  ```sh
  cd test-e2e
  curl -L https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar -o jna-5.13.0.jar
  CLASSPATH=jna-5.13.0.jar cargo test --test circom -- --nocapture
  ```
- halo2-e2e
  ```sh
  cd test-e2e
  curl -L https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar -o jna-5.13.0.jar
  CLASSPATH=jna-5.13.0.jar cargo test --test halo2 -- --nocapture
  ```

## Community

- X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
- Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>

## Acknowledgements

This work was initially sponsored by a joint grant from [PSE](https://pse.dev/) and [0xPARC](https://0xparc.org/). It is currently incubated by PSE.