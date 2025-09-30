# Test for Mopro FFI

Run tests for different FFI bindings before building the CLI.

## UniFFI

### iOS
```sh
cargo run --bin ios
```

### Android

```sh
cargo run --bin android
```

## Flutter FFI

> [!IMPORTANT]  
> The `flutter` feature cannot be enbaled with `uniffi` feature together

### Dart

```sh
cargo run --bin flutter --no-default-features --features flutter
```