# This script compiles the wasm_example files and launch a simple server
# to test the implementation.

set -e
cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
wasm-bindgen --target no-modules target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
# python3 js_integrate_polymorphism.py
npm run serve
