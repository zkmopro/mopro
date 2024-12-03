# Mopro example app

This is the example app of mopro. You can use the following commands to build native bindings for your iOS and/or Android app.

**📚 To learn more about mopro, visit: https://zkmopro.org**

## Getting Started

To set up and build bindings, follow these steps.

### 1. Install the Mopro CLI Tool

```sh
git clone https://github.com/zkmopro/mopro
cd mopro/cli
cargo install --path .
```

### 2. Generate Native Bindings

Navigate to the Mopro example app directory, then build the bindings with:

```sh
mopro build
```

### 3. Create Platform-Specific Templates

To generate templates tailored to your target platform, use:

```sh
mopro create
```

## Advanced: Customize Builds Using Rust

For advanced usage, you can manually run Rust commands to build in either debug or release mode.

### iOS

- Debug Mode:
    ```sh
    cargo run --bin ios  # Debug mode
    ```
- Release Mode:
    ```sh
    CONFIGURATION=release cargo run --bin ios # Release mode
    ```

### Android

- Debug Mode:
    ```sh
    cargo run --bin android  # Debug mode
    ```
- Release Mode:
    ```sh
    CONFIGURATION=release cargo run --bin android # Release mode
    ```

## Community

-   X account: <a href="https://twitter.com/zkmopro"><img src="https://img.shields.io/twitter/follow/zkmopro?style=flat-square&logo=x&label=zkmopro"></a>
-   Telegram group: <a href="https://t.me/zkmopro"><img src="https://img.shields.io/badge/telegram-@zkmopro-blue.svg?style=flat-square&logo=telegram"></a>
