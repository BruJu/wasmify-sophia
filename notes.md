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



- Problème avec webpack-cli, solution : `npm uninstall webpack-cli`



## Mardi 04 Février 2019

### Matin

- Etude de la documentation de Sophia

### Après-midi

- Etude de la documentation de rdf.js.org et de wasm_bindgen afin d'anticiper
les difficultés à la conception d'une librarie


**Objectif : Créer des adapteurs de classe de Rust pour proposer une interface
se conformant à la spec JS**


*Literal*

Dans Sophia : `Literal(T, LiteralKind<T>)`

> If the literal has a language, its datatype has the IRI "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".
> Otherwise, if no datatype is explicitly specified, the datatype has the IRI "http://www.w3.org/2001/XMLSchema#string". 



*DefaultGraph*

Est const et doit être assigné si dans le quad (ou si c'est un triplet),
g matche Option::None




## Mercredi 05 Février 2019

### Matin

- Continuation de l'essai de prototyper Term

### Après-midi

- Reflexion sur les Box / Arc / Rc pour accéder à leurs string

```cpp
#[wasm_bindgen]
pub fn azaf() -> String {
    let lama = Box::new(String::from("Lama"));
    let z = String::from(lama.as_str());
    print_type(lama); // Lama still exists
    print_type(z);
    z
}
```

- Thomas : "Pourquoi tu utilises Webpack ?" -> c'est vrai que dans mon
contexte ça ne sert à rien.


## Jeudi 06 Février 2019

### Matin

- Nouvelle idée : utiliser des closures pour donner accés aux membres

Mais je n'arrive pas à mettre des boxed closure ni des boxed attributs


- Lecture des papiers de HyLAR













