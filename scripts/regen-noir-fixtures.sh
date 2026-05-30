#!/usr/bin/env bash
# Regenerate the Noir multiplier2 test fixtures (.json / .srs / .vk) used by
# the cli template tests and the iOS/Android sample apps. Run from repo root
# whenever the noir-rs pin in `cli/src/init/noir.rs` changes.
#
# Requires:
#   - nargo at the version pinned in cli/src/init/noir.rs (currently v1.0.0-beta.19)
#   - network access to crs.aztec.network for the SRS download
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CIRCUIT_DIR="$ROOT/test-vectors/noir/multiplier2"
TARGET_DIR="$CIRCUIT_DIR/target"
TEMPLATE_VEC="$ROOT/cli/src/template/init/test-vectors/noir"
TOPLEVEL_VEC="$ROOT/test-vectors/noir"

echo "[1/4] Compiling multiplier2 with $(nargo --version | head -1)"
( cd "$CIRCUIT_DIR" && nargo compile )

echo "[2/4] Downloading SRS and computing Keccak + Poseidon VKs"
cargo run --manifest-path "$ROOT/scripts/regen-noir-fixtures/Cargo.toml" --release -- \
    "$TARGET_DIR/noir_multiplier2.json" \
    "$TARGET_DIR/noir_multiplier2.srs" \
    "$TARGET_DIR/noir_multiplier2.vk" \
    "$TARGET_DIR/noir_multiplier2_poseidon.vk"

# nargo bakes the absolute source path into file_map; clear it so the committed
# fixture is host-independent and this script is reproducible across machines.
echo "[3/4] Normalizing circuit JSON (clearing machine-specific file_map)"
python3 - "$TARGET_DIR/noir_multiplier2.json" <<'PY'
import json, sys
p = sys.argv[1]
with open(p) as f:
    data = json.load(f)
data["file_map"] = {}
with open(p, "w") as f:
    f.write(json.dumps(data, separators=(",", ":")))
PY

echo "[4/4] Copying artifacts to $TEMPLATE_VEC and $TOPLEVEL_VEC"
cp "$TARGET_DIR/noir_multiplier2.json"          "$TEMPLATE_VEC/"
cp "$TARGET_DIR/noir_multiplier2.srs"           "$TEMPLATE_VEC/"
cp "$TARGET_DIR/noir_multiplier2.vk"            "$TEMPLATE_VEC/"
cp "$TARGET_DIR/noir_multiplier2_poseidon.vk"   "$TEMPLATE_VEC/"
cp "$TARGET_DIR/noir_multiplier2.vk"            "$TOPLEVEL_VEC/"

echo "Done. Verify with: cd <fresh mopro init project> && cargo test --all --all-features"
