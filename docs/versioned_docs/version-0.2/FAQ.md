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

Currently

-   Circom/Groth16(BN254 and BLS12-381)
-   Halo2/Plonkish
-   Noir/Barretenberg

but due to its modular architecture it is easy to add support for new proof systems.

-   GKR: https://github.com/zkmopro/mopro/pull/241/files
-   Binius: https://github.com/vivianjeng/binius-sha256
-   Spartan: https://github.com/zkmopro/mopro/pull/244
-   Nova Scotia: https://github.com/zkmopro/mopro/pull/240

Please checkout [Customize the bindings](setup/rust-setup.md#-customize-the-bindings) to learn how to add a new proving system.
We welcome people to contribute support for [more proof systems](https://github.com/zkmopro/mopro/issues/15).

## What platforms does Mopro support?

Mopro is multi-platform and aims to support as many platforms as possible. iOS, Android, WASM and Desktop (through Rust/CLI) are the main platforms supported.

Please refer to the [Getting Started](getting-started.md) guide for instructions on integrating mopro into iOS and Android apps, as well as examples for [React Native](https://github.com/zkmopro/react-native-app), [Flutter](https://github.com/zkmopro/flutter-app) and [WASM](setup/web-wasm-setup.md) applications.

## Is Mopro just for verifying proofs on mobile?

Mopro is for both proving and verifying ZKPs on mobile.

## Does Mopro run natively on a phone?

Yes. Witness and proof generation happen natively in the app.

## What suggestions for developers looking to build apps that work on both iOS and Android?

**React Native** or **Flutter**

If you're more familiar with TypeScript or JavaScript, **React Native** might feel more natural.

On the other hand, **Flutter** generally offers faster development performance and a smoother UI experience out of the box.

Both are great choices—just pick the one that best fits your team's experience and needs.

## If developers just want to try out Mopro, what's the easiest way to get started?

We recommend starting with **Swift (iOS)** or **Kotlin (Android)**.

They allow you to directly interact with the native module and are simpler to set up for quick experimentation.

Using React Native or Flutter requires additional setup: you'll need to define and bridge APIs across multiple layers—TypeScript (or Dart), the iOS module, and the Android module. This adds complexity, especially for quick prototyping.

## How can I access mobile hardware (e.g., camera, biometrics, GPS)?

To access mobile hardware features, you’ll need to use a mobile development framework or native language such as **Swift**, **Kotlin**, **React Native**, or **Flutter**.
Mopro focuses primarily on **computation and zero-knowledge proving**—it doesn’t provide direct APIs for hardware access.

## How can I get support from the Mopro team?

You’re welcome to join our [Telegram group](https://t.me/zkmopro) or follow us on [X (Twitter)](https://x.com/zkmopros) for updates and community support.
For bug reports or technical issues, please open an issue on [GitHub](https://github.com/zkmopro/mopro/issues).

## How to contribute to Mopro?

Mopro is an open-source project, and contributions are always welcome!
To get started, simply fork the repository and submit a pull request (PR) with your changes. We’ll review it as soon as possible.
You can also check out the [open issues](https://github.com/zkmopro/mopro/issues) to see where help is needed.

:::warning
We do not accept minor grammatical fixes (e.g., correcting typos, rewording sentences) unless they significantly improve clarity in technical documentation. These contributions, while appreciated, are not a priority for merging. If there is a grammatical error feel free to message the team.
:::

## I already have a ZK web app. How can I use Mopro?

If you want to bring your ZK web app to mobile, you’ll need to build a native mobile app using one of the supported frameworks: **Swift**, **Kotlin**, **React Native**, or **Flutter**. Mopro provides the tools to integrate ZK proving into these platforms.

Alternatively, you can embed a WebView inside your native mobile app—this allows you to keep your existing frontend while leveraging Mopro’s native provers underneath to boost proving performance.

If you're focused on web performance, you can use wasm-bindgen to compile your Rust-based prover to WebAssembly for use in browsers. In the future, we also plan to explore **WebGPU** support to further accelerate ZK proving in web environments.
