# ./run_server -> starts a server with a sophia backend
# ./run_server test -> starts unit tests

set -e

pre_compile() {
  cargo +nightly build --target wasm32-unknown-unknown
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
  elif [ $1 == "reload" ]
  then
    pre_compile
  elif [ $1 == "release" ]
  then
    cargo +nightly build --target wasm32-unknown-unknown --release
    wasm-bindgen target/wasm32-unknown-unknown/release/wasm_example.wasm --out-dir .
    wasm-bindgen --target nodejs target/wasm32-unknown-unknown/release/wasm_example.wasm --out-dir .
    mocha
    # Running some quick benchark would be nice here (and eventually plotting the evolution)
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
