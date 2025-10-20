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

The system eliminates payment friction with 5-second NFC transactions that work on any EVM chain. Privacy is maintained through zero-knowledge proofs that verify user eligibility without exposing personal data. A single card handles all event interactions: tap-to-buy tickets, tap-to-register, tap-to-pay for concessions.

## Application Track: General Privacy

### 🥇 First Prize: zkETHer
> [View Project](https://ethglobal.com/showcase/zkether-geppk) | [GitHub](https://github.com/yashsharma22003/zkETHer-Protocol)

zkETHer implements a privacy-preserving protocol for ERC20 tokens, functioning as a non-custodial mixer. Users deposit fixed amounts by submitting cryptographic commitments to an on-chain Merkle tree, then withdraw to new addresses using zero-knowledge proofs generated on their mobile devices.

The protocol uses X25519 (ECDH) for key exchange, HKDF-SHA256 for deriving secrets, and Poseidon2 hash for commitments. Mopro enables computationally intensive ZK proofs to be generated directly on phones, making privacy accessible without specialized hardware.

The circuit implementation is robust, though real-world feasibility needs improvement for production deployment. The architecture demonstrates how mobile-first proving can bring mixer-style privacy to standard ERC20 tokens.

### 🥈 Second Prize: Wisk
> [View Project](https://ethglobal.com/showcase/wisk-gdvfw) | [GitHub](https://github.com/YadlaMani/wisk)

Wisk rethinks background verification for the digital age with zkPDF. Instead of sharing documents with third parties, users prove specific claims about government-issued certificates without revealing the full content.

The system integrates with [India's DigiLocker](https://www.digilocker.gov.in/) to verify official PDFs. Using zkPDF, Wisk validates the government's digital signature embedded in PDFs and generates ZKPs for requested fields (name, PAN number, credentials). The entire process happens in the browser—raw documents never leave the user's device.

### 🥉 Third Prize: ZeroSurf
> [View Project](https://ethglobal.com/showcase/zerosurf-9988k) | [GitHub](https://github.com/Krane-Apps/zerosurf-anon-aadhaar)

ZeroSurf is a mobile browser with built-in ZK age verification using [Anon Aadhaar](https://github.com/anon-aadhaar). The smooth deeplink integration allows users to prove age requirements without revealing birth dates, enabling privacy-preserving access to age-restricted content.

The implementation showcases how deeplinks can bridge mobile browsers and ZK proving apps, creating frictionless user experiences for privacy-preserving authentication.

## Key Takeaways

The hackathon revealed several promising directions for client-side privacy:

**UX innovations like NFC and deeplinks** proved that privacy-preserving technology can match the convenience of traditional systems. These features should be modularized within Mopro to improve developer experience. We'll invite teams that built these integrations to contribute reusable components.

**Photo-identity integrity** emerged as a recurring theme across multiple projects, adding security layers to identity verification. Integrating solutions like [Rarimo's Bionetta](https://docs.rarimo.com/zkml-bionetta/) and zkCamera with mobile-native proving through Mopro could strengthen this approach.

## Future Explorations

### zkTLS and Mobile Proving

zkTLS enables portable, verifiable data from any HTTPS connection without server cooperation. Using multi-party computation (MPC), zkTLS allows users to prove statements about web data—like account balances, transaction histories, or credentials—without revealing the underlying information or requiring platform APIs.

[**TLSNotary**](https://github.com/tlsnotary) leads the MPC-based approach, using garbled circuits to split TLS session keys between users and notaries, ensuring neither party can forge proofs alone. This creates portable proofs of web data while preserving privacy.

Mobile integration remains an open challenge. While TLSNotary works well on desktop, coordinating MPC between mobile apps and browsers presents unique technical hurdles. Solving this would unlock powerful use cases: proving income from banking apps, verifying social media reputation, or demonstrating transaction history—all without sharing credentials or raw data.

This represents an audacious but valuable exploration for the mobile proving ecosystem. Success would enable trustless verification of any web2 data directly from mobile devices.

### Unified ZK Registry System

The ZK identity landscape is fragmented. Projects like Anon Aadhaar, passport verification systems, and zkPDF each maintain separate on-chain registries for their proofs. Users juggling multiple credentials face redundant verifications, and developers must integrate with each system independently.

[ERC-7812](https://eips.ethereum.org/EIPS/eip-7812) proposes a solution: a singleton on-chain registry for storing commitments to private data. The registry uses Sparse Merkle Trees to maintain provable statements that can be verified via zero-knowledge proofs without revealing underlying data.

The architecture enables **multiple proof integration** through specialized Registrars. One Registrar might handle Anon Aadhaar commitments, another manages passport proofs, and a third processes zkPDF credentials—all writing to the same EvidenceDB. Each Registrar operates in an isolated namespace while benefiting from shared infrastructure.

**Revocation support** comes naturally from the SMT structure, which maintains idempotent add/remove operations. When a credential is revoked, its commitment can be removed from the tree while maintaining cryptographic proofs of the registry's state before and after.

Key benefits of a unified identity system:
- **Cross-chain portability**: A single Merkle root represents the entire registry state, enabling cheap cross-chain proof verification
- **Proof reusability**: Credentials proven once can be referenced across applications without re-verification
- **Centralized trust model**: Users trust one immutable, permissionless contract instead of multiple registries
- **Interoperability**: Different proof systems coexist, allowing developers to choose optimal solutions for their use cases

This would reduce deployment friction, improve interoperability, and lower costs for projects building ZK applications. Future hackathons should explore standards and implementations that move us toward this unified vision.
