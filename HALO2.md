### Halo2 configuration

We currently support Halo2 circuits in an experimental stage. To use Halo2, you need to set the `adapter` to `halo2` in the `mopro-config.toml` file.

```toml
[circuit]
adapter = "halo2" # Options: circom, halo2
dir = "mopro-core/examples/halo2/fibonacci" # Directory of the circuit
name = "fibonacci"                # Name of the circuit
```

The `dir` should point to the directory where the Halo2 circuit is located. The `name` should be the name of the circuit.

Note that currently the Halo2 circuit **must** be a cargo crate, with the package name hardcoded to `halo2-circuit`. This is due to us swapping out the default implementation of the `halo2-ciurcuit` crate with the user's circuit during the build process.
This is done using the `build.rs` script in the `mopro-core` crate, which changes the path to the default `examples/halo2/fibonacci` to `$dir`. This also requires the name of the package to be `halo2-circuit` for the substitution to work.

Additionally, the `halo2-circuit` crate **must** implement and expose three items, which you can see in the `mopro-core/examples/halo2/fibonacci/src/lib.rs`:
- `Circuit` - the struct that represents the circuit
- `prove` - the function that generates the proof
- `verify` - the function that verifies the proof

This is then used by the `mopro-core` crate to generate the proof and verify it.

