Built from [here](https://github.com/chancehudson/rapidsnark/tree/rust-ffi). This is built into static libraries and then linked into the rust project. Importantly we need to compile static libraries for each platform we intend to support.

Linking static libraries instead of compiling from c++ source should make it easier to maintain the `build.rs` script.
