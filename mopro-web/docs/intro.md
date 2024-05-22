---
sidebar_position: 1
---

# Introduction

Mopro is a toolkit for ZK app development on mobile. Mopro makes client-side proving on mobile simple.

## Overview

mopro consists of a set of libraries and utilities. Here's a list of the various subprojects:

- `mopro-cli` - core Rust CLI util.
- `mopro-core` - core mobile Rust library.
- `mopro-ffi` - wraps `mopro-core` and exposes UniFFI bindings.
- `templates/mopro-example-app` - example multi-platform app template.
- `ark-zkey` - helper utility to make zkey more usable and faster in arkworks.
- `mopro-ios` - iOS CocoaPod library exposing native Swift bindings. (will be deprecated)
- `mopro-android` - Android library exposing native Kotlin bindings. (will be deprecated)
- `webprover` - Prove example circuits through a browser, used for benchmarking.
- `scripts` - various helper scripts for `mopro-cli` and testing.

## Architecture

The following illustration shows how mopro and its components fit together into the wider ZKP ecosystem:

![mopro architecture](/img/mopro_architecture2.png)
