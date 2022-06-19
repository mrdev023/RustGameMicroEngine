#!/bin/bash

if [[ $1 == "dev" ]]; then
  ARGS="--dev"
else
  ARGS="--release"
fi

wasm-pack build -t web $ARGS
python3 -m http.server