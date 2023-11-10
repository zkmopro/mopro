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
test tests::test_keccak256_serialization_deserialization has been running for over 60 seconds
Time to process zkey data: 90.343011917s
Serializing proving key and constraint matrices
Time to serialize proving key and constraint matrices: 0ns
Writing arkzkey to: ../mopro-core/examples/circom/keccak256/target/keccak256_256_test_final.arkzkey
Time to write zkey: 10.742360375s
Reading arkzkey from: ../mopro-core/examples/circom/keccak256/target/keccak256_256_test_final.arkzkey
Time to open arkzkey file: 55µs
Time to mmap arkzkey: 15.125µs
Time to deserialize proving key: 10.497670542s
Time to deserialize matrices: 41.603833ms
Time to read arkzkey: 10.539544125s
test tests::test_keccak256_serialization_deserialization ... ok
```
