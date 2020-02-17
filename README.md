# Portable Reasoning in Web Assembly

This repository is currently a compilation of my experiments from my internship
at [LIRIS](https://liris.cnrs.fr/) in the team
[TWEAK](https://liris.cnrs.fr/equipe/tweak).

The project I'm working on is to reasoning using a RDF API in Javascript
that resorts to
[the Sophia library written in Rust](https://github.com/pchampin/sophia_rs).

See [Project REPID](https://projet.liris.cnrs.fr/repid/).

As I am still exploring Rust / Web Assembly / tryining to learn web
technologies, this repository does not currently expose a clear tool with a
proper README.

## Sophia interface that matches rdF.js.org specification

The first goal is to enable javascript users to use *Sophia* as a backend.

### How to "use"

*Normally*

- To start a web server that resorts to the Sophia backend :
`./wasm_example/run_server.sh`

*Unit Testing*

I am currently working on integrating Unit Tests

- `npm i @rdfjs/namespace`

- `npm run test`

---

## Temporary1 link heap

1 This word is often a lie

### Iterators / Symbol.Iterator

- https://github.com/rustwasm/wasm-bindgen/issues/1036

- https://rustwasm.github.io/wasm-bindgen/api/js_sys/struct.Iterator.html



---

## Links that might be useful or not

### Rust / Web Assembly

#### Official Documentation / Links I should stop losing

| Link    | Description |
| ------- | ----------- |
| https://doc.rust-lang.org/book/ | Rust book |
| https://doc.rust-lang.org/std/ | `std` intensifies |
| https://rustwasm.github.io/docs/wasm-bindgen/ | wasm_bindgen |



#### Wasm bindgen

| Link    | Description |
| ------- | ----------- |
| https://dev.to/sendilkumarn/rust-and-webassembly-for-the-masses-wasm-bindgen-57fl | A basic tutorial on wasm_bindgen |
| https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html | A potential way to make the code faster |
| https://rustwasm.github.io/wasm-bi:bindgen/reference/cli.html | We are actually not supposed to use the command wasm_bindgen but wasm-pack |
| https://github.com/rustwasm/wasm-pack | wasm pack repository |

#### Other documentation

| Link    | Description |
| ------- | ----------- |
| https://play.rust-lang.org/ | Rust Online Compiler. Doesn't offer suggestion |
| https://learnxinyminutes.com/docs/rust/ | Could be used as a quick cheatsheet but I prefer using the documentation with the search feature |
| https://github.com/pchampin/rust-iut/ | A Rust course teached to 2 years students. |



### RDF


| Link    | Description |
| ------- | ----------- |
| https://www.w3.org/TeamSubmission/turtle/ | Turtle Spec |


### RDF JS interface


|   Title    |            Specification            |           Basic implementation           |
| ---------- | ----------------------------------- | ---------------------------------------- |
| Data Model | https://rdf.js.org/data-model-spec/ | https://github.com/rdfjs-base/data-model |
| Dataset    | https://rdf.js.org/dataset-spec/    | https://github.com/rdfjs-base/dataset    |
| Stream     | https://rdf.js.org/stream-spec/     |                                          |



### Internship

- https://projet.liris.cnrs.fr/repid/2019/stage-raisonnement-portable/fr/

- HyLAR : https://github.com/ucbl/HyLAR-Reasoner#readme
    - https://hal.archives-ouvertes.fr/hal-01154549/file/hylar.pdf
    - https://hal.archives-ouvertes.fr/hal-01276558/file/Demo_www2016.pdf
    - http://mmrissa.perso.univ-pau.fr/pub/Accepted-papers/2018-TheWebConf-RoD.pdf

- Sophia :
    - Master : https://github.com/pchampin/sophia_rs
    - Clone : https://github.com/bruju/sophia_rs

- wasm_example : https://github.com/davidavzP/wasm_example

- rflib.js : http://linkeddata.github.io/rdflib.js/doc/


===

## Content I should clean / translate

### Test suite pour RDF JS:

Community group RDF JS:

- Faire une test suite : https://github.com/rubensworks/jest-rdf

- https://www.w3.org/community/rdfjs/



### Misc

- http://iswc2011.semanticweb.org/fileadmin/iswc/Papers/Workshops/SSWS/Emmons-et-all-SSWS2011.pdf


- https://github.com/Tpt/oxigraph/

- https://karthikkaranth.me/blog/my-experience-with-rust-plus-wasm/


## Vrac de notes et de remarques à moi-même


> comment implémenter ça ?

```rust
impl BJQuad {
    pub fn new<T>(cloned_quad: &T) -> BJQuad 
    where T: Graph<TermData = RcTerm> {
        BJQuad {
            subject: cloned_quad.s().clone(),
            predicate: cloned_quad.p().clone(),
            object: cloned_quad.o().clone(),
            graph: cloned_quad.g().clone(),
        }
    }
}
```

```rust
fn toto<T>(...) where
T : Graph + foo,
  <T as Graph>::TermData
```

> Comme Rust est un langage orienté expression on peut refactor de cette
manière 

```rust
    pub fn quad(&self, subject: JssTerm, predicate: JssTerm, object: JssTerm, graph: Option<JssTerm>) -> BJQuad {
        match graph {
            None => BJQuad::new_by_move(
                build_rcterm_from_jss_term(&subject).unwrap(),
                build_rcterm_from_jss_term(&predicate).unwrap(),
                build_rcterm_from_jss_term(&object).unwrap(),
                None
            ),
            Some(g) => BJQuad::new_by_move(
                build_rcterm_from_jss_term(&subject).unwrap(),
                build_rcterm_from_jss_term(&predicate).unwrap(),
                build_rcterm_from_jss_term(&object).unwrap(),
                build_rcterm_from_jss_term(&graph));
            )
        }
    }
```

en

```rust
    pub fn quad(&self, subject: JssTerm, predicate: JssTerm, object: JssTerm, graph: Option<JssTerm>) -> BJQuad {
        BJQuad::new_by_move(
            build_rcterm_from_jss_term(&subject).unwrap(),
            build_rcterm_from_jss_term(&predicate).unwrap(),
            build_rcterm_from_jss_term(&object).unwrap(),
            match graph {
                None => None,
                Some(g) => build_rcterm_from_jss_term(&g)
            }
        )
    }
```

> Problème pour les litéraux : la spec veut que l'on puisse passer un string ou
un named node. C'est pas possible par défaut.

~~Quelques pistes à explorer :

- Générer du code typescript https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html
- Faire de la reflexion / introspection https://docs.rs/js-sys/0.3.35/js_sys/Reflect/index.html
    - Creuser la piste de l'introspection


    - https://github.com/rustwasm/wasm-bindgen/issues/1906

Pour le moment j'ajoute une fonction javascript à la main dans le code généré
qui fait la vérification et appelle la bonne fonction wasm
~~

*Solution* : `JsValue`


> J'aimerais bien avoir des versions optimisées des opérations sur les
adapteurs si il renvoie un adapteur (au lieu de devoir supposer que les seules
hypothèses que l'on peut faire est qu'on a reçu un truc répondant à la norme
RDFJS)


> Une idée que j'ai serait de pouvoir manier soit un FastDataset, soit un
dataset plus adapté selon si l'utilisateur vient de faire beaucoup de match ou
beaucoup de modifications du graphe (sans que l'utilisateur ne s'en rende
compte)

> Retourner this

    - La piste du script Python est bof (autant ajouter à la main lors d'une
    release les quelques lignes à générer)



