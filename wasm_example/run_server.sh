cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
wasm-bindgen --target no-modules target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
npm run serve
