# Architectures

Different platforms use different architectures. This section will guide you in selecting the appropriate architecture for your target platform.

## iOS

| Current support target  | Description                               | Examples               |
| ----------------------- | ----------------------------------------- | ---------------------- |
| `aarch64-apple-ios`     | 64-bit iOS devices (iPhone/iPad)          | iPhone 11+, iPad Pro   |
| `aarch64-apple-ios-sim` | ARM64 iOS simulator on Apple Silicon Macs | Simulator on M1/M2 Mac |
| `x86_64-apple-ios`      | x86_64 iOS simulator on Intel Macs        | Simulator on Intel Mac |

## Android

| Current support target    | Description                                             | Examples                |
| ------------------------- | ------------------------------------------------------- | ----------------------- |
| `x86_64-linux-android`    | 64-bit Android emulators (x86_64 architecture)          | Emulator on x86_64 host |
| `i686-linux-android`      | 32-bit Android emulators (x86 architecture, legacy)     | Legacy Android Emulator |
| `armv7-linux-androideabi` | 32-bit ARM devices (older Android smartphones/tablets)  | Nexus 7, Galaxy S5      |
| `aarch64-linux-androids`  | 64-bit ARM devices (modern Android smartphones/tablets) | Pixel 6, Galaxy S22     |

## Web (WASM)

| Current support target   | Description             | Examples     |
| ------------------------ | ----------------------- | ------------ |
| `wasm32-unknown-unknown` | Bare WebAssembly target | Browser apps |

## Proving Systems Currently Supported

|  Current support target   | Circom<br/>(rust-witness<br/>arkworks) | Circom<br/>(witnesscalc<br/>rapidsnark) | Halo2<br/>(Plonkish) | Noir<br/>(barretenberg) |
| :-----------------------: | :------------------------------------: | :-------------------------------------: | :------------------: | :---------------------: |
|    `aarch64-apple-ios`    |                   ✅                   |                   ✅                    |          ✅          |           ✅            |
|  `aarch64-apple-ios-sim`  |                   ✅                   |                   ✅                    |          ✅          |           ✅            |
|    `x86_64-apple-ios`     |                   ✅                   |                   ✅                    |          ✅          |           ✅            |
|  `x86_64-linux-android`   |                   ✅                   |                   ✅                    |          ✅          |           ✅            |
|   `i686-linux-android`    |                   ✅                   |                   ❌                    |          ✅          |           ❌            |
| `armv7-linux-androideabi` |                   ✅                   |                   ❌                    |          ✅          |           ❌            |
| `aarch64-linux-androids`  |                   ✅                   |                   ✅                    |          ✅          |           ✅            |
| `wasm32-unknown-unknown	`  |     rust-witness ❌ / ark-works ✅     |                   ❌                    |          ✅          |         ❌[^1]          |

[^1]: The current Mopro stack doesn't support compiling Barretenberg directly to WebAssembly. However, you can still use [@aztec/bb.js](https://www.npmjs.com/package/@aztec/bb.js) to generate proofs in the browser. Similar to the Circom prover, you can use [snarkjs](https://github.com/iden3/snarkjs) to generate a witness directly in the browser.
