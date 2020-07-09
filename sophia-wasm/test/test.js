"use strict";

let wasm_example = require('./../pkg/sophia_wasm.js');
let sophia_wasm_wrapped = require('./../pkg/wrapper');

let DataFactory = wasm_example.DataFactory;

function basic(className) {
    return {
        'init': function (quads) {
            const ds = new className();
            if (quads !== undefined) {
                ds.addAll(quads);
            }
            return ds;
        },
        'isIterable': false
    };
}

function wrapped(className) {
    return {
        'init': function (quads) {
            const ds = new sophia_wasm_wrapped.SophiaDatasetWrapper(new className());
            if (quads !== undefined) {
                ds.addAll(quads);
            }
            return ds;
        },
        'isIterable': true
    };
}

let datasets = {
    'TreedDataset': basic(wasm_example.TreedDataset),
    'FastDataset': basic(wasm_example.FastDataset),
    'LightDataset': basic(wasm_example.LightDataset),
    'FullIndexDataset': basic(wasm_example.FullDataset),
    'WrappedTreedDataset': wrapped(wasm_example.TreedDataset),
    'WrappedFastDataset': wrapped(wasm_example.FastDataset),
    'WrappedLightDataset': wrapped(wasm_example.LightDataset),
    'WrappedFullIndexDataset': wrapped(wasm_example.FullDataset),
};

// ToA variants (like TreedDatasetToA) and ArrayDataset are
// not tested because we know that they are not sets (so they
// can contain duplicates quads)

require('.')(DataFactory, datasets);
