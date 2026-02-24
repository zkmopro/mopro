#!/bin/sh

set -e

OUTDIR=out

[ -d "$OUTDIR" ] || mkdir "$OUTDIR"

circom --r1cs --wasm ./circuits/multiplexer.circom -p bls12381 -o $OUTDIR

cd $OUTDIR

# Build a ptau for bls
snarkjs ptn bls12381 12 power_bls.ptau
snarkjs powersoftau prepare phase2 power_bls.ptau power_bls_final.ptau

snarkjs ptn bn128 12 power_bn.ptau
snarkjs powersoftau prepare phase2 power_bn.ptau power_bn_final.ptau

snarkjs groth16 setup multiplexer.r1cs power_bls_final.ptau multiplexer_final.zkey
snarkjs zkey export verificationKey multiplexer_final.zkey multiplexer.vkey.json