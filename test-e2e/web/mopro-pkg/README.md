# mopro-wasm

Mopro can compile wasm code instead of [mopro-ffi](../mopro-ffi/) for supported proving systems.

## Introduction to WASM Compilation with Halo2

To enable multithreading in WASM, `wasm-bindgen-rayon` must be used for Halo2.

> Usage with WebAssembly
By default, when building to WebAssembly, Rayon will treat it as any other platform without multithreading support and will fall back to sequential iteration. This allows existing code to compile and run successfully with no changes necessary, but it will run slower as it will only use a single CPU core.

from: [Rayon - github](https://github.com/rayon-rs/rayon#usage-with-webassembly)

## Getting started

- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Install `chrome` and `chromedriver` with same version.

## Run tests

Run the fibonacci circuit tests for all backends—"plonk," "hyperplonk," and "gemini"—in the browser in a headless.

```bash
wasm-pack test --chrome --headless -- --all-features
```