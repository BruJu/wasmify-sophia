# ./run_server -> starts a server with a sophia backend
# ./run_server test -> starts unit tests

set -e

if [ $# -ne 0  ]
then
  if [ $1 == "test" ]
  then
    wasm-pack build --target nodejs
    mocha
#  elif [ $1 == "run" ]
#  then
#    pre_compile
#    wasm-bindgen --target no-modules target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
#    npm run serve
  elif [ $1 == "reload" ]
  then
  echo "Not supported"
#    pre_compile
  elif [ $1 == "release" ]
  then
    wasm-pack build --target nodejs
    mocha
    # Running some quick benchark would be nice here (and eventually plotting the evolution)
  else
    echo "Unknown argument"
  fi
else
  echo "Not supported"
#  pre_compile
#  wasm-bindgen --target nodejs target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
#  mocha
#  wasm-bindgen --target no-modules target/wasm32-unknown-unknown/debug/wasm_example.wasm --out-dir .
#  npm run serve
fi
