# Getting started

This tutorial guides you through building a static library with the Circom/Halo2 adapter for Android, iOS and Web and creating example templates for mobile development.

## 0. Prerequisites

Make sure you've installed the [prerequisites](/docs/prerequisites).

## 1. Install CLI

Clone the `mopro` repository and install the `mopro` CLI tool.

```sh
git clone https://github.com/zkmopro/mopro
cd mopro/cli
cargo install --path .
cd ../..
```

## 2. Initialize adapters

Navigate to the folder where you want to build the app. Select the adapters using the `mopro` CLI.

```sh
mopro init
```

## 3. Build bindings

Navigate to your project directory. (e.g. `cd mopro-example-app`) <br/>
Build bindings for specific targets (iOS, Android, Web).

```sh
mopro build
```

:::warning
The process of building bindings may take a few minutes. 
:::

:::info
Running your project in `release` mode significantly enhances performance compared to `debug` mode. This is because the Rust compiler applies optimizations that improve runtime speed and reduce binary size, making your application more efficient.

:::

## 4. Create templates

Create templates for developing your mobile app.

```sh
mopro create
```

Follow the instructions to open the development tools

For iOS:
```sh
open ios/MoproApp.xcodeproj
```

For Android:
```sh
open android -a Android\ Studio
```

For Web:
