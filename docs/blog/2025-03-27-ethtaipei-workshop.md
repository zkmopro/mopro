---
slug: 2025-ethtaipei-workshop
title: 2025 ETHTaipei Workshop
authors:
    name: Vivian Jeng
    title: Developer on the Mopro Team
    url: https://github.com/vivianjeng
    image_url: https://github.com/vivianjeng.png
tags: [workshop]
---

## Overview

This tutorial guides developers through getting started with Mopro and building a native mobile app from scratch. It covers

-   Setting up an example [multiplier2](https://github.com/zkmopro/circuit-registry/blob/main/multiplier2/multiplier2.circom) Circom circuit
-   Modifying it to use a different circuit, such as [keccak256](https://github.com/zkmopro/circuit-registry/blob/main/keccak256/keccak256_256_test.circom)
-   Additionally, we'll integrate the [semaphore-rs](https://github.com/worldcoin/semaphore-rs) Rust crate to generate native bindings and run the implementation on both iOS and Android.

:::info
This is a workshop tutorial from [ETHTaipei](https://ethtaipei.org/) 2025 in April. If you'd like to follow along and build a native mobile app, please check out this commit: [eab28f](https://github.com/zkmopro/mopro/tree/eab28f8e318ff0afc053c2c004c58afe2f34fdb7).
:::

## Prerequisites

-   XCode or Android Studio
    -   If you're using Android Studio, ensure that you follow the [Android configuration](https://zkmopro.org/docs/prerequisites/#android-configuration) steps and set the `ANDROID_HOME` environment variable.
-   Rust and CMake

:::info
Documentation: https://zkmopro.org/docs/prerequisites
:::
