# BN254 Garaga calldata golden fixtures

SnarkJS-compatible artifacts used to verify `generate_circom_groth16_garaga_calldata`
against [Garaga v1.1.0](https://github.com/keep-starknet-strange/garaga/tree/v1.1.0)
`get_groth16_calldata_felt`.

| File | Source |
|------|--------|
| `proof.json` | Garaga `snarkjs_proof_bn254.json` example |
| `public.json` | Garaga `snarkjs_public_bn254.json` example |
| `verification_key.json` | Garaga `snarkjs_vk_bn254.json` example |
| `expected_garaga_calldata.json` | Generated via `cargo run -p garaga-calldata-tests --bin gen-garaga-calldata-fixture` |

Regenerate `expected_garaga_calldata.json` after changing parsers or bumping Garaga:

```bash
cargo run -p garaga-calldata-tests --bin gen-garaga-calldata-fixture
cargo test -p garaga-calldata-tests
```
