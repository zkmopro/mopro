# CLAUDE.md — mopro (root)

This file defines architectural invariants for the mopro monorepo during and
after the `core` / `provers` / `ffi` / `cli` restructuring. It is the source
of truth for dependency boundaries that the compiler will NOT enforce for
you — read this before touching any crate under `provers/`, `ffi/`, or `cli/`.

## Target structure

```
mopro/
├── core/                  # Prover trait + shared types. No FFI deps, no internal deps.
├── provers/               # "adapters" — thin wrappers that implement core::Prover
│   ├── circom-adapter/    # delegates to external circom-prover crate
│   ├── halo2-adapter/     # wraps user-supplied circuit fn pointers
│   ├── noir-adapter/      # delegates to noir-rs (barretenberg feature)
│   └── plonk-prover/      # future community slot
├── ffi/
│   ├── common/        # Arch/Mode/PlatformBuilder traits + shared build utilities
│   ├── uniffi/        # build pipeline: runs uniffi-bindgen, emits Swift/Kotlin
│   ├── flutter/       # build pipeline: runs FRB codegen, emits Dart package
│   ├── react-native/  # build pipeline: runs uniffi-bindgen-react-native
│   └── wasm/          # build pipeline: runs wasm-pack, emits pkg/
├── cli/                   # mopro-cli — the ONLY composition root
├── templates/             # scaffold templates per (adapter × platform)
├── tests/
├── test-vectors/
└── docs/
```

## Dependency direction

```
core/          ──► (nothing internal)
provers/*      ──► core  +  external prover crate
                   +  optional FFI runtime/annotation crates
                      (uniffi, wasm-bindgen, flutter_rust_bridge, serde)
ffi/* ──► (nothing internal) — build pipeline only
cli/           ──► core,  provers/*,  ffi/*
```

### `core/` — defines the contract, nothing else

- Holds the `Prover` trait, `MoproError`, and shared types (`ProofBytes`, etc.).
- Zero FFI-tool dependencies: no `uniffi`, no `wasm-bindgen`, no `flutter_rust_bridge`.
- Does not know that `ffi/` or `cli/` exist.

### `provers/*` — implement the contract AND own FFI compatibility

Each adapter crate has **two responsibilities**:

1. **Implement `core::Prover`** — a thin Rust wrapper that delegates all actual
   proving/verifying logic to an external crate (circom-prover, noir-rs, etc.).
   No proving code lives inside this monorepo.

2. **Own FFI compatibility for their output types** — the `Output` type (and any
   supporting structs/enums) carries the annotations that make it directly usable
   by all three FFI layers via optional Cargo features:

   | Feature flag  | FFI layer              | What gets added                                                      |
   |---------------|------------------------|----------------------------------------------------------------------|
   | `uniffi`      | iOS / Android (UniFFI) | `uniffi::Record` / `uniffi::Enum` derives; `setup_scaffolding!()` in `lib.rs` |
   | `wasm`        | Browser (wasm-bindgen) | `#[wasm_bindgen]` on wrapper functions; `serde` for `JsValue` conversion |
   | `flutter`     | Flutter (FRB)          | FRB codegen scans public types; `String`/`Vec<u8>`/`Vec<String>` are native — no extra annotation needed |

   `serde` (`Serialize` / `Deserialize`) is always present on output types regardless
   of feature flags, because JSON serialisation is useful in every context.

   **Allowed optional dependencies in `provers/*`:**
   - `uniffi` — runtime proc-macros (`Record`, `Enum`, `setup_scaffolding!`)
   - `wasm-bindgen` — proc-macros + JS glue for browser exports
   - `flutter_rust_bridge` — proc-macros for FRB codegen
   - `serde` — always on
   - The external prover crate being adapted

   **Never allowed in `provers/*`:**
   - `uniffi-bindgen` — this is the Swift/Kotlin *code generator* (CLI tool), not
     the runtime. It belongs in the `ffi/uniffi` build pipeline.
   - Any `ffi/*` crate as a hard dependency.

3. **Adding a new adapter must require zero changes to `ffi/*`.**
   If it does require changes, the boundary has been violated — stop and re-check.

### `ffi/*` — build pipeline only, no ZK knowledge, no Rust types

`ffi/*` crates are **build-pipeline orchestrators**, not Rust glue-code crates:

- `ffi/uniffi` — invokes `uniffi-bindgen` to generate Swift/Kotlin from the adapter's scaffolding; runs `cargo build` with `--features uniffi`.
- `ffi/wasm` — runs `wasm-pack build` with `--features wasm`; sets up `pkg/` output.
- `ffi/flutter` — runs FRB codegen; sets up the Dart package.
- `ffi/react-native` — runs `uniffi-bindgen-react-native`; sets up the JS package.

Rules:
- No `ffi/*` crate's `Cargo.toml` lists `mopro-core` or any `provers/*` crate as a dependency.
- No Rust types related to proving live in `ffi/*` — those live in the adapter.
- If you find yourself writing "what a proof is" inside `ffi/*`, that logic belongs in the adapter or in `cli/`.

### `cli/` — the only composition root

- The only crate allowed to depend on both a specific adapter (from `provers/*`)
  and a specific FFI backend (from `ffi/*`) simultaneously.
- Given a user's choice of (adapter × platform), `cli/` enables the right
  feature flags on the adapter and invokes the right `ffi/*` pipeline.
- All "which adapters exist" and "which backend to use" logic lives here.

## Verifying the boundaries

Run before finishing any change under `ffi/`, `provers/`, or `core/`.

```bash
# ffi/backends must NOT pull in core or any adapter (the one invariant the
# compiler won't catch for you)
cargo tree -p <ffi-backend-crate-name> | grep -iE "mopro-core|circom-prover-adapter|halo2-prover-adapter|noir-prover-adapter"
# Should print nothing.
```

## Migration notes

- The old `mopro-ffi` crate combined (1) FFI tooling orchestration and (2) wiring
  specific provers into specific platforms. This refactor splits them:
  (1) → `ffi/*`, (2) → `cli/`.
- Do not recreate a crate that re-merges these two responsibilities.
- `circom-prover` already existed as a standalone crate on crates.io and is
  adapted (not rewritten) under `provers/circom-adapter`.
- `halo2-adapter` and `noir-adapter` extract logic that used to live inside
  `mopro-ffi`.
- `plonk-prover` does not exist yet — it is a placeholder proving that the
  architecture admits community-contributed adapters with zero changes to
  `ffi/` or `cli/`.

## Status

> Update this section as migration proceeds.
> States: `not started` / `in progress` / `done`

- [x] `core/` — `Prover` trait, `MoproError`, `ProofBytes` defined; no FFI deps
- [x] `provers/circom-adapter` — `Prover` impl done; `serde` + optional `uniffi` derives on output types; compiles (integration tests need zkey test-vectors)
- [x] `provers/halo2-adapter` — `Prover` impl done; `Halo2Output` with `serde` + optional `uniffi::Record`
- [x] `provers/noir-adapter` — `Prover` impl done; Input/Output are FFI-primitive (`Vec<String>` / `Vec<u8>`); `barretenberg` feature gates noir-rs
- [x] `ffi/uniffi` — `IosPlatform`/`AndroidPlatform` defined locally; `impl PlatformBuilder` for both; compiles; no deps on core or provers
- [x] `ffi/flutter` — `FlutterPlatform` defined locally; `impl PlatformBuilder`; compiles; no deps on core or provers
- [x] `ffi/react-native` — `ReactNativePlatform` defined locally; `impl PlatformBuilder`; compiles; no deps on core or provers
- [x] `ffi/wasm` — `WebPlatform` defined locally; `impl PlatformBuilder`; compiles; no deps on core or provers
- [ ] `cli/` — updated to composition root (not started)
- [ ] old `mopro-ffi` crate removed (not started)
- [ ] `templates/` updated to reflect new (adapter × platform) structure (not started)
