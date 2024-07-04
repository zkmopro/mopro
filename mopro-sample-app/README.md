# mopro-sample-app

### WORKS FOR `IOS` ONLY NOW

This is a sample app. To compile it and the bindings, run the following:

```
cargo build --release && cargo run --bin ios generate --library target/release/libmopro_bindings.dylib --language swift --out-dir target/out
```

You can then open the iOS project using:

```
open ios/mopro-test.xcodeproj
```

You can then have a look at `ContentView` where there is a `hello` function that shows how Halo2 proof can be used,
while also showing that all of the previous exports from `mopro-ffi` also work.