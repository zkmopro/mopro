---
sidebar_position: 1
---

# Introduction

Mopro makes client-side proving simple. You can think of it as a toolkit for ZK app development. It primarily focuses on running natively mobile, but it works for any platform.

How? Mopro connects different adapters with different platforms. You can think of an adapter as a way to use a library with some proof system and performance profile. Because Mopro takes care of hooking up your circuit to some library, and generating bindings for use on multiple platforms, you can focus on what you do best: ZK app development.

![mopro adapters and platforms](/img/mopro_adapters_platforms.png).

Note that above is a work in progress, and the dashed lines indicate things that are still experimental and/or in an an early stage.

If you just want to get started using mopro, see [getting started](/docs/getting-started).

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