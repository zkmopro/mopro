#!/bin/bash

#circom ./keccak256_256_test.circom --r1cs --wasm --sym --output ./target
circom ./examples/circom/keccak256/keccak256_256_test.circom --r1cs --wasm --sym --output ./examples/circom/keccak256/target
