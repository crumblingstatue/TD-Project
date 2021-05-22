#!/bin/sh

xargo build --target=wasm32-unknown-unknown --release
wasm-opt -Os --strip-debug target/wasm32-unknown-unknown/release/td.wasm -o td.wasm
