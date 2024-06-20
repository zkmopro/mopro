---
sidebar_position: 4
---

# Mopro Configuration

This config file is best used together with `mopro-cli`.

By creating a `toml` configuration file you can specify what build settings you want to use. Example is provided in `config-example.toml`:

```toml
# mopro-config.toml

[build]
# For iOS device_type can be x86_64, simulator, device
ios_device_type = "device" # Options: x86_64, simulator, device
# For Android device_type can be x86_64, x86, arm, arm64
android_device_type = "arm64" # Options: x86_64, x86, arm, arm64

# debug is for Rust library to be in debug mode and release for release mode
# We recommend release mode by default for performance
build_mode = "release" # Options: debug, release

[circuit]
# multiplier2
adapter = "circom"
dir = "core/circuits/multiplier2"
name = "multiplier2"
ptau = "02"                       # ptau to use for trusted setup of circuit, "01" to "27"

[dylib]
# NOTE: Dylib support is experimental and requires some fiddling in iOS
# See https://github.com/zkmopro/mopro/pull/37 and https://github.com/zkmopro/mopro/pull/38
use_dylib = false        # Options: true, false
name = "multiplier2.dylib" # Name of the dylib file, only used if use_dylib is true

[witness]
# Note: circom-witness-rs is experimental
# See https://github.com/zkmopro/mopro/issues/32 for updates
# Only works for keccak256_256_test circuit now
use_native_witness_generation = false # Options: true, false
```

## `build` options

### `ios_device_type`

-   `x86_64`
-   `simulator`
-   `device`

### `android_device_type`

-   `x86_64`
-   `x86`
-   `arm`
-   `arm64`

### `build_mode`

-   `debug`
-   `release`

## `circuit` options

### `adapter`

Now we support the following adapters:

-   `circom`
-   `halo2`

### `dir`

-   path to the circuit directory<br/>
    e.g. `core/circuits/multiplier2`

### `name`

-   the name of the circuit<br/>
    e.g. `multiplier2`

### `ptau`

-   ptau is to used for trusted setup<br/>
    e.g. `02`

## `dylib` options (experimental)

:::warning
Dylib support is experimental and requires some fiddling in iOS.<br/>
See https://github.com/zkmopro/mopro/pull/37 and https://github.com/zkmopro/mopro/pull/38
:::

### `use_dylib`

-   `true`
-   `false`

## `witness` options (experimental)

:::warning
circom-witness-rs is experimental.<br/>
https://github.com/zkmopro/mopro/issues/32 for updates.
:::

### `use_native_witness_generation`

-   `true`
-   `false`
