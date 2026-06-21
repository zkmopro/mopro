mod adapter;

pub use adapter::NoirProverAdapter;

// Input  = Vec<String> — UniFFI: [String]/List<String>; wasm: JsValue via serde; FRB: List<String>
// Output = Vec<u8>     — UniFFI: Data/ByteArray;        wasm: Uint8Array;          FRB: Uint8List
// No custom Record/Enum types needed — primitives are FFI-native in all three layers.
#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
