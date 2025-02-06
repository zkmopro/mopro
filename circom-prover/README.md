# Circom Prover

Circom prover is a Rust library for generating and verifying proofs for [Circom](https://github.com/iden3/circom) circuits.
It is designed to be used in cross-platform applications, and is compatible with the [Mopro](https://github.com/zkmopro/mopro) library.

## Adapters

## Witness Generation

-   [x] [Rust Witness](https://github.com/chancehudson/rust-witness)
-   [ ] [Witnesscalc](https://github.com/zkmopro/witnesscalc-adapter)
-   [ ] [circom witnesscalc](https://github.com/iden3/circom-witnesscalc)

## Proof Generation

-   [x] [Arkworks](https://github.com/arkworks-rs)
-   [ ] [Rapidsnark](https://github.com/zkmopro/rust-rapidsnark)

## Performance

It speeds up circom proof by ~100x comparing to [arkworks-rs/circom-compat](https://github.com/arkworks-rs/circom-compat) in keccak256 circuits.
We will provide more benchmarks with different adapters in the future.
And you can also check the [Mopro documentation](https://zkmopro.org/docs/performance) for more benchmarks.

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
-   Mopro Documentation: https://zkmopro.org

## Acknowledgements

This work is sponsored by [PSE](https://pse.dev/).
