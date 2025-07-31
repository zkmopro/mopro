---
slug: ethglobal-cannes-2025
title: "Announcing the ETHGlobal Cannes 2025 Mopro Track Winners: Advancing Mobile Proving"
authors:
  name: Moven Tsai
  title: Developer on the Mopro Team
  url: https://github.com/moven0831
  image_url: https://github.com/moven0831.png
tags: [ethglobal, hackathon]
---

[ETHGlobal Cannes](https://ethglobal.com/events/cannes) brought together some of the brightest minds in blockchain development, and we're thrilled to announce the winners of the Mopro track. Over 36 intense hours, developers pushed the boundaries of client-side proving with real-world applications on mobile device.

With innovative submissions ranging from protocol-level integrations to consumer-facing applications, Mopro track showed how mobile ZK proving is becoming increasingly practical. Teams leveraged Mopro's mobile proving capabilities to build everything from tamper-proof media verification to post-quantum secure smart accounts with [EIP-7702](https://eip7702.io/), proving that privacy-preserving technology can be both powerful and user-friendly.

## The Power of Client-Side Proving

Before diving into the winners, it's worth highlighting what made this hackathon special. While many ZK applications rely on server-side proving or trusted execution environments (TEEs), Mopro enables developers to generate zero-knowledge proofs directly on mobile devices. This approach puts privacy control back in users' hands and opens up new possibilities for decentralized applications.

The submissions demonstrated two key trends: protocol developers integrating zkVM such as [RISC-0](https://github.com/risc0/risc0) and libraries like [Gnark](https://github.com/Consensys/gnark) with Mopro, and application developers building user-facing products that leverage existing ZK circuits. Both approaches are critical for the ecosystem's growth.

## The Winners

### 🥇 First Place: Mobiscale - Photo-ID Verification with Apple's Secure Enclave

**Project:** [Mobiscale](https://ethglobal.com/showcase/mobiscale-n9vj6)  
**GitHub:** [ElusAegis/MobiScale](https://github.com/ElusAegis/MobiScale)

Mobiscale achieved what many thought impossible in a hackathon setting: a complete end-to-end proof of photo-ID verification running entirely on a mobile device in under 90 seconds. The team cleverly combined Apple's Secure Enclave for facial recognition, RISC-0 for TEE attestation verification, and [Noir](https://noir-lang.org/) with the [Barretenberg proving backend](https://github.com/AztecProtocol/aztec-packages/tree/master/barretenberg) for ECDSA signature validation.

What makes this project remarkable is its practical approach to identity verification. By computing cosine similarity between a passport photo and a live selfie within the TEE, then proving this computation happened correctly using ZK proofs, Mobiscale demonstrates a privacy-preserving liveness check that could be deployed in the near future. The integration with Mopro shows how mobile proving can complement hardware security features to create robust identity solutions.

### 🥈 Second Place: Zkipper - PQ-Secure EIP-7702 Smart Accounts

**Project:** [Zkipper](https://ethglobal.com/showcase/zkipper-czc3z)  
**GitHub:** [ZKNoxHQ/ZKipper](https://github.com/ZKNoxHQ/ZKipper)

The ZKNox team showcased their cryptographic expertise at the hackathon with "Zkipper," a project that turns the ARX chips in ETHGlobal wristbands into transaction signers, enabling post‑quantum–secure smart accounts via EIP‑7702.

The technical achievement here is twofold: First, they successfully integrated Gnark with Mopro within the 48-hour timeframe, which is a significant contribution to the ecosystem. Second, they implemented [Falcon512](https://falcon-sign.info/) post-quantum signatures to secure smart accounts, preventing "Bybit-style" attacks by separating admin commands onto distinct devices. This approach shows how Mopro can enable hardware-based security solutions that are both quantum-resistant and user-friendly.

### 🥉 Third Place: 👀Proov! - Tamper-Proof Media with ZK

**Project:** [👀Proov!](https://ethglobal.com/showcase/eyes-proov-at10u)  
**GitHub:** [undefinedlab/PROOV_ZK](https://github.com/undefinedlab/PROOV_ZK)

👀Proov! stood out not just for its technical implementation but for its exceptional UI/UX design. The team created a complete solution for tamper-proof media capsules, combining Mopro-generated proofs with AI-powered image summaries, decentralized storage on Walrus, and verification on Flow blockchain.

The project offers one-tap proof generation with instant cryptographic capsule creation, selective privacy controls, and future-disclosure capabilities. By embedding tamper-proof QR codes in image capsules, they made every photo machine-verifiable and portable across platforms like Instagram, X, and Telegram. This shows how ZKP can be packaged into consumer-friendly applications without sacrificing security or privacy.

## Other Notable Submissions

Beyond the top three winners, the Mopro track at ETHGlobal Cannes attracted submissions that collectively show the diversity of ZK use cases on mobile. Teams explored verification and attestation use cases through projects like [ProofOfParticipation](https://ethglobal.com/showcase/proofofparticipation-xca0a) (GPS-based event attendance), [ZKAge Proof Mobile](https://ethglobal.com/showcase/zkage-proof-mobile-2yroo) (age verification for restricted services). [Bidet](https://ethglobal.com/showcase/bidet-gdvtq)'s privacy-preserving NFC tag game showcased gaming and social use cases, and [ProofOfFunds](https://ethglobal.com/showcase/proofoffunds-zczod) showed financial privacy by letting users prove they meet a cryptocurrency balance threshold without disclosing exact amounts or wallet addresses.

These submissions integrated various technical stacks across different proving frameworks—Circom and Noir—and mobile platforms—Swift (iOS), Kotlin (Android), React Native, and Flutter—alongside Mopro’s proving capabilities. This integration validates the platform’s flexibility across use cases and tech stacks. The diversity of both applications and technical aspect highlights the ecosystem’s readiness for real-world deployment and shows how mobile proving can address privacy challenges across multiple industries.

## Key Insights and Developer Feedback

The hackathon provided valuable insights into the state of mobile ZK proving:

**What's Working Well:**
- Mopro's developer experience received consistently positive feedback
- Teams successfully integrated various proving systems (RISC-0, Noir, Gnark) with Mopro
- The ecosystem is mature enough for developers to build meaningful applications in 36 hours

**Challenges Identified:**
- Writing circuits remains the biggest pain point for developers
- On-chain verification varies in stability (Circom is more mature, Noir is catching up)
- Developers need single-architecture templates for iOS-only or Android-only builds

## Looking Forward: The Future of Client-Side Proving

The diversity of submissions—from geo-proof games adapted from [zkVerify's proof-of-geolocation circuits](https://github.com/zkVerify/explorations/tree/main/mopro-proof-of-geolocation/proof-of-geolocation) to age verification systems—shows that mobile ZK proving is ready for mainstream adoption. While Mopro may not pursue the same level of direct adoption as protocols aimed at end users, it serves a critical role as an incubation platform for client-side ZK applications especially on mobile phone.

Based on developer feedback, we're prioritizing several improvements:
- **Enhanced Templates** - Expanding variety for different use cases ([Issue #503](https://github.com/zkmopro/mopro/issues/503), [Issue #438](https://github.com/zkmopro/mopro/issues/438))
- **Single Architecture Support** - iOS-only and Android-only bindings for cross-platform frameworks like Flutter ([Issue #502](https://github.com/zkmopro/mopro/issues/502)) and React Native ([Issue #501](https://github.com/zkmopro/mopro/issues/501)).
- **Improved DevEx** - Better naming for custom bindings ([Issue #500](https://github.com/zkmopro/mopro/issues/500))
- **Documentation** - Simplified architecture overview ([Issue #498](https://github.com/zkmopro/mopro/issues/498))

## Acknowledgments

We'd like to thank all teams who participated in the Mopro track at ETHGlobal Cannes. Your innovation, dedication, and feedback are invaluable in advancing the state of mobile proving.

Special recognition goes to the ETHGlobal team for organizing an exceptional event and providing the infrastructure that makes these innovations possible.

## Get Involved

The work showcased at ETHGlobal Cannes is just the beginning. If you're interested in building with Mopro or contributing to the ecosystem:

- Explore the [Mopro documentation](https://zkmopro.org/docs)
- Check out the [project gallery](https://zkmopro.org/docs/projects) for inspiration
- Join the [community discussions on Telegram](https://t.me/zkmopro) and contribute to the [open issues](https://github.com/zkmopro/mopro/issues)
