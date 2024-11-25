#!/bin/sh

set -e

SRCDIR=$(realpath "$(dirname "$0")")
WORKDIR=$(mktemp -d)

cd $WORKDIR

git clone --depth=1 https://github.com/zkmopro/benchmark-app.git
cd $WORKDIR/benchmark-app/witness/circuits/keccak256

npm install

cd $WORKDIR
git clone --depth=1 https://github.com/iden3/circom-witnesscalc.git

cd $WORKDIR/circom-witnesscalc

KECCAK_BIN=keccak256_256_test.bin
# build the binfile
cargo run --release --package circom_witnesscalc --bin build-circuit $WORKDIR/benchmark-app/witness/circuits/keccak256/keccak256_256_test.circom $KECCAK_BIN

mv $KECCAK_BIN $SRCDIR/test-vectors/circom/$KECCAK_BIN

