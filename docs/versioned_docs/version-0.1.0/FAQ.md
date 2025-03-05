---
sidebar_position: 7
---

# FAQ

## What are the design goals of Mopro?

1. Modularity
2. Developer-friendly
3. Performance
4. Multi-platform

See one of the recent [talks](/docs/community) for more details.

## What proof systems does Mopro support?

Currently Circom/Groth16, Halo2 and Ashlang, but due to its modular architecture it is easy to add support for new proof systems.

There's experimental support for GKR in this [PR](https://github.com/zkmopro/mopro/pull/241/files).

We welcome people to contribute support for [more proof systems](https://github.com/zkmopro/mopro/issues/15).

## What platforms does Mopro support?

Mopro is multi-platform and aims to support as many platforms as possible. iOS, Android and Desktop (through Rust/CLI) are the main platforms supported. *Website support is currently under development.* (See [#202](https://github.com/zkmopro/mopro/issues/202))

Please refer to the [Getting Started](getting-started.md) guide for instructions on integrating mopro into iOS and Android apps, as well as examples for [React Native](https://github.com/zkmopro/react-native-app) and [Flutter](https://github.com/zkmopro/flutter-app) applications.

## Is Mopro just for verifying proofs on mobile?

Mopro is for both proving and verifying ZKPs on mobile.

## Does Mopro run natively on a phone?

Yes. Witness and proof generation happen natively in the app.