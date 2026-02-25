#!/bin/sh

set -e

OUTDIR=out

[ -d "$OUTDIR" ] || mkdir "$OUTDIR"

circom --r1cs --wasm ./circuits/multiplexer.circom -p bls12381 -o $OUTDIR

cd $OUTDIR

# Build a ptau for bls
snarkjs ptn bls12381 12 power_bls.ptau
snarkjs powersoftau contribute power_bls.ptau power_bls_2.ptau --name="dev" -e="some random text"
snarkjs powersoftau contribute power_bls_2.ptau power_bls_3.ptau --name="dev" -e="some random text2"
snarkjs powersoftau prepare phase2 power_bls_3.ptau power_bls_final.ptau

snarkjs ptn bn128 12 power_bn.ptau
snarkjs powersoftau prepare phase2 power_bn.ptau power_bn_final.ptau

snarkjs groth16 setup multiplexer.r1cs power_bls_final.ptau multiplexer_final.zkey
snarkjs zkey export verificationKey multiplexer_final.zkey multiplexer.vkey.json