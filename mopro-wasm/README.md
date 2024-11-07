# mopro-wasm

Mopro can compile wasm code instead of iOS/Android native code at [mopro-ffi](../mopro-ffi/).

## Getting started

- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Install `chrome` and `chromedriver` with same version.
 
## Run tests

Run test in browser on headless way.

```bash
wasm-pack test --chrome --headless
```