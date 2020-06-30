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


## Sophia interface that matches rdf.js.org specification

The first goal was to enable javascript users to use *Sophia* as a backend.

### Getting Started

- "Install rust, nodejs and npm"

```sh
rustup toolchain install nightly
cargo +nightly install wasm-bindgen-cli
# rustup target add wasm32-unknown-unknown (Not sure if it is required)
rustup +nightly target add wasm32-unknown-unknown
npm install

# For unit tests
npm i @rdfjs/namespace
npm i mocha -g
```

### How to "use"

*Normally*

- To start a web server that resorts to the Sophia backend :
`./wasm_example/run_server.sh`

*Unit Testing*

- `./wasm_example/run_server.sh test`


### Frequently Encoutered Issues

- RLS can't find crates

reinstall rls vscode plugin

`rustup update` and restart VSCode


## Wasm based datasets

I wrote [this gist which is a compilation of links about why Web Assembly or Javascript is faster than the other one](https://gist.github.com/BruJu/351bbb74216cee6d34aae7938586dca7).

I have yet to explore all the solutions.

But as strings are slow in my benchmark, I (successfully) wrote a [RDF JS compliant DatasetCore class ](https://github.com/BruJu/WasmTreeDataset) which uses both Javascript and Rust.


---

## Temporary1 link heap

1 This word is often a lie

- https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/inspectable.html

- https://github.com/rustwasm/wasm-bindgen/tree/master/examples


### Blank Node Normalization

https://rdf.js.org/dataset-spec/#datasetcore-interface : "Blank Nodes will be normalized."

- http://json-ld.github.io/normalization/spec/


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
| https://rustwasm.github.io/wasm-bindgen/reference/cli.html | We are actually not supposed to use the command wasm_bindgen but wasm-pack |
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
| https://github.com/rubensworks/jest-rdf   | How to build a Jest test suite for RDF |
| https://www.w3.org/community/rdfjs/       | RDF JS Work group |
| http://iswc2011.semanticweb.org/fileadmin/iswc/Papers/Workshops/SSWS/Emmons-et-all-SSWS2011.pdf | Article on RDF Literal Data Types |

### RDF JS interface


|   Title    |            Specification            |           Basic implementation           |
| ---------- | ----------------------------------- | ---------------------------------------- |
| Data Model | https://rdf.js.org/data-model-spec/ | https://github.com/rdfjs-base/data-model |
| Dataset    | https://rdf.js.org/dataset-spec/    | https://github.com/rdfjs-base/dataset    |
| Stream     | https://rdf.js.org/stream-spec/     |                                          |



### Indexing

- http://wifo5-03.informatik.uni-mannheim.de/benchmarks-200801/


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

- My script to synchronize my sophia_rs fork and use the latest version when compiling : https://gist.github.com/BruJu/eade23cd8fa94e1543898cb30ab02ff8


### Other people that do related things


| Link | Description |
| ---- | ----------- |
| https://github.com/Tpt/oxigraph/ | Another rdf implementation in rust : Oxygraph |
| https://karthikkaranth.me/blog/my-experience-with-rust-plus-wasm/ | A feedback on rust + wasm |

---

## Rust snippets / Observations


> Condition on genetic terms

```rust
impl BJQuad {
    pub fn new<T>(cloned_quad: &T) -> BJQuad 
    where T: Graph<TermData = RcTerm> {
        /* blabla */
    }
}
```

or

```rust
fn toto<T>(...) where
T : Graph + Foo,
  <T as Graph>::TermData
```

To answer the question "where does TermData comes from ?"


> I would like to optimize the operations on JsImport if the received object is
a RustExport

> We could also change the type of dataset depending on if we are currently
are doing a lot of adds or a lot of matches

> In some function, I'd like to return self to let the user chain its call

*Problem* : wasm_bindgen can't return references.

*Doesn't work* :

- `pub fn add(self) -> MyClass { self }` because if we don't assign we lose
the instance

- `pub fn add(self) -> *MyClass { *self }` returns a number that have no
sense for the user and that he can't use.

*Bad* ;

- Python script that modifies the generated javascript code : we can just
manually add the `return this;` in the ~4 functions that requires it

> Polymorphism on import / export types

For certains methods, we would like to have 3 possibles behaviors.

For example :

```rust
impl SomeExportedStruct {
    pub fn equals(&self, other: &SomeExportedStruct) {
        // Some code
    }

    pub fn equals(&self, other: &SomeImportedStruct) {
        // Some code that is slower because we have to ask Javascript the fields
    }

    pub fn equals(&self, other: JsValue) {
        // Some other thing
        false
    }
}
```


*Concrete snippet*

```rust
    #[wasm_bindgen(js_name = equals)]
    pub fn equals(&self, other: JsValue) -> bool {
        if other.is_null() || other.is_undefined() {
            return false;
        }

        if let Some(exported_quad) = other.dyn_ref::<SophiaExportQuad>() {
            self._subject == exported_quad._subject
            && self._predicate == exported_quad._predicate
            && self._object == exported_quad._object
            && self._graph == exported_quad._graph
        } else {
            let other: JsImportQuad = other.into();

            self.subject().equals(Some(other.subject()))
            && self.predicate().equals(Some(other.predicate()))
            && self.object().equals(Some(other.object()))
            && self.graph().equals(Some(other.graph()))
        }
    }
```

Currently, we can simulate this behavior by using pointers and a getter to potentially get a pointer from this rust structure (other implementations will probably return undefined when getting this field which will be casted into 0).

Note that if we have a function that requires an exported type, the one who performs the check if Javascript using (paramter instanceof Class) and a pointer in passed to wasm.

