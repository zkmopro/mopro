---
slug: ethglobal-new-delhi
title: "ETHGlobal New Delhi: Advancing Client-Side Privacy"
authors:
  name: Moven Tsai
  title: Developer on the Mopro Team
  url: https://github.com/moven0831
  image_url: https://github.com/moven0831.png
tags: [ethglobal, hackathon]
---

import Tweet from '@site/src/components/Tweet';

At [ETHGlobal New Delhi](https://ethglobal.com/events/newdelhi/prizes#ethereum-foundation) this September, the Mopro and zkPDF teams sponsored two tracks focused on bringing general privacy to Ethereum. The hackathon delivered impressive projects that pushed boundaries in both infrastructure and application development.

Several submissions exceeded expectations with standout UX features. **Deeplink integration** enables seamless transitions between mobile apps and browsers, allowing native mobile proving across existing browser applications that require ZK (like age verification for websites). **NFC integration** demonstrated tap-to-prove and tap-to-verify capabilities, creating an experience as intuitive as Apple Pay. These implementations show the maturity of client-side ZK proving and its readiness for real-world adoption.

<div style={{display: 'flex', justifyContent: 'center', margin: '2rem 0'}}>
  <table style={{border: '2px solid #444', borderRadius: '8px', padding: '1rem', maxWidth: '500px'}}>
    <tbody>
      <tr>
        <td style={{padding: '1rem', textAlign: 'center'}}>
          <p style={{fontStyle: 'italic', marginBottom: '1rem', color: '#888', fontSize: '0.95rem'}}>
            [ZeroSurf](#-third-prize-zerosurf) demo: Privacy-preserving age verification with deeplink integration
          </p>
          <Tweet tweetId="1979055018683556051" width="400px" />
        </td>
      </tr>
    </tbody>
  </table>
</div>

## Infrastructure Track: Client-Side Privacy

### 🏆 Grand Prize: AccessFI
> [View Project](https://ethglobal.com/showcase/accessfi-8v4ns) | [GitHub](https://github.com/coderwithsense/EthGlobalDelhi)

AccessFI reimagines event payments with NFC-powered privacy. Users receive P-256 compatible SECP256k1 NFC cards linked to their wallets, enabling instant tap-to-pay for tickets, registration, food, and merchandise while preserving privacy through deterministic encryption.

The system eliminates payment friction with 5-second NFC transactions that work on any EVM chain. Privacy is maintained through ZK proofs that verify user eligibility without exposing personal data. A single card handles all event interactions: tap-to-buy tickets, tap-to-register, tap-to-pay for concessions.

![AccessFI Flow](/img/ethglobal-new-delhi-accessfi.jpg)

## Application Track: General Privacy

### 🥇 First Prize: zkETHer
> [View Project](https://ethglobal.com/showcase/zkether-geppk) | [GitHub](https://github.com/yashsharma22003/zkETHer-app)

zkETHer implements a privacy-preserving protocol for ERC20 tokens, functioning as a non-custodial mixer. Users deposit fixed amounts by submitting cryptographic commitments to an on-chain Merkle tree, then withdraw to new addresses using ZK proofs generated on their mobile devices.

The protocol uses X25519 (ECDH) for key exchange, HKDF-SHA256 for deriving secrets, and Poseidon2 hash for commitments. Mopro enables computationally intensive ZK proofs to be generated directly on phones, making privacy accessible without specialized hardware.

The circuit implementation is robust, though real-world feasibility needs improvement for production deployment. The architecture demonstrates how mobile-first proving can bring mixer-style privacy to standard ERC20 tokens.

<div style={{display: 'flex', justifyContent: 'center', margin: '1.5rem 0'}}>
  <table style={{border: '2px solid #444', borderRadius: '8px', padding: '1rem', maxWidth: '500px'}}>
    <tbody>
      <tr>
        <td style={{padding: '1rem', textAlign: 'center'}}>
          <p style={{fontStyle: 'italic', marginBottom: '1rem', color: '#888', fontSize: '0.95rem'}}>
            zkETHer demo: Privacy-preserving ERC20 mixer with mobile ZK proving
          </p>
          <video
            width="400"
            controls
            style={{borderRadius: '4px', maxWidth: '100%'}}
          >
            <source src="https://ethglobal.storage/projects/geppk/video/high.mp4?t=1760977955579" type="video/mp4" />
            Your browser does not support the video tag.
          </video>
        </td>
      </tr>
    </tbody>
  </table>
</div>

### 🥈 Second Prize: Wisk
> [View Project](https://ethglobal.com/showcase/wisk-gdvfw) | [GitHub](https://github.com/YadlaMani/wisk)

Wisk rethinks background verification for the digital age with zkPDF. Instead of sharing documents with third parties, users prove specific claims about government-issued certificates without revealing the full content.

The system integrates with [India's DigiLocker](https://www.digilocker.gov.in/) to verify official PDFs. Using zkPDF, Wisk validates the government's digital signature embedded in PDFs and generates ZKPs for requested fields (name, PAN number, credentials). The entire process happens in the browser—raw documents never leave the user's device.

### 🥉 Third Prize: ZeroSurf
> [View Project](https://ethglobal.com/showcase/zerosurf-9988k) | [GitHub](https://github.com/Krane-Apps/zerosurf-anon-aadhaar)

ZeroSurf is a mobile browser with built-in ZK age verification using [Anon Aadhaar](https://github.com/anon-aadhaar). The smooth deeplink integration allows users to prove age requirements without revealing birth dates, enabling privacy-preserving access to age-restricted content.

The implementation showcases how deeplinks can bridge mobile browsers and ZK proving apps, creating frictionless user experiences for privacy-preserving authentication.

## Key Takeaways & Future Explorations

The hackathon revealed several promising directions for client-side privacy:

**UX innovations like NFC and deeplinks** proved that privacy-preserving technology can match the convenience of traditional systems. These features should be modularized within Mopro to improve developer experience. We'll invite teams that built these integrations to contribute reusable components.

**Photo-identity integrity** emerged as a recurring theme across multiple projects, adding security layers to identity verification. Integrating solutions like [Rarimo's Bionetta](https://docs.rarimo.com/zkml-bionetta/) and zkCamera with mobile-native proving through Mopro could strengthen this approach.

We're excited to see more UX innovations emerge in future hackathons. Whether it's tap-to-prove bringing native mobile experiences, smooth deeplink transitions between apps and browsers, or entirely new interaction patterns—the goal is providing developers with easy-to-use building blocks. By modularizing these patterns in Mopro, we can transform what took teams days to build during the hackathon into features that take minutes to integrate.

Beyond these UX enhancements, there are also two fundamental challenges worth exploring:

### zkTLS and Mobile Proving

zkTLS enables portable, verifiable data from any HTTPS connection without server cooperation. Using multi-party computation (MPC), zkTLS allows users to prove statements about web data—like account balances, transaction histories, or credentials—without revealing the underlying information or requiring platform APIs.

[**TLSNotary**](https://github.com/tlsnotary) leads the MPC-based approach, using garbled circuits to split TLS session keys between users and notaries, ensuring neither party can forge proofs alone. This creates portable proofs of web data while preserving privacy.

Mobile integration remains an open challenge. While TLSNotary works well on desktop, coordinating MPC between mobile apps and browsers presents unique technical hurdles. Solving this would unlock powerful use cases: proving income from banking apps, verifying social media reputation, or demonstrating transaction history—all without sharing credentials or raw data.

### Unified ZK Registry System

The ZK identity landscape is fragmented. Projects like [Anon Aadhaar](https://github.com/anon-aadhaar), passport-based zkID solutions, and zkPDF each maintain separate on-chain registries. Users face redundant verifications, and developers must integrate with each system independently.

[ERC-7812](https://eips.ethereum.org/EIPS/eip-7812) proposes a solution: a singleton on-chain registry using Sparse Merkle Trees to store commitments to private data. Statements can be verified via ZK proofs without revealing underlying information.

With unified client libraries built around ERC-7812 and integrated with Mopro, developers would call one API after generating proofs on-device, regardless of proof type. The real power emerges in cross-application identity: a user proves their age once with Anon Aadhaar in one Mopro app, committing to ERC-7812. Later, a Mopro app using different schemes verifies that commitment without re-proving. The unified registry enables seamless credential reuse across the mobile applications while preserving privacy.
