# ark-zkey

Experiments in reading `zkey` faster for arkworks.

```
cargo test --release -- --nocapture
```

See https://github.com/oskarth/mopro/issues/25 for context.


## multiplier2 test

(Release)

```
running 1 test
Reading zkey from: ../mopro-core/examples/circom/multiplier2/target/multiplier2_final.zkey
Time to read zkey: 7.561959ms
Serializing proving key and constraint matrices
Time to serialize proving key and constraint matrices: 42ns
Processing zkey data...
Time to process zkey data: 6.311958ms
Serializing proving key and constraint matrices
Time to serialize proving key and constraint matrices: 42ns
Writing arkzkey to: ../mopro-core/examples/circom/multiplier2/target/multiplier2_final.arkzkey
Time to write zkey: 776.209µs
Reading arkzkey from: ../mopro-core/examples/circom/multiplier2/target/multiplier2_final.arkzkey
Time to open arkzkey file: 15.333µs
Time to mmap arkzkey: 9.583µs
Time to deserialize proving key: 4.260667ms
Time to deserialize matrices: 1µs
Time to read arkzkey: 4.301084ms
test tests::test_serialization_deserialization ... ok
```

Deserializing proving key should be a lot faster.