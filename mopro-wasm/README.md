# mopro-wasm

Mopro can compile wasm code instead of iOS/Android native code at [mopro-ffi](../mopro-ffi/).

## Getting started

- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
 
## Run tests

<!-- TODO: Is that right?? yes-->
<!-- follow this refs: https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/usage.html#configure-cargoconfig-to-use-the-test-runner -->
Run test in headless browser.

```bash
wasm-pack test --chrome --headless
```