# ./run_server -> starts a server with a sophia backend
# ./run_server test -> starts unit tests

set -e

pre_compile() {
  cargo +nightly build --target wasm32-unknown-unknown
  wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
  wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
}


if [ $# -ne 0  ]
then
  if [ $1 == "test" ]
  then
    pre_compile
    wasm-bindgen --target nodejs target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
    mocha
  elif [ $1 == "run" ]
  then
    pre_compile
    wasm-bindgen --target no-modules target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
    npm run serve
  else
    echo "Unknown argument"
  fi
else
  pre_compile
  wasm-bindgen --target nodejs target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
  mocha
  wasm-bindgen --target no-modules target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
  npm run serve
fi
