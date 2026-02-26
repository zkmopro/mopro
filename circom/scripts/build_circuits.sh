#!/bin/sh

set -e

OUTDIR=out

[ -d "$OUTDIR" ] || mkdir "$OUTDIR"

circom ./circuits/mux.circom --r1cs --wasm --sym -l ./node_modules -p bls12381 -o $OUTDIR

cd $OUTDIR

# Build a ptau for bls
snarkjs ptn bls12381 14 "pot_bls12381_14_0.ptau"
snarkjs powersoftau contribute "pot_bls12381_14_0.ptau" "pot_bls12381_14_1.ptau" --name="dev" -e="some random text"
snarkjs powersoftau prepare phase2 "pot_bls12381_14_1.ptau" "pot_bls12381_14_final.ptau"

snarkjs groth16 setup mux.r1cs "pot_bls12381_14_final.ptau" mux.zkey
snarkjs zkey export verificationkey mux.zkey mux.vkey.json
snarkjs wtns calculate mux_js/mux.wasm ../input_mux.json "witness_mux.wtns"
snarkjs groth16 prove mux.zkey "witness_mux.wtns" "proof_mux.json" "public_mux.json"
snarkjs groth16 verify mux.vkey.json "public_mux.json" "proof_mux.json"