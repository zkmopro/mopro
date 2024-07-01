# Halo2 Keccak

This repo is a refactored version of the Axiom's [halo2-lib/hashes](https://github.com/axiom-crypto/halo2-lib/tree/community-edition/hashes/zkevm) repo, adjusted so that it compiles on the stable Rust toolchain.
The adjustments were made to avoid the dependency on the `halo2-base` crate as well as Axioms `halo2_proofs`, instead using [PSE's `halo2_proofs`](https://github.com/privacy-scaling-explorations/halo2) with tag `"v2023_04_20"`, which works with stable Rust toolchain.

## Usage

### Run tests
**Note**: The tests take a long time to run (`k_18_rows_per_round_9` takes over an hour on M1 Pro).
```bash
  cargo test -- --nocapture
```


## Implementation Details

Keccak circuit in vanilla halo2. This implementation starts from [PSE version](https://github.com/privacy-scaling-explorations/zkevm-circuits/tree/main/zkevm-circuits/src/keccak_circuit), then adopts some changes from [this PR](https://github.com/scroll-tech/zkevm-circuits/pull/216) and later updates in PSE version.

The major difference is that this version directly represent raw inputs and Keccak results as witnesses, while the original version only has RLCs(random linear combination) of raw inputs and Keccak results. Because this version doesn't need RLCs, it doesn't have the 2nd phase or use challenge APIs.

### Logical Input/Output

Logically the circuit takes an array of bytes as inputs and Keccak results of these bytes as outputs.

`keccak::vanilla::witness::multi_keccak` generates the witnesses of the circuit for a given input.

### Background Knowledge

All these items remain consistent across all versions.

- Keccak process a logical input `keccak_f` by `keccak_f`.
- Each `keccak_f` has `NUM_ROUNDS`(24) rounds.
- The number of rows of a round(`rows_per_round`) is configurable. Usually less rows means less wasted cells.
- Each `keccak_f` takes `(NUM_ROUNDS + 1) * rows_per_round` rows. The last `rows_per_round` rows could be considered as a virtual round for "squeeze".
- Every input is padded to be a multiple of RATE (136 bytes). If the length of the logical input already matches a multiple of RATE, an additional RATE bytes are added as padding.
- Each `keccak_f` absorbs `RATE` bytes, which are splitted into `NUM_WORDS_TO_ABSORB`(17) words. Each word has `NUM_BYTES_PER_WORD`(8) bytes.
- Each of the first `NUM_WORDS_TO_ABSORB`(17) rounds of each `keccak_f` absorbs a word.
- `is_final`(anothe name is `is_enabled`) is meaningful only at the first row of the "squeeze" round. It must be true if this is the last `keccak_f` of a logical input.
- The first round of the circuit is a dummy round, which doesn't correspond to any input.

### Raw inputs

- In this version, we added column `word_value`/`bytes_left` to represent raw inputs.
- `word_value` is meaningful only at the first row of the first `NUM_WORDS_TO_ABSORB`(17) rounds.
- `bytes_left` is meaningful only at the first row of each round.
- `word_value` equals to the bytes from the raw input in this round's word in little-endian.
- `bytes_left` equals to the number of bytes, which haven't been absorbed from the raw input before this round.
- More details could be found in comments.

### Keccak Results

- In this version, we added column `hash_lo`/`hash_hi` to represent Keccak results.
- `hash_lo`/`hash_hi` of a logical input could be found at the first row of the virtual round of the last `keccak_f`.
- `hash_lo` is the low 128 bits of Keccak results. `hash_hi` is the high 128 bits of Keccak results.

### Example

In this version, we care more about the first row of each round(`offset = x * rows_per_round`). So we only show the first row of each round in the following example.
Let's say `rows_per_round = 10` and `inputs = [[], [0x89, 0x88, .., 0x01]]`. The corresponding table is:

| row           | input idx | round | word_value           | bytes_left | is_final | hash_lo | hash_hi |
| ------------- | --------- | ----- | -------------------- | ---------- | -------- | ------- | ------- |
| 0 (dummy)     | -         | -     | -                    | -          | false    | -       | -       |
| 10            | 0         | 1     | `0`                  | 0          | -        | -       | -       |
| ...           | 0         | ...   | ...                  | 0          | -        | -       | -       |
| 170           | 0         | 17    | `0`                  | 0          | -        | -       | -       |
| 180           | 0         | 18    | -                    | 0          | -        | -       | -       |
| ...           | 0         | ...   | ...                  | 0          | -        | -       | -       |
| 250 (squeeze) | 0         | 25    | -                    | 0          | true     | RESULT  | RESULT  |
| 260           | 1         | 1     | `0x8283848586878889` | 137        | -        | -       | -       |
| 270           | 1         | 2     | `0x7A7B7C7D7E7F8081` | 129        | -        | -       | -       |
| ...           | 1         | ...   | ...                  | ...        | -        | -       | -       |
| 420           | 1         | 17    | `0x0203040506070809` | 9          | -        | -       | -       |
| 430           | 1         | 18    | -                    | 1          | -        | -       | -       |
| ...           | 1         | ...   | ...                  | 0          | -        | -       | -       |
| 500 (squeeze) | 1         | 25    | -                    | 0          | false    | -       | -       |
| 510           | 1         | 1     | `0x01`               | 1          | -        | -       | -       |
| 520           | 1         | 2     | -                    | 0          | -        | -       | -       |
| ...           | 1         | ...   | ...                  | 0          | -        | -       | -       |
| 750 (squeeze) | 1         | 25    | -                    | 0          | true     | RESULT  | RESULT  |

