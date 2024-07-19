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

Currently Circom/Groth16, but due to its modular architecture it is easy to add support for new proof systems.

There's experimental support for Kimchi, a Plonkish proof system, that was done during a hackathon in this [PR](https://github.com/zkmopro/mopro/pull/34).

There's a grantee working on adding Halo2 support. Please see the [Telegram group](https://t.me/zkmopro) for more information.

We welcome people to contribute support for [more proof systems](https://github.com/zkmopro/mopro/issues/15).

## What platforms does Mopro support?

Mopro is multi-platform and aims to support as many platforms as possible. iOS, Android and Desktop (through Rust/CLI) are the main platforms supported.

There's also very experimental React Native support [here](https://github.com/anon-aadhaar/anon-aadhaar-react-native/commit/d6443316200cd3e1f17ad2679458cc6e6e9fe1f2). We aim to make this easier to consume.

We welcome people to contribute support for [more platforms](https://github.com/zkmopro/mopro/issues/16).

## Is Mopro just for verifying proofs on mobile?

Mopro is for both proving and verifying ZKPs on mobile.

## Does Mopro run natively on a phone?

Yes. Witness and proof generation happens natively in app.