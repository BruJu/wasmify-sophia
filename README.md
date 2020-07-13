# Wasm-ify Sophia

This repository is a work in progress to export datasets
implementing the `Dataset` trait from the [Sophia Rust too l kit][Sophia] to the Javascript world, using Web Assembly
and by exposing an [RDF.JS] compliant implementation.

## sophia_wasm

The crate [sophia_wasm](sophia-wasm) provides an exportation of terms and quads from [Sophia] using [wasm_bindgen] and [wasm-pack]. It also provides basic implementation of exportation of datasets into Javascript, that tries to be compliant with the [RDF.JS] specification.

Macro and tools are also provided to export to Javascript other implementations of the `Dataset` trait, as long as the exported dataset implements `Default` and `MutableDataset`.

The exported datasets can be used out-of-the-box, which tries to implement, but does not adhere completely, to the [RDF.JS Dataset][RDFJSDataset] specification.

A wrapper class is provided to address some of the issues of the defaultly exported structures (memory leaks and some details to be compliant like being able to chain the calls to the `add` method).


## bjdatasets

The folder [bjdatasets](bjdatasets) exposes some implementation of [the Sophia Dataset trait][Sophia].

- `TreedDataset`, a dataset that resorts on multiple trees to do efficient quad research
- `FullIndexDataset`, a dataset that stores for every possible pattern every corresponding quad
- `VecOrDataset<D>`, a dataset that can use either a vector of quads or another Dataset structure


`TreedDataset` is used as the base structure of [WasmTree][WasmTree], a project to implement the [RDF.JS specification][RDFJSDataset] using Web Assembly but without resorting to [Sophia].

## Build

***TODO***





## Problems / Issues / I can't find the word I want to write here

Currently, [WasmTree] is faster than the tested exportations. So if you want to just use Web Assembly to improve the performances of your Javascript application, you should consider using it instead.


## License and funding

This work is distributed under the [MIT License](LICENSE.md).

This project has been funded by the [REPID Project](https://projet.liris.cnrs.fr/repid/) during my internship in the [TWEAK team](https://liris.cnrs.fr/equipe/tweak) at [LIRIS](https://liris.cnrs.fr/).

[Sophia]: https://github.com/pchampin/sophia_rs
[WasmTree]: https://github.com/BruJu/WasmTreeDataset
[RDFJSDataset]: https://rdf.js.org/dataset-spec/
[RDF.JS]: https://rdf.js.org/
[wasm_bindgen]: https://rustwasm.github.io/docs/wasm-bindgen/
[wasm-pack]: https://rustwasm.github.io/docs/wasm-pack/
