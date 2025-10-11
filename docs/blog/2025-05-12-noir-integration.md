---
slug: noir-integraion
title: "Mopro x Noir: Powering Mobile Zero-Knowledge Proofs"
authors:
  name: Vivian Jeng
  title: Developer on the Mopro Team
  url: https://github.com/vivianjeng
  image_url: https://github.com/vivianjeng.png
tags: [noir]
---

## Introduction

[Noir](https://noir-lang.org/docs) has been gaining significant traction recently, and for good reason. It's renowned for its elegant circuit frontend design and high-performance backend. With robust SDKs that make building web-based zero-knowledge apps seamless—and the ability to verify proofs on-chain—Noir stands out as a powerful and promising framework for developing modern ZKP applications.

However, while Noir offers strong support for web-based applications, native mobile support is still limited within the ecosystem. Most existing resources for mobile development have been contributed by the [zkPassport](https://github.com/zkpassport) team, including key projects like

- [noir_rs](https://github.com/zkpassport/noir_rs)
- [Swoir](https://github.com/Swoir/Swoir)
- [noir_android](https://github.com/madztheo/noir_android).
- [noir-react-native-starter](https://github.com/madztheo/noir-react-native-starter)

Our work is deeply inspired by the zkPassport team’s contributions, but builds upon them with significant improvements and optimizations.

In this article, we’ll walk through how we integrated Noir into the Mopro project, highlighting the key challenges we faced and the opportunities we uncovered along the way. We'll also introduce [Stealthnote Mobile](https://github.com/vivianjeng/stealthnote-mobile), a fully native mobile application built with Noir, to demonstrate the potential and performance of running zero-knowledge proofs natively on mobile devices.

## Build `noir-rs`

Like many apps with Mopro, our journey began with building a Rust crate. We started by adopting [`noir_rs`](https://github.com/zkpassport/noir_rs) from the zkPassport team. However, we quickly ran into some major challenges: **compiling the Barretenberg backend** — the proving system used by most Noir projects — could take up to 20 minutes. On top of that, the build process required a very _specific developer environment_, and _lacked caching_, meaning the entire binary had to be rebuilt from scratch even after a successful build.

To address these issues, we introduced a solution in

- `noir-rs`: https://github.com/zkmopro/noir-rs

by _prebuilding the backend binaries_ and hosting them on a server. During the build process, these binaries are downloaded automatically, eliminating the need for local compilation. This approach drastically reduces build time—from 20 minutes to just **8 seconds**—and removes the need for any special environment setup. In fact, you can see from our [GitHub Actions YAML file](https://github.com/zkmopro/noir-rs/blob/main/.github/workflows/test.yml) that **no additional dependencies are required**.

### Suggestions

Currently, the build process for the backend happens locally, which makes it non-reproducible and difficult to upgrade in CI environments like GitHub Actions. To improve this, we believe the build logic should be moved into CI. However, this process is quite complex and largely duplicated across repositories like the Noir and Aztec packages (See: [publish-bb.yml](https://github.com/AztecProtocol/aztec-packages/blob/46c2ad0b551a37e74118a789a1ea32a2daa1f849/.github/workflows/publish-bb.yml)).

Our proposal is that the Noir and Aztec teams consolidate this effort by **building `libbarretenberg.a` within the same CI pipeline and releasing them**. Since [`bb`](https://noir-lang.org/docs/dev/getting_started/quick_start#proving-backend-1) depends on these static libraries, they would naturally be compiled as part of that process. These prebuilt artifacts could then be published alongside the bb binary. This would allow downstream consumers like `noir-rs` to directly use the published binaries, eliminating the need to maintain custom build scripts or host binaries separately.

:::note

`bb` is a CLI tool that generates proofs directly in the user's terminal. Underneath, it relies on `libbarretenberg.a`—a static library that exposes low-level function APIs for proof generation, e.g.

```cpp
void acir_prove_ultra_honk(uint8_t const* acir_vec,
                           bool const* recursive,
                           uint8_t const* witness_vec,
                           uint8_t** out)
```

When properly linked, this library can be used directly within a Rust program, enabling seamless integration of the proving system into native applications.

:::

## Go Mobile-Native

Once we have the Rust crate ready, integrating it into a Mopro project allows us to easily generate bindings for both iOS and Android. Mopro significantly improves the mobile integration experience in the following ways:

1. **No additional Swift or Kotlin bindings required**

   Unlike the zkPassport team's approach—where they built separate [`Swoir`](https://github.com/Swoir/Swoir) and [`noir_android`](https://github.com/madztheo/noir_android) libraries with custom domain-specific interfaces—Mopro leverages [uniffi](https://github.com/mozilla/uniffi-rs) to automatically generate language bindings. This means developers don’t need to manually maintain Swift [^1] or Kotlin wrappers [^2]; they can directly import and use the generated bindings in their mobile codebases. e.g.

   For swift:

   ```swift
   import moproFFI

   let proofData = try! generateNoirProof(circuitPath: zkemailCircuitPath, srsPath: zkemailSrsPath, inputs: inputs)
   ```

   For kotlin:

   ```kotlin
   import uniffi.mopro.generateNoirProof

   let proofData = generateNoirProof(circuitFile, srsPath, inputs)
   ```

2. **Framework-Agnostic Design**

   Mopro is not tied to any specific mobile framework. By defining reusable templates for each target framework, developers are free to choose the environment they’re most comfortable with. Currently, Mopro supports _native iOS (Xcode + Swift)_, _native Android (Android Studio + Kotlin)_, as well as cross-platform frameworks like _React Native_ and _Flutter_—offering maximum flexibility and accessibility for diverse developer needs.

### Mopro Support

We’ve successfully integrated Noir proving into both Mopro-FFI and the Mopro CLI. You can now install the Mopro CLI and follow the steps in the [Getting Started](/docs/getting-started) guide to create a Noir project. Simply replace the SRS and circuit files with your own, and provide your custom circuit input — that’s it!

We’ve also provided an example zkEmail repository featuring a Noir circuit

- mopro-zkemail-nr: https://github.com/zkmopro/mopro-zkemail-nr

along with a NoirHack workshop video.

<p align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/UrT2x3JSKFg?si=OdRMi93vIBu9lSRn" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</p>

Feel free to explore the repo, clone it, and follow along to learn how to use Noir with Mopro in a real-world application.

### Challenge: Cross-platform support is limited

Our current implementation draws heavily from the zkPassport team’s work, which currently supports iOS devices and ARM64 Android devices/emulators. However, there are limitations: iOS simulators—essential for efficient development and testing—are not supported, and many Android developers (especially those using Windows with WSL) rely on x86_64 emulators. Even CI environments like GitHub Actions commonly use x86_64 Android emulators.

| Platforms | Current support target        | Support |
| --------- | ----------------------------- | ------- |
| iOS       | `aarch64-apple-ios`           | ✅      |
| iOS       | `aarch64-apple-ios-sim`       | ✅      |
| iOS       | `x86_64-apple-ios`            | ✅      |
| Android   | `x86_64-linux-android`        | ✅      |
| Android   | `i686-linux-android`          | ❌      |
| Android   | `armv7-linux-androideabi`     | ❌      |
| Android   | `aarch64-linux-androids`      | ✅      |
| MacOS     | `stable-aarch64-apple-darwin` | ✅      |
| Linux     | `x86_64-unknown-linux-gnu`    | ✅      |
| Windows   | `x86_64-pc-windows-msvc`      | ❌      |

Expanding support to these platforms is a significant challenge. The `barretenberg` backend, written in C++ and built with CMake, is large and complex, making cross-compilation non-trivial. While we’re evaluating the effort required to support additional architectures, we’re also hopeful that the Noir or zkPassport teams may address this gap in the future.

## Case Study: Stealthnote Mobile App

During the NoirHack 2025 (April 14th to May 10th), the Mopro team participated by building a mobile-native Noir application. Our project was inspired by [**Stealthnote**](https://stealthnote.xyz/), originally created by Saleel. Stealthnote is a web-based app that allows users to sign in with Google OAuth and prove ownership of their organizational email address. It leverages a Noir circuit to generate a zero-knowledge proof from the JWT issued by Google OAuth. You can read more about the original project in [Saleel’s blog post](https://saleel.xyz/blog/stealthnote/).

We aim to enhance performance and user experience by building a fully native mobile app. At the same time, we want to demonstrate that the current Mopro + Noir stack is fully capable of supporting mobile-native development. Therefore, we decided to build a mobile version of Stealthnote.

### What We Built

During the hackathon, we developed

- Stealthnote Mobile: https://github.com/vivianjeng/stealthnote-mobile
- Testflight for iOS: [download](https://testflight.apple.com/join/8hqYwe8C)
- Android APK: [download](https://drive.google.com/file/d/1IMsH0fBpaLGkFgFX0oqnlS6LQk3WCr3t/view?usp=sharing)

A mobile-native frontend for the original Stealthnote project. To maintain compatibility, we kept the original Noir circuits and backend logic intact, focusing entirely on building the mobile frontend. Even so, creating a mobile frontend involved tackling many challenges across platforms.

We chose Flutter as our cross-platform framework because of its fast build times compared to React Native and its rich ecosystem of packages for mobile functionalities—such as Google authentication, biometric login, camera access, gallery browsing, and file storage. In our view, _Flutter is currently the best choice for building high-performance, cross-platform apps_.

Our architecture separates responsibilities clearly:

- **Flutter** handles:

  1. UI/UX design
  2. Google OAuth authentication
  3. Backend API communication

- **Rust** handles:

  1. Parsing the JWT and converting it into the structured inputs required by the Noir circuit
  2. Generating and verifying Noir proofs
  3. Ephemeral key generation using Ed25519 and Poseidon2
     <br/>

  :::info
  To learn how to integrate Mopro Rust bindings into a Flutter project, please refer to the

  - [Flutter Setup](/docs/setup/flutter-setup) section
  - [flutter-app](https://github.com/zkmopro/flutter-app) example
  - [mopro_flutter_package](https://github.com/zkmopro/mopro_flutter_package) plugin example

  :::

This clear separation lets us leverage the strengths of both technologies—Flutter’s frontend speed and flexibility, and Rust’s performance and security for core cryptographic logic.

### Benchmark

While the Mopro team anticipated the outcome, the benchmark results are still notable: a mobile-native prover delivers performance up to **10 times faster** than running the same proof in a browser environment.

| JWT Operation              | Prove    | Verify  |
| -------------------------- | -------- | ------- |
| Browser                    | 37.292 s | 0.286 s |
| Desktop (Mac M1 Pro)       | 2.02 s   | 0.007 s |
| Android emulator (Pixel 8) | 4.786 s  | 3.013 s |
| iPhone 16 Pro              | 2.626 s  | 1.727 s |

### Challenge: Insufficient Rust tooling and SDKs.

One of the main challenges we faced was converting all the TypeScript functions used in StealthNote into their Rust equivalents. For standard cryptographic operations like Ed25519 signatures, this was relatively straightforward — Rust has mature libraries we could rely on.

However, **Poseidon2** presented a bigger challenge. Unlike the original Poseidon hash, Poseidon2 is a newer variant used in the Noir circuits. Unfortunately, there was no direct Rust implementation available. The Noir team primarily supports a TypeScript version via [`bb.js`](https://www.npmjs.com/package/@aztec/bb.js), which is a wrapper around Barretenberg's C++ implementation compiled to WASM. This made it hard to fully understand the logic and port it to Rust.

Eventually, we located the [Poseidon2 Noir circuit](https://github.com/noir-lang/poseidon/blob/master/src/poseidon2.nr) and its [permutation logic](https://github.com/noir-lang/noir/blob/master/acvm-repo/bn254_blackbox_solver/src/poseidon2.rs) in the noir-lang repository. Using that as a reference, we implemented our own version of

- poseidon2.rs in Rust: https://github.com/vivianjeng/stealthnote-mobile/blob/main/src/proof/poseidon2.rs

aligning it with the Noir circuit to ensure compatibility.

### Suggestions

Since Barretenberg already provides a C++ implementation for most Noir circuit functions, it would actually be more efficient and sustainable for the Noir or Aztec team to maintain a native C++ or Rust SDK for Rust developers — rather than focusing primarily on the JavaScript ([`bb.js`](https://www.npmjs.com/package/@aztec/bb.js)) interface.

As we suggested earlier, an ideal solution would be for the team to publish precompiled static binaries (e.g. `libbarretenberg.a`) alongside versioned releases. This way, other developers — like us — could easily integrate these binaries into their Rust projects using proper FFI bindings.

This approach would remove the need for every developer to rebuild Barretenberg from source or maintain their own custom builds. For example, our [`bb.rs`](https://github.com/zkmopro/noir-rs/tree/main/bb) project demonstrates this idea, but it still lacks many of the standard library features available in bb.js, such as: poseidon2, blake2, AES encryption/decryption. Providing official support for these as native libraries would greatly improve the developer experience and adoption for building performant, **mobile-native** or **server-side** ZK applications.

## Conclusion

We began integrating Noir with Mopro in early April, and the process progressed smoothly — thanks in large part to the foundational work done by the zkPassport team. In contrast to our experience with the Circom prover (which took several months to make mobile-compatible), Noir integration was faster and more straightforward.

However, we also discovered significant gaps in Noir’s current support for mobile-native development. While the prover itself is crucial, a complete SDK ecosystem is equally important. At the moment, teams aiming to build mobile-native apps — like zkPassport — must develop many of the components from scratch, rather than being able to rely on official tooling or prebuilt libraries from the Noir team.

This raises a challenge: _what if a project doesn’t have the expertise, time, or resources to build and maintain a mobile-native infrastructure?_ These barriers can prevent great ideas from becoming usable apps.

There’s still a long road ahead — for both the Mopro team and the Noir ecosystem — to provide a full suite of mobile-native infrastructure that can empower ZK developers, whether they're building the next zkPassport or an entirely new kind of app.

Finally, we want to encourage more developers to explore building mobile-native apps. These platforms offer better performance, deeper device integration, and more seamless user experiences — which are essential if you want your ZK project to truly reach users.

## Contribution

All kinds of contributions are welcome! 🎉

Feel free to check out the current issues on the [Mopro GitHub repository](https://github.com/zkmopro/mopro/issues), or open a new issue if you notice something missing or have ideas to improve the project.

Here are some current Noir-related issues worth exploring:

- [[Noir] Download SRS script #423](https://github.com/zkmopro/mopro/issues/423)
- [[Noir] separating proof bytes and public signals bytes #422](https://github.com/zkmopro/mopro/issues/422)
- [[Noir] React native template update #410](https://github.com/zkmopro/mopro/issues/410)
- [[Noir] Flutter template update #409](https://github.com/zkmopro/mopro/issues/409)

Feel free to reach out to the Mopro team on Telegram [@zkmopro](https://t.me/zkmopro) for questions or support, and follow us on X (formerly Twitter) [@zkmopro](https://x.com/zkmopro) to stay up to date with our latest progress!

[^1]: Noir Swift wrapper: https://github.com/Swoir/Swoir/blob/main/Sources/Swoir/Circuit.swift
[^2]: Noir Kotlin wrapper: https://github.com/madztheo/noir_android/blob/main/lib/src/main/java/com/noirandroid/lib/Circuit.kt
