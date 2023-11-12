# ark-zkey

Library to read `zkey` faster by serializing to `arkworks` friendly format.

See https://github.com/oskarth/mopro/issues/25 for context.

## To generate arkzkey

Hacky, but the way we generate `arkzkey` now is by running the corresponding test.

Note that we also neeed to change the const `ZKEY_BYTES` above.

E.g.:

```
cargo test multiplier2 --release -- --nocapture
cargo test keccak256 --release -- --nocapture
cargo test rsa --release -- --nocapture
```

Will take corresponding `zkey` and put `arkzkey`` in same folder.

## Multiplier

NOTE: Need to change const ZKEY here

`cargo test multiplier2 --release -- --nocapture`

```
running 1 test
[build] Processing zkey data...
[build] Time to process zkey data: 3.513041ms
[build] Serializing proving key and constraint matrices
[build] Time to serialize proving key and constraint matrices: 42ns
[build] Writing arkzkey to: ../mopro-core/examples/circom/multiplier2/target/multiplier2_final.arkzkey
[build] Time to write arkzkey: 1.884875ms
Reading arkzkey from: ../mopro-core/examples/circom/multiplier2/target/multiplier2_final.arkzkey

Time to open arkzkey file: 18.084µs
Time to mmap arkzkey: 8.542µs
Time to deserialize proving key: 305.75µs
Time to deserialize matrices: 5µs
Time to read arkzkey: 348.083µs
test tests::test_multiplier2_serialization_deserialization ... ok
```

Naive test: `cargo test naive --release -- --nocapture` (with right zkey constant).

**Result: `350µs` vs naive `3.3ms`**

## Keccak

NOTE: Need to change const ZKEY here

`cargo test keccak256 --release -- --nocapture`

```
[build] Processing zkey data...
test tests::test_keccak256_serialization_deserialization has been running for over 60 seconds
[build]Time to process zkey data: 158.753181958s
[build] Serializing proving key and constraint matrices
[build] Time to serialize proving key and constraint matrices: 42ns
[build] Writing arkzkey to: ../mopro-core/examples/circom/keccak256/target/keccak256_256_test_final.arkzkey
[build] Time to write arkzkey: 16.204274125s
Reading arkzkey from: ../mopro-core/examples/circom/keccak256/target/keccak256_256_test_final.arkzkey
Time to open arkzkey file: 51.75µs
Time to mmap arkzkey: 17.25µs
Time to deserialize proving key: 18.323550083s
Time to deserialize matrices: 46.935792ms
Time to read arkzkey: 18.3730695s
test tests::test_keccak256_serialization_deserialization ... ok
```

Vs naive:

`[build] Time to process zkey data: 158.753181958s`


**Result: 18s vs 158s**
