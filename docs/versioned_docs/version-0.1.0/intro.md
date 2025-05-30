---
sidebar_position: 1
---

# Introduction

Mopro makes client-side proving simple. You can think of it as a toolkit for ZK app development. It primarily focuses on running natively mobile.

How? Mopro connects different adapters with different platforms. You can think of an adapter as a way to use a library with some proof system and performance profile. Because Mopro takes care of hooking up your circuit to some library, and generating bindings for use on multiple platforms, you can focus on what you do best: ZK app development.

![mopro adapters and platforms](/img/roadmap.png)

Note that the above is a work in progress, and the dashed lines indicate things that are still experimental and/or in an early stage.

If you just want to get started using mopro, see [getting started](getting-started).

## Overview

Mopro consists of a set of libraries and utilities. The following subprojects are included in the [mopro monorepo](https://github.com/zkmopro/mopro).

Primary libraries and utilities of interest:

-   `mopro-ffi` - main package, exposes macros for configuring and building projects.
-   `mopro-wasm` - enables the compilation of Halo2 circuits into wasm modules.
-   `test-e2e` - example implementation of mopro in Android and iOS apps, used for testing.

Secondary subprojects:

-   `docs` - This documentation website.

## Architecture

The following illustration shows how mopro and its components fit together into the wider ZKP ecosystem:

![mopro architecture](/img/architecture.png)
