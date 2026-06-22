#!/usr/bin/env bash
# Verify that no ffi/backends/* crate pulls in mopro-core or any prover adapter.
# Run from the workspace root: ./scripts/check-boundaries.sh

set -euo pipefail

CARGO=${CARGO:-cargo}
FORBIDDEN="mopro-core|circom-prover-adapter|halo2-prover-adapter|noir-prover-adapter"

FFI_BACKEND_CRATES=(
    mopro-build-common
    mopro-uniffi-backend
    mopro-wasm-backend
    mopro-flutter-backend
    mopro-react-native-backend
)

VIOLATIONS=0

for crate in "${FFI_BACKEND_CRATES[@]}"; do
    result=$("$CARGO" tree -p "$crate" 2>/dev/null | grep -iE "$FORBIDDEN" || true)
    if [ -n "$result" ]; then
        echo "VIOLATION  $crate  →  $result"
        VIOLATIONS=$((VIOLATIONS + 1))
    else
        echo "OK         $crate"
    fi
done

echo ""
if [ "$VIOLATIONS" -gt 0 ]; then
    echo "FAILED: $VIOLATIONS boundary violation(s). ffi/backends/* must not depend on core or provers."
    exit 1
else
    echo "PASSED: all ffi/backends/* crates respect the dependency boundary."
fi
