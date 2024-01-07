#!/bin/sh

./scripts/wasm-bindgen-macroquad.sh interactive_vsa $1

# https://github.com/WebAssembly/wabt
# wasm-strip docs/wbindgen/simple_gravity.wasm
mv docs/wbindgen/interactive_vsa_bg.wasm docs/
mv docs/wbindgen/interactive_vsa.js docs/

if [ "$1" = "serve" ]
then
    # cargo install basic-http-server
    basic-http-server docs
fi
