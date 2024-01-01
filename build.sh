#!/bin/sh

cargo build --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/debug/interactive_vsa.wasm ./docs
wasm-bindgen ./docs/interactive_vsa.wasm --out-dir ./docs/pkg --target no-modules
