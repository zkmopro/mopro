---
sidebar_position: 1
---

# 🛠️ Introducing Mopro: The Mobile Prover Toolkit

**Mopro** is a developer toolkit designed to make building **mobile-native ZK apps** easier, faster, and more accessible. Whether you're a ZK protocol author, a mobile app developer, or a Rust engineer exploring zero-knowledge proofs, Mopro provides a streamlined workflow to bring your ideas to mobile devices.

## 🚀 What Is Mopro?

Mopro simplifies the development of mobile-native apps by offering:

-   A powerful **CLI** to scaffold, build, and update projects

-   Prebuilt **templates** for iOS, Android, React Native, and Flutter

-   Clear, step-by-step **documentation** to guide developers at every stage

Our goal is to remove the friction from mobile ZK app development—so you can focus on your product, not the plumbing.

## 🔧 How It Works

Mopro takes each proving system and wraps it as an **adapter** written in Rust. These adapters provide a unified interface for ZK proof generation regardless of the backend (e.g., Circom, Noir, Halo2). Mopro then uses [UniFFI](https://github.com/mozilla/uniffi-rs) to generate native bindings for **Swift (iOS)** and **Kotlin (Android)** from Rust code. These bindings can then be reused in cross-platform frameworks like **React Native** and **Flutter**, with ready-to-use templates for each platform.

## 👩‍💻 Who It's For

### 📱 ZK Mobile App Developers

Get a full-stack monorepo template that handles Rust bindings, mobile UIs, and proof generation. Just follow the `mopro` CLI to bootstrap your app from zero to working prototype. See [**Getting Started**](getting-started).

### 🔐 ZK Protocol Developers

Don't want to maintain a full app? No problem. Mopro helps you ship production-ready **mobile SDKs** for your protocol, making it easier for others to integrate your tech.

See Mopro Bindings for Multiplatform

-   [`mopro-kotlin-package`](https://github.com/zkmopro/mopro-kotlin-package): Kotlin bindings for Android.
-   [`mopro-swift-package`](https://github.com/zkmopro/mopro-swift-package): Swift bindings for iOS.
-   [`mopro-react-native-package`](https://github.com/zkmopro/mopro-react-native-package): A React Native wrapper.
-   [`mopro_flutter_package`](https://github.com/zkmopro/mopro_flutter_package): Flutter bindings for Dart-based apps.

### 📲 Mobile Developers

Easily consume ZK SDKs via familiar package managers like **CocoaPods**, **Gradle**, **npm**, or **pub.dev**. No Rust knowledge required.

### 🦀 Rust Developers

Mopro supports various ZK backends—even those not originally written in Rust—via **wrapper** crates.

Examples include

-   [`circom-prover`](https://github.com/zkmopro/mopro/tree/main/circom-prover)
-   [`witnesscalc_adapter`](https://github.com/zkmopro/witnesscalc_adapter/tree/main/witnesscalc_adapter)
-   [`rust-rapidsnark`](https://github.com/zkmopro/rust-rapidsnark/tree/main)
-   [`noir-rs`](https://github.com/zkmopro/noir-rs).


## ⚡ Why Mobile-Native?

Mobile-native apps offer up to **10x performance improvement** over browser-based ZK apps. They also provide **smoother UX**, better integration with device features (e.g., biometric auth, secure storage), and offline-friendly capabilities—bringing your ZK protocol to a broader, mainstream audience.

See our [**benchmarks**](performance) for performance comparisons.

## ⚙️ GPU Acceleration

Mopro also focuses on **mobile-native GPU acceleration**, enabling client-side devices to leverage their GPUs to speed up operations like **MSM (Multi-Scalar Multiplication)** during proof generation. This significantly improves performance for ZK proving on mobile.

See implementation details and updates in [**gpu-acceleration**](https://github.com/zkmopro/gpu-acceleration).

## 📚 Learn More About Mopro

Explore the full ecosystem, documentation, and community resources:

-   📱 Main GitHub Repository: https://github.com/zkmopro/mopro

-   💡 Example Projects: https://zkmopro.org/docs/projects

-   💬 Community & Talks: https://zkmopro.org/docs/community

-   📰 Blog: https://zkmopro.org/blog
