---
sidebar_position: 1
---

# Introduction

Mopro is a toolkit for ZK app development on mobile. Mopro makes client-side proving on mobile simple.

If you just want to get started using mopro, see [getting started](/getting-started).

## Overview

Mopro consists of a set of libraries and utilities. The following subprojects are including in the [mopro monorepo](https://github.com/zkmopro/mopro).

Primary libraries and utilities of interest:
-   `mopro-cli` - core Rust CLI util.
-   `mopro-core` - core mobile Rust library.
-   `mopro-ffi` - wraps `mopro-core` and exposes UniFFI bindings.
-   `templates/mopro-example-app` - example multi-platform app template.
-   `ark-zkey` - helper utility to make zkey more usable and faster in arkworks.

Secondary subprojects:
-   `mopro-ios` - iOS CocoaPod library exposing native Swift bindings. (will be deprecated)
-   `mopro-android` - Android library exposing native Kotlin bindings. (will be deprecated)
-   `web-prover` - Prove example circuits through a browser, used for benchmarking.
-   `scripts` - various helper scripts for `mopro-cli` and testing.
-   `mopro-web` - Mopro website.
-   `research` - Ongoing research, e.g. GPU explorations.

## Architecture

The following illustration shows how mopro and its components fit together into the wider ZKP ecosystem:

![mopro architecture](/img/mopro_architecture2.png)