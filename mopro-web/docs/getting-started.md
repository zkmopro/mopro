---
sidebar_position: 2
---

# Getting Started

We recommend you use [mopro-cli](https://github.com/oskarth/mopro/tree/main/mopro-cli#mopro-cli) to create and maintain your application. Here's how you can get started with your example app in a few minutes.

You can also watch this short [tutorial](https://www.loom.com/share/6ff382b0497c47aea9d0ef8b6e790dd8).

## Prerequisites

Depending on what platforms and adapters you use, there are several prerequisites to install before getting started.

- General
    - [Rust](https://www.rust-lang.org/learn/get-started)
- Circom
    - [circom](https://docs.circom.io/)
    - [snarkjs](https://github.com/iden3/snarkjs)
- iOS
    - [Xcode](https://developer.apple.com/xcode/)
    - [CocoaPods](https://cocoapods.org/)
- Android
    - [Android Studio](https://developer.android.com/studio)
    - Also see configuration below

### Android configuration

Some additional configuration is required for Android.

First, install the latest SDK. In Android Studio, go to `SDK Manager > SDK Tools`  and install `NDK (Side by Side)` (see [Android Developer site](https://developer.android.com/studio/projects/install-ndk#default-version)).

After that, set the following  environment variables:

```sh
# Export `$ANDROID_HOME` and change `{USER_NAME}` to your username
export ANDROID_HOME="/Users/{USER_NAME}/Library/Android/sdk"

# Locate which NDK version you have
ls $ANDROID_HOME/ndk # => 26.1.10909125

# Set it to your `NDK_PATH` environment variable
NDK_PATH=$ANDROID_HOME/ndk/26.1.10909125
```

(Reference: [Running Rust on Android with UniFFI](https://sal.dev/android/intro-rust-android-uniffi/)).

## Install dependencies

First, make sure you've installed the prerequisites above.

Then, run the following commands:

```sh
# Clone the mopro repo
git clone git@github.com:oskarth/mopro.git

# Go to your newly cloned checkout
cd mopro

# Install mopro-cli locally
(cd mopro-cli && cargo install --path .)

# Set `MOPRO_ROOT` (replace with path to your git checkout of mopro)
# For example: `export MOPRO_ROOT=/Users/user/repos/github.com/oskarth/mopro`
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

```sh
# Build the project
mopro build

# Run end-to-end test (in Rust only)
mopro test

# Build the project for iOS
mopro build --platforms ios

# Open in Xcode to run on simulator/device
open ios/ExampleApp/ExampleApp.xcworkspace

# Build the project for Android
mopro build --platforms android

# Open in Android Studio to run on simulator/device
open android -a Android\ Studio
```

See [mopro-cli](https://github.com/oskarth/mopro/tree/main/mopro-cli#mopro-cli) for more details on usage.