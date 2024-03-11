# mopro-cli

`mopro` is a CLI tool for client-side proving of Zero Knowledge Proofs. It simplifies the process of initializing, building, updating, and testing projects across different platforms and configurations.

Think of it as Foundry for client-side proving.

## Installation

To use `mopro`, you need to have Rust and Cargo installed on your system. You can install them from [the official Rust website](https://www.rust-lang.org/learn/get-started).

Run `cargo install --path .` to install the `mopro` CLI util.

## Usage

Here are the basic commands of `mopro`:

- `mopro init`: Initialize a new project with support for multiple platforms.
- `mopro deps`: Install required dependencies.
- `mopro prepare`: Prepare and build circuit and its artifacts.
- `mopro build`: Build the project for specified platforms.
- `mopro test`: Run tests for specific platform and test cases.
- `mopro export-bindings`: Export platform bindings to some other directory.

(May be added soon: `mopro update`: Update bindings with new API for specified platforms.)

## Examples

### Basic example

Initialize, build and test a circuit with Rust bindings:

```
# Set MOPRO_ROOT
export MOPRO_ROOT=/Users/user/repos/github.com/oskarth/mopro

# Install dependencies
mopro deps

# Default to circom adapter and core Rust bindings
mopro init

# Go to the newly created directory
cd mopro-example-app

# Prepare circuit artifacts
mopro prepare

# Build the project
mopro build

# Run end-to-end-test
mopro test
```

### iOS

Initialize and build an app with iOS support.

```
mopro init --platforms ios
cd mopro-example-app
mopro prepare
mopro build --platforms ios

# Currently testing only available for Rust bindings,
# Can run iOS tests from newly created Xcode project
mopro test
```

### Android

Initialize and build an app with Android support.
> Currently support [Android Studio](https://developer.android.com/studio)

```
mopro init --platforms android
cd mopro-example-app
mopro prepare
mopro build --platforms android
```

### Exporting bindings

To export bindings to a different directory:

`mopro export-bindings --destination <DESTINATION_DIR> --platforms <IOS_OR_ANDROID>`

This will the following files, assuming they've been built, to the destination directory:

```
├── SwiftBindings
│   ├── mopro.swift
│   ├── moproFFI.h
│   └── moproFFI.modulemap
└── libmopro_ffi.a
```

```
├── KotlinBindings
│   └── mopro.kt
├── JniLibs
│   └── <ARCHITECTURE>
│       └── libuniffi_mopro.so
```

## Contributing

Contributions to `mopro` are welcome. Please feel free to submit issues and pull requests.