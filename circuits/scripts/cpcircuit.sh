#!/bin/bash

FULL_PATH=$1
CIRCUIT_NAME=$(basename $FULL_PATH)
CIRCUIT_NAME=${CIRCUIT_NAME:=test}

set -e # stop for errors

mkdir -p ../ui/public/circuits

cp ${FULL_PATH}_obj/${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm ../ui/public/circuits/${CIRCUIT_NAME}.wasm
cp ${FULL_PATH}_obj/${CIRCUIT_NAME}_final.zkey ../ui/public/circuits/${CIRCUIT_NAME}_final.zkey
