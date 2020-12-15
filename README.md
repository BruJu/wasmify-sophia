# Wasm-ify Sophia

This repository propose an architecture to export to Javascript classes
that implements the [Dataset API of the Sophia tool kit][Sophia].

The Rust code is compiled to Web Assembly and the resulting Javascript
class is (almost) [RDF.JS] Dataset compliant.

It also provides some new custom Sophia Dataset implementation.




## sophia_wasm

The crate [sophia_wasm](sophia-wasm) provides an exportation of terms and quads from [Sophia] using [wasm_bindgen] and [wasm-pack]. It also provides basic implementation of exportation of datasets into Javascript, that tries to be compliant with the [RDF.JS] specification.

Macro and tools are also provided to export to Javascript other implementations of the `Dataset` trait, as long as the exported dataset implements `Default` and `MutableDataset`.

The exported datasets can be used out-of-the-box, which tries to implement, but does not adhere completely, to the [RDF.JS Dataset][RDFJSDataset] specification.

A wrapper class is provided to address some of the issues of the defaultly exported structures (memory leaks and some details to be compliant like being able to chain the calls to the `add` method).


## bjdatasets

The crate [bjdatasets](bjdatasets) exposes some implementation of [the Sophia Dataset trait][Sophia].

- `TreeDataset`, a dataset that resorts on multiple trees. By storing quad in different orders, it provides efficient quad research (see *identifier-tree*)
- `FullIndexDataset`, a dataset that stores for every possible pattern every corresponding quad
- `VecOrDataset<D>`, a dataset that can use either a vector of quads or another Dataset structure


## identifier-forest

The crate [identifier-forest](identifier-forest) provides a forest structure able to store quads in the form of 4 identifiers (that can be mapped to actual terms using an external library).

The main features of the forest are :
- One tree is built on creation.
- Up to 5 over trees can be spawned to store the identifier quads in different orders
- The 6 trees provides optimal pattern maching for all kind of patterns SPOG.
- While the current context is RDF Dataset heavy, it may be possible to be used in other context.

`identifier-forest` is used both as the base structure of :
- [WasmTree][WasmTree], another repository which implements the [RDF.JS specification][RDFJSDataset] using Web Assembly but without resorting to [Sophia].
- The `TreeDataset` implementation in the `bjdatasets` crate.




## Build

***TODO***

### Required

- Rust / [wasm_bindgen]
- [wasm-pack]
- mocha (`sudo npm install -g mocha`)

### Run tests

Rust : ***TODO***

Javascript :
- `cd sophia-wasm`
- `npm install`
- `./run_server.sh test`


***TODO : Rename or get rid of run_server (it doesn't actually run a server)**


## Issue

Currently, [WasmTree] is faster than the tested exportations. So if you want to just use Web Assembly to improve the performances of your Javascript application, you should consider using it instead.


## License and funding

This work is distributed under the [MIT License](LICENSE).

This project has been funded by the [REPID Project](https://projet.liris.cnrs.fr/repid/) during my internship in the [TWEAK team](https://liris.cnrs.fr/equipe/tweak) at [LIRIS](https://liris.cnrs.fr/).

[Sophia]: https://github.com/pchampin/sophia_rs
[WasmTree]: https://github.com/BruJu/WasmTreeDataset
[RDFJSDataset]: https://rdf.js.org/dataset-spec/
[RDF.JS]: https://rdf.js.org/
[wasm_bindgen]: https://rustwasm.github.io/docs/wasm-bindgen/
[wasm-pack]: https://rustwasm.github.io/docs/wasm-pack/
