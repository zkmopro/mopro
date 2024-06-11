# Halo2 Examples

This rust crate contains examples of Halo2 circuit that does Fibonacci calculation and checks if the provided number is
a Fibonacci number.

## Instruction

1. Compile the repo

```
cargo build
```

2. Run Fibonacci example

```
cargo test -- --nocapture fibonacci_example
```

3. Plot the circuit layout

```
cargo test --all-features -- --nocapture plot_fibonacci
```

4. Write the proving and verifying keys, as well as the SRS

```
cargo run --bin fibonacci
```
