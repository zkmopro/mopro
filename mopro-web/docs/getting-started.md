---
sidebar_position: 3
---

# Getting Started

We recommend you use [mopro-cli](/docs/guides/mopro-cli) to create and maintain your application. Here's how you can get started with your example app in a few minutes.

You can also watch this short (~5m) [tutorial](https://www.loom.com/share/6ff382b0497c47aea9d0ef8b6e790dd8).

## Install dependencies

First, make sure you've installed the [prerequisites](/docs/prerequisites).

Then, run the following commands:

```sh
# Clone the mopro repo
git clone https://github.com/zkmopro/mopro.git

# Go to your newly cloned checkout
cd mopro

# Install mopro-cli locally
(cd mopro-cli && cargo install --path .)

# Set `MOPRO_ROOT` (replace with path to your git checkout of mopro)
# For example: `export MOPRO_ROOT=/Users/user/repos/github.com/zkmopro/mopro`
export MOPRO_ROOT=$(PWD)

# Install `mopro` dependencies
mopro deps
```

## Create a project

Create and initialize a project:

```sh
# Initialize a project
# This will create a new project in your current directory
mopro init --platforms ios android

# Go to your project folder
cd mopro-example-app
```

## Configure mopro settings

You may adapt `mopro-config.toml` to your needs. For example, if you already have a Circom project you can use that.

Prepare your circuit artifacts:

```sh
mopro prepare
```

This only has to be done once when changing the circuit.

## Build, test and run your project

Depending on what platforms you are targetting, you can run the following commands:

-   Build the project

    ```sh
    mopro build
    ```

-   Run end-to-end test (in Rust only)

    ```sh
    mopro test
    ```

-   Build the project for iOS

    ```sh
    mopro build --platforms ios
    ```

-   Open in Xcode to run on simulator

    ```sh
    open ios/ExampleApp/ExampleApp.xcworkspace
    ```

    Use `command`+`U` to run tests.

-   Build the project for Android

    ```sh
    mopro build --platforms android
    ```

-   Open in Android Studio to run on simulator

    ```sh
    open android -a Android\ Studio
    ```

    Use `^R` (`control`+`R`) to execute a simulator.

> See [mopro-cli](/docs/guides/mopro-cli) for more details on usage.
> Edit [mopro configuration](/docs/guides/mopro-configuration) to build for device or build for other circuits.
