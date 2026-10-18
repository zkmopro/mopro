---
slug: private-balance-proofs-eas-noir-mopro
title: "Private Balance Proofs with EAS, Noir, and Mopro"
authors:
  name: Sooyoung Hyun
  title: Co-founder, ZKProofport
  url: https://github.com/hsy822
  image_url: https://github.com/hsy822.png
tags: [noir, mopro, eas, giwa, solidity]
---

An application checking a balance requirement may only need one answer: does this user meet the threshold? The exact balance and the wallet associated with it can remain private.

[GIWA Dojang](https://docs.giwa.io/giwa-ecosystem/dojang) provides a useful starting point. Built on the **Ethereum Attestation Service (EAS)**, it defines a Balance Root format that represents a snapshot of balances as a Merkle root.

We used that existing format to build a complete example: express the private conditions in Noir, generate a proof through Mopro’s native Noir adapter, and verify it in a Solidity contract that reads EAS state.

The example uses synthetic balances and a local chain. Proof generation, EAS interactions, and Solidity verification are all real.

[Explore the code and reproduction scripts →](https://github.com/zkproofport/dojang-private-balance)

## From an attestation to a private balance proof

A Balance Root summarizes a collection of balance entries. Each entry contains a wallet address, an amount, and a salt. A Merkle path connects the entry to the root recorded in an EAS attestation.

Suppose an entry contains a balance of 100, and an application requires at least 80. Our Noir circuit proves that:

- The balance entry belongs to the specified root.
- Its amount meets the public threshold.
- The wallet associated with the entry authorized this request.

The exact amount, original wallet address, Merkle path, and signature are private inputs. The threshold, root, receiving address, and request context are public.

The receiving address can differ from the wallet in the balance entry. The signed request binds that address, the chain, application contract, nonce, deadline, and attestation context. Changing these fields requires a new authorization.

The Solidity application checks the EAS record, verifies the proof, and marks the request as consumed.

## How the pieces fit together

Each component handles a different part of the flow:

| Component | Responsibility |
|---|---|
| **GIWA Dojang** | Provides the existing balance data format we target |
| **EAS** | Records the root’s issuer, schema, expiration, and revocation state |
| **Noir** | Expresses private membership, balance comparison, and wallet authorization |
| **Mopro** | Connects the Noir circuit to native proving and generated bindings |
| **Barretenberg and Solidity** | Generate the proof and verify it on the EVM |

Keeping the existing data format was an important constraint. Dojang uses Ethereum ABI encoding followed by double Keccak hashing for leaves, with sorted hash pairs along the Merkle path.

We implemented the same byte layout in Noir, including address padding. Amounts are represented as 32-byte integers, preserving the full `uint256` range without reducing values into a single field element.

Compatibility tests cover OpenZeppelin-generated trees and an original Dojang test vector. The Noir vector test calls the same Merkle functions used by the complete circuit.

## Why the EAS check still matters

A cryptographically valid proof can refer to a root that has expired, been revoked, or was issued by an organization the application does not trust.

The gate therefore reads EAS state when processing each request. It checks the issuer, schema, expiration, revocation, asset, and snapshot policy before verifying the proof. A root revoked after proof generation will still be rejected.

This separates two responsibilities: Noir checks the conditions over the private data, while the application checks whether the referenced attestation is acceptable.

We tested both successful requests and failure cases:

| Case | Expected result |
|---|---|
| Balance 100, threshold 80 or 100 | Accept |
| Balance 100, threshold 101 | Reject |
| Modified amount, Merkle path, or signature | Reject |
| Existing proof used with a different receiving address | Reject |
| Unauthorized issuer or expired/revoked root | Reject |
| Identical request submitted twice | Reject |

The test runner also distinguishes circuit rejection from execution errors. Missing files and malformed inputs cannot count as successful negative tests.

## Connecting Mopro to Solidity verification

We used Mopro’s Noir adapter with `noir-rs` beta.19 and Barretenberg `4.2.0-aztecnr-rc.2`. Proofs use the Keccak-based EVM configuration with zero knowledge enabled.

Three integration details were particularly useful:

- **Proof serialization.** This `noir-rs` version includes a four-byte public-input count before the inputs and proof. We used its parser and compared the resulting public inputs with those constructed by the Solidity gate.
- **Verifier deployment.** The generated Solidity verifier requires library linking. The scripts generate the verifier from the current verification key and deploy its linked library.
- **Native bindings.** We generated Swift bindings and verified the complete proof through Swift on macOS.

The iOS simulator build encountered a transitive `backtrace`/`libc` dependency issue. This balance circuit has not been executed on a simulator or physical phone. The repository records that failure alongside the successful native and Swift host paths.

## Measuring the complete circuit

We measured the initial implementation at three Merkle depths. Every run included membership verification, full-width balance comparison, wallet signature verification, and request binding.

| Merkle depth | Barretenberg finalized size (pre-padding) | Median adapter call | Median peak process memory |
| ------------ | ------------------------------- | ------------------- | -------------------------- |
| 8            | 311,356                         | 4.62 s              | 2.17 GiB                   |
| 12           | 383,248                         | 6.19 s              | 2.26 GiB                   |
| 16           | 455,140                         | 7.90 s              | 2.35 GiB                   |

Circuit sizes were extracted using the benchmark’s pinned toolchain and hash-matched ACIR artifacts. All three circuits have a padded size of 524,288 (2¹⁹).

Measurements used an Apple M4 with 16 GiB RAM, three measured runs per depth, and a preparatory run excluded.

The adapter interval includes witness execution, proving, and internal SRS reading/setup. Initial downloads and initial verification-key preparation are excluded. Memory covers the entire native process. These are host measurements, not mobile benchmarks or isolated proving times.

Successful local redemption transactions used approximately 3.06 million gas, including EAS policy checks, proof verification, and request consumption.

The [results document](https://github.com/zkproofport/dojang-private-balance/blob/main/docs/RESULTS.md) identifies the measured revision and preserves the raw data. Validation of the subsequent refactor is recorded separately; its depth benchmarks have not been rerun.

## Try the example

With the prerequisites listed in the README installed:

```sh
git clone https://github.com/zkproofport/dojang-private-balance.git
cd dojang-private-balance
bash scripts/bootstrap.sh
bash scripts/run.sh
```

The scripts prepare the pinned dependencies, start a local Anvil chain, deploy EAS and the verifier, and run the proof and policy checks.

Connecting this example to real GIWA issuer data still requires a private way for holders to obtain their amount, salt, and Merkle path. The implementation checks a condition about an issuer-asserted snapshot. It does not establish a current balance or conceal information already published elsewhere.

The repository brings together an existing GIWA attestation format, EAS state checks, Noir conditions, Mopro native proving, and Solidity verification in one reproducible example. Developers can use it to explore the same boundaries with their own attestation data and application policies.

At ZKProofport, we have also explored a related flow in a [Dojang KYC demo on GIWA Sepolia](https://x.com/ZKProofport/status/2106953631660651002). This separate prototype uses test attestations to demonstrate KYC eligibility and authorization for another wallet, applying a similar idea to a different credential.
