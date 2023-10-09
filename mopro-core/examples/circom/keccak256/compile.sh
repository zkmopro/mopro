#!/bin/bash

#circom ./keccak_256_256_test.circom --r1cs --wasm --sym --output ./target
circom ./examples/circom/keccak256/keccak_256_256_test.circom --r1cs --wasm --sym --output ./examples/circom/keccak256/target
