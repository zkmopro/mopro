# ark-zkey

Experiments in reading `zkey` faster for arkworks.

```
cargo test --release -- --nocapture
```

See https://github.com/oskarth/mopro/issues/25 for context.

## Keccak

In-memory, custom serialization and unchecked

`cargo test keccak256 --release -- --nocapture`

```
running 1 test
Processing zkey data...
Time to process zkey data: 2.995291ms
Serializing proving key and constraint matrices
Time to serialize proving key and constraint matrices: 42ns
Writing arkzkey to: ../mopro-core/examples/circom/keccak256/target/keccak256_256_test_final.arkzkey
Time to write zkey: 432.917µs
Reading arkzkey from: ../mopro-core/examples/circom/keccak256/target/keccak256_256_test_final.arkzkey
Time to open arkzkey file: 10µs
Time to mmap arkzkey: 3.791µs
Time to deserialize proving key: 253.875µs
Time to deserialize matrices: 5.709µs
Time to read arkzkey: 315.875µs
test tests::test_keccak256_serialization_deserialization ... ok
```
