---
sidebar_position: 3
---

# Getting Started

We recommend you use [mopro-cli](/docs/mopro-cli) to create and maintain your application. Here's how you can get started with your example app in a few minutes.

You can also watch this short (~5m) [tutorial](https://www.loom.com/share/6ff382b0497c47aea9d0ef8b6e790dd8).

## Install dependencies

First, make sure you've installed the [prerequisites](/docs/prerequisites).

Then, run the following commands:

1. Clone the mopro repo

```sh
git clone https://github.com/zkmopro/mopro.git
```

2. Go to your newly cloned checkout

```sh
cd mopro
```

3. Install mopro-cli locally

```sh
(cd mopro-cli && cargo install --path .)
```

4. Set `MOPRO_ROOT` (replace with path to your git checkout of mopro)

```sh
export MOPRO_ROOT=$(PWD)
```

5. Install `mopro` dependencies

```sh
mopro deps
```

## Create a project

Create and initialize a project:

1. Initialize a project
   This will create a new project in your current directory

```sh
mopro init --platforms ios android
```

2. Go to your project folder

```sh
cd mopro-example-app
```

## Configure mopro settings

You may adapt [`mopro-config.toml`](circom/configuration.md) to your needs. For example, if you already have a Circom project you can use that.

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

## iOS

-   Build the project for iOS

    ```sh
    mopro build --platforms ios
    ```

-   Open in Xcode to run on simulator

    ```sh
    open ios/ExampleApp/ExampleApp.xcworkspace
    ```

    Use `command`+`U` to run tests.

## Android

-   Build the project for Android

    ```sh
    mopro build --platforms android
    ```

-   Open in Android Studio to run on simulator

    ```sh
    open android -a Android\ Studio
    ```

    Use `^R` (`control`+`R`) to execute a simulator.

> See [mopro-cli](/docs/mopro-cli) for more details on usage.
> Edit [mopro configuration](/docs/circom/configuration) to build for device or build for other circuits.
