# mopro

Making client-side proving on mobile simple.

## Overview

This is a WIP.

- `mopro-core` - core mobile Rust library.
- `mopro-ffi` - wraps `mopro-core` and exposes UniFFI bindings.
- `mopro-ios` - iOS CocoaPod library exposing native Swift bindings.
- `mopro-example-app` - example iOS app using `mopro-ios`.

## Architecture

The following illustration shows how mopro and its components fit together into the wider ZKP ecosystem:

![mopro architecture (full)](images/mopro_architecture2_full.png)

Zooming in a bit:

![mopro architecture](images/mopro_architecture2.png)

## Bindings

To update bindings, run `./script/update_bindings.sh`.