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

-   [noir_rs](https://github.com/zkpassport/noir_rs)
-   [Swoir](https://github.com/Swoir/Swoir)
-   [noir_android](https://github.com/madztheo/noir_android).

They've also provided a helpful [noir-react-native-starter](https://github.com/madztheo/noir-react-native-starter) to assist developers in building React Native apps with Noir. Our work is deeply inspired by the zkPassport team’s contributions, but builds upon them with significant improvements and optimizations.

In this article, we’ll walk through how we integrated Noir into the Mopro project, highlighting the key challenges we faced and the opportunities we uncovered along the way. We'll also introduce [Stealthnote Mobile](https://github.com/vivianjeng/stealthnote-mobile), a fully native mobile application built with Noir, to demonstrate the potential and performance of running zero-knowledge proofs natively on mobile devices.

## Build `noir-rs`

Like many apps with Mopro, our journey began with building a Rust crate. We started by adopting [`noir_rs`](https://github.com/zkpassport/noir_rs) from the zkPassport team. However, we quickly ran into some major challenges: **compiling the Barretenberg backend** — the proving system used by most Noir projects — could take up to 20 minutes. On top of that, the build process required a very _specific developer environment_, and _lacked caching_, meaning the entire binary had to be rebuilt from scratch even after a successful build.

To address these issues, we introduced a solution in

-   `noir-rs`: https://github.com/zkmopro/noir-rs

by _prebuilding the backend binaries_ and hosting them on a server. During the build process, these binaries are downloaded automatically, eliminating the need for local compilation. This approach drastically reduces build time—from 20 minutes to just **8 seconds**—and removes the need for any special environment setup. In fact, you can see from our [GitHub Actions YAML file](https://github.com/zkmopro/noir-rs/blob/main/.github/workflows/test.yml) that **no additional dependencies are required**.

### Additional Suggestions

Currently, the build process for the backend happens locally, which makes it non-reproducible and difficult to upgrade in CI environments like GitHub Actions. To improve this, we believe the build logic should be moved into CI. However, this process is quite complex and largely duplicated across repositories like the Noir and Aztec packages (See: [publish-bb.yml](https://github.com/AztecProtocol/aztec-packages/blob/46c2ad0b551a37e74118a789a1ea32a2daa1f849/.github/workflows/publish-bb.yml)).

Our proposal is that the Noir and Aztec teams consolidate this effort by **building `libbarretenberg.a` within a the same CI pipeline and releasing them**. Since [`bb`](https://noir-lang.org/docs/dev/getting_started/quick_start#proving-backend-1) depends on these static libraries, they would naturally be compiled as part of that process. These prebuilt artifacts could then be published alongside the bb binary. This would allow downstream consumers like `noir-rs` to directly use the published binaries, eliminating the need to maintain custom build scripts or host binaries separately.

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

    let proofBytes = generateNoirProof(circuitFile, srsPath, inputs)
    ```

2. **Framework-Agnostic Design**

    Mopro is not tied to any specific mobile framework. By defining reusable templates for each target framework, developers are free to choose the environment they’re most comfortable with. Currently, Mopro supports native iOS (Xcode + Swift), native Android (Android Studio + Kotlin), as well as cross-platform frameworks like React Native and Flutter—offering maximum flexibility and accessibility for diverse developer needs.

### Challenges

Our current implementation draws heavily from the zkPassport team’s work, which currently supports iOS devices and ARM64 Android devices/emulators. However, there are limitations: iOS simulators—essential for efficient development and testing—are not supported, and many Android developers (especially those using Windows with WSL) rely on x86_64 emulators. Even CI environments like GitHub Actions commonly use x86_64 Android emulators.

Expanding support to these platforms is a significant challenge. The `barretenberg backend`, written in C++ and built with CMake, is large and complex, making cross-compilation non-trivial. While we’re evaluating the effort required to support additional architectures, we’re also hopeful that the Noir or zkPassport teams may address this gap in the future.

[^1]: Noir Swift wrapper: https://github.com/Swoir/Swoir/blob/main/Sources/Swoir/Circuit.swift
[^2]: Noir Kotlin wrapper: https://github.com/madztheo/noir_android/blob/main/lib/src/main/java/com/noirandroid/lib/Circuit.kt
