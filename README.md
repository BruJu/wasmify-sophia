# Raisonnement portable en Web Assembly

Ce dépôt est actuellement un vrac de mes expérimentations dans le cadre de mon
stage au [https://liris.cnrs.fr/](LIRIS) au sein de l'équipe
[https://liris.cnrs.fr/equipe/tweak](TWEAK). L'objectif de ce stage est de
permettre de faire du raisonnement RDF en Javascript se reposant sur
[https://github.com/pchampin/sophia_rs](Sophia).

https://projet.liris.cnrs.fr/repid/


- Start a web server that resorts to the Sophia backend : `./wasm_example/run_server.sh`


## Liens utiles


### Rust

- https://play.rust-lang.org/ (n'offre pas de suggestion -> c'est nul)

- https://learnxinyminutes.com/docs/rust/

- https://doc.rust-lang.org/std/keyword.struct.html

- https://github.com/pchampin/rust-iut/


### Spécification RDF

- - https://www.w3.org/TeamSubmission/turtle/





### Sophia vers une librairie répondant à la norme de rdf.js.org

- Spécification attendue pour une librairie JS : https://rdf.js.org

|   Titre    |            Spécification            |       Implémentation basique en TS       |
| ---------- | ----------------------------------- | ---------------------------------------- |
| Data Model | https://rdf.js.org/data-model-spec/ | https://github.com/rdfjs-base/data-model |
| Dataset    | https://rdf.js.org/dataset-spec/    | https://github.com/rdfjs-base/dataset    |
| Stream     | https://rdf.js.org/stream-spec/     |                                          |





### En lien avec le sujet / les ressources de base

- Sujet du stage : https://projet.liris.cnrs.fr/repid/2019/stage-raisonnement-portable/fr/

- HyLAR : https://github.com/ucbl/HyLAR-Reasoner#readme
    - https://hal.archives-ouvertes.fr/hal-01154549/file/hylar.pdf
    - https://hal.archives-ouvertes.fr/hal-01276558/file/Demo_www2016.pdf
    - http://mmrissa.perso.univ-pau.fr/pub/Accepted-papers/2018-TheWebConf-RoD.pdf

- Sophia :
    - Master : https://github.com/pchampin/sophia_rs
    - Clone : https://github.com/bruju/sophia_rs

- wasm_example : https://github.com/davidavzP/wasm_example

- rflib.js : http://linkeddata.github.io/rdflib.js/doc/



### Web Assembly - Rust

- Un tutoriel qui marche pour commencer : https://dev.to/sendilkumarn/rust-and-webassembly-for-the-masses-wasm-bindgen-57fl

- Documentation de wasm-bindgen : https://rustwasm.github.io/docs/wasm-bindgen/

    - Mesure de performances
    
        - https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html


#### Wasm bind gen


- https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/getter-and-setter.html

Normalement on est pas censé utiliser la commande `wasm-bindgen`

- https://rustwasm.github.io/wasm-bindgen/reference/cli.html
    
- https://github.com/rustwasm/wasm-pack





### Misc

- http://iswc2011.semanticweb.org/fileadmin/iswc/Papers/Workshops/SSWS/Emmons-et-all-SSWS2011.pdf


- https://github.com/Tpt/oxigraph/


## Vrac de notes et de remarques à moi-même


> comment implémenter ça ?

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

Quelques pistes à explorer :

- Générer du code typescript https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html
- Faire de la reflexion / introspection https://docs.rs/js-sys/0.3.35/js_sys/Reflect/index.html

Pour le moment j'ajoute une fonction javascript à la main dans le code généré
qui fait la vérification et appelle la bonne fonction wasm

> J'aimerais bien avoir des versions optimisées des opérations sur les
adapteurs si il renvoie un adapteur (au lieu de devoir supposer que les seules
hypothèses que l'on peut faire est qu'on a reçu un truc répondant à la norme
RDFJS)


> Une idée que j'ai serait de pouvoir manier soit un FastDataset, soit un
dataset plus adapté selon si l'utilisateur vient de faire beaucoup de match ou
beaucoup de modifications du graphe (sans que l'utilisateur ne s'en rende
compte)




