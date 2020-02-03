# Notes

## Lundi 03 Février 2019

### Matin

- Visite des lieux
- Vision globale de Rust et de l'état actuel de Sophia


### Après-midi

- Compilation de rust vers wasm avec fonctions utilisables en js

- https://dev.to/sendilkumarn/rust-and-webassembly-for-the-masses-wasm-bindgen-57fl
- ~/LIRIS/cmd/hello_world


*TL;DR* :



- Installer `cargo install wasm-bindgen-cli`


- Commandes :

    - `cargo new --lib hello_world`

Cargo.toml :

```
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.56"
```

src/lib.rs :

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_world() -> String {
 "Hello World".to_string()
}
```


    - `cargo build --target=wasm32-unknown-unknown`
    - `wasm-bindgen target/wasm32-unknown-unknown/debug/hello_world.wasm --out-dir .`


- Fichiers pour webpack

> `webpack.config.js`

```js
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {     
    entry: './index.js',
    output: {         
        path: path.resolve(__dirname, 'dist'),         
        filename: 'bundle.js',     
    },
    plugins: [         
        new HtmlWebpackPlugin(),         
    ],
    mode: 'development'
};
```

> `package.json`

```json
{
    "scripts": {
        "build": "webpack",
        "serve": "webpack-dev-server"
    },
    "devDependencies": {
        "html-webpack-plugin": "^3.2.0",
        "webpack": "^4.41.5",
        "webpack-cli": "^3.3.10",
        "webpack-dev-server": "^3.10.1"
    }
}
```

> `index.js`

```js
import("./hello_world").then(module => {
    console.log(module.hello_world());
});
```

- Run the server

    - Install dependencies : `npm install`

    - Run the server : `npm run serv`





