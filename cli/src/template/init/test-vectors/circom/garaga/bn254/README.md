# BN254 Garaga calldata golden fixtures

Artifacts used to verify `generate_circom_groth16_garaga_calldata`
against [Garaga v1.1.0](https://github.com/keep-starknet-strange/garaga/tree/v1.1.0)
`get_groth16_calldata_felt`.

| File | Role |
|------|------|
| `proof.json` | Test fixture only — builds a `CircomProofResult` for golden tests (not a runtime API input) |
| `public.json` | Test fixture only — public inputs for the fixture `CircomProofResult` |
| `verification_key.json` | Required at runtime (one-time snarkjs zkey export) |
| `expected_garaga_calldata.json` | Generated via `cargo run -p garaga-calldata-tests --bin gen-garaga-calldata-fixture` |

Regenerate `expected_garaga_calldata.json` after changing parsers or bumping Garaga:

```bash
cargo run -p garaga-calldata-tests --bin gen-garaga-calldata-fixture
cargo test -p garaga-calldata-tests
```
