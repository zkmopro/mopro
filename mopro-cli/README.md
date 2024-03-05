# mopro-cli

`mopro` is a CLI tool for client-side proving of Zero Knowledge Proofs. It simplifies the process of initializing, building, updating, and testing projects across different platforms and configurations.

Think of it as Foundry for client-side proving.

## Installation

To use `mopro`, you need to have Rust and Cargo installed on your system. You can install them from [the official Rust website](https://www.rust-lang.org/learn/get-started).

Run `cargo install --path .` to install the `mopro` CLI util.

## Usage

Here are the basic commands of `mopro`:

- `mopro init`: Initialize a new project with support for multiple platforms.
- `mopro build`: Build the project for specified platforms.
- `mopro update`: Update bindings with new API for specified platforms.
- `mopro test`: Run tests for specific platform and test cases.

### Example

Here's a basic example with Rust bindings:

```
# Set MOPRO_ROOT
export MOPRO_ROOT=/Users/user/repos/github.com/oskarth/mopro

# Default to circom adapter and core Rust bindings
mopro init

# Go to the newly created directory
cd mopro-example-app

# Build the project, including preparing circuit artifacts
mopro build

# Run end-to-end-test
mopro test
```

For iOS:

```
mopro init --platforms ios
mopro build --platforms ios

# Currently testing only available for Rust bindings,
# Can run iOS tests from newly created Xcode project
mopro test
```

## Contributing

Contributions to `mopro` are welcome. Please feel free to submit issues and pull requests.