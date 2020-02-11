# Notes


# Semaine 1

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


- Tentative d'étudierl a structure de rdflib js, infructueuse


### Après midi


- Implémenttion basique de la spec rdf js pour des Box<str>


```rust
use std::rc::Rc;

fn main() {
    let heyyy = String::from("Heyyyyy");
    let rced_heyyy: Rc<String> = heyyy.into();

    println!("{} !", &rced_heyyy);
}

```

- Les IriData<T> ne sont pas mutables. Les Term ne sont pas mutables non plus

- Une piste pour muter les enum : https://users.rust-lang.org/t/mutate-enum-in-place/18785/3

- Début d'implémentation des termes pour RcTerm



## Vendredi 07 Février 2019

### Matin

- Support de Web Assembly par Oxygraphe :

> Thomas : "OxyGraphe a l'air de supporter le wasm"

    - https://github.com/Tpt/oxigraph/issues/16

    - https://github.com/Tpt/oxigraph/pull/22
    
Pas de support de l'API standard rdf.js mais support avec js-sys (connecte des
objets WebAssembly avec des objets Rust

- Clarification du sujet -> "Proposer une interface rdf-js-like basée sur Sophia
ne collant pas spécifiquement à la documentation"

- "C'est le projet REPID"

- Réunion d'équipe : KATIE (Beatrice Fuchs) -> concepts intéressant pour
nettoyer les données d'un de mes projets personnels (mais HS avec ce stage
à priori)

### Après-midi

- On se fixe défintivement sur le choix de faire une interface pour RcTerm.

- PoC de la classe Term.


### Bilan de la semaine

- Principalement de la lecture de documentation pour essayer de se rendre compte
de ce qu'il est possible de faire avec rust, sophia, wasm_bindgen et la spec
de rdfjs.

- Objectif pour la fin de la semaine prochaine : finir d'avoir une api RDFJS
pour le DataModel et le DatasetModel + avoir bien déblayé le Stream Interface.


# Semaine 2

## Lundi 10 Février 2020

### Matin

- Réunion :
    - Le choix de faire des triplets a été fait pour coller à la spec RDF (il
    serait trop éloigné de définir les dataframe comme un semble de quads, et de
    définir la notion de triplets et de graphe à partir de quad et dataframe
    sachant que la spec fait totalement l'inverse)

    - Pour un utilisateur javascript, il n'y a pas de raison de manipuler des
    triplets et des graphes car la spec RDFJS ne mentionne pas ces types : pas
    besoin de manipuler les adapteurs de triplets. Surtout que PAC a confirmé
    que face au coût de manipulation des objets JS, l'optimisation mémoire
    apportée par les triplets est mineure.


- Continuation de l'implémentation de term; en particulier sur l'importation
de termes issus de Javascript

### Après-midi

- Fin implémentation des BJTerm / JSTerm

- Début de test

- Utilisation de Fastdataset au lieu de graphes


## Mardi 11 Février 2020

### Matin

- Liste de termes

- SUPER affichage dans une page html au lieu de la console JS

- Default Graph

- Début quads

### Après-midi


- Question : comment implémenter ça ?

```rust
impl BJQuad {
    pub fn new<T>(cloned_quad: &T) -> BJQuad 
    where T: T::TermData == RcTerm {
        BJQuad {
            subject: cloned_quad.s().clone(),
            predicate: cloned_quad.p().clone(),
            object: cloned_quad.o().clone(),
            graph: cloned_quad.g().clone(),
        }
    }
}
```









































