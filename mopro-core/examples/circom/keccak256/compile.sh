#!/bin/bash

mkdir target
circom ./keccak256_256_test.circom --r1cs --wasm --sym --output ./target