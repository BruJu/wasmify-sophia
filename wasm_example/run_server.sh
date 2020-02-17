# This script compiles the wasm_example files and launch a simple server
# to test the implementation.

set -e

# TODO : if test


cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
wasm-bindgen --target nodejs target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
mocha

# TODO : if nothing

# cargo +nightly build --target wasm32-unknown-unknown
# wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
# wasm-bindgen --target no-modules target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
# npm run serve
