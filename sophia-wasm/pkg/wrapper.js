"use strict";
// This file provides methods to wrap a Sophia Exported Dataset
// to improve performances and remove memory leaks

// TODO : a.union(b) where both a and b are wrapping the same class

let n3 = require("n3");

function rebuild_quad(quad) {
    //return n3.DataFactory.fromQuad(quad);
    return n3.DataFactory.quad(
        quad.subject,
        quad.predicate,
        quad.object,
        quad.graph
    );
}

function remakeFilter(filterFunction) {
    return function (wasm_quad) {
        let js_quad = rebuild_quad(wasm_quad);
        wasm_quad.free();
        return filterFunction(js_quad);
    }
}

class SophiaDatasetWrapper {
    constructor(wrapped) {
        this.base = wrapped;
    }

    // == Improve
    addAll(quads) {
        const writer = new n3.Writer({ format: 'N-Quads' });
        for (let quad of quads) {
            writer.addQuad(quad);
        }
        let content = undefined;
        writer.end((_err, result) => { content = result; })

        while (content === undefined) {
            // We should never pass here as N3.Writer without an output stream should be synchrone
        }

        this.base.addNQuads(content);
    }

    toArray() {
        let nquads = this.base.toNQuads();
        let k = new n3.Parser({ format: 'N-Quads' }).parse(nquads);
        return k;
    }

    forEach(quadRunIteratee) {
        for (let quad of this) {
            quadRunIteratee(quad);
        }
    }
    
    // == Create
    [Symbol.iterator]() {
        return this.toArray()[Symbol.iterator]();
    }

    // == Fix
    add(quad) {
        this.base.add(quad);
        return this;
    }

    delete(quad) {
        this.base.delete(quad);
        return this;
    }

    deleteMatches(subject, predicate, object, graph) {
        this.base.deleteMatches(subject, predicate, object, graph);
        return this;
    }

    // == Unleak
    some(filter_function) {
        let func = remakeFilter(filter_function);
        return this.base.some(func);
    }

    every(filter_function) {
        let func = remakeFilter(filter_function);
        return this.base.every(func);
    }

    filter(filterFunction) {
        let func = remakeFilter(filterFunction);
        return new SophiaDatasetWrapper(this.base.filter(func));
    }

    reduce(reducer, initial_value) {
        let newReducer = (acc, wasmQuad) => {
            let jsQuad = rebuild_quad(wasmQuad);
            wasmQuad.free();
            return reducer(acc, jsQuad);
        };

        return this.base.reduce(newReducer, initial_value);
    }

    // Transmit
    free() {
        this.base.free();
    }

    has(quad) {
        return this.base.has(quad);
    }

    get size() {
        return this.base.size;
    }

    toString() {
        return this.base.toString();
    }

    // Rewrap
    match(subject, predicate, object, graph) {
        return new SophiaDatasetWrapper(this.base.match(subject, predicate, object, graph));
    }

    map(mapFunction) {
        return new SophiaDatasetWrapper(this.base.map(mapFunction));
    }

    // Functions that should try to unwrap the other
    difference(other) {
        return new SophiaDatasetWrapper(this.base.difference(other));
    }

    intersection(other) {
        return new SophiaDatasetWrapper(this.base.intersection(other));
    }

    union(other) {
        return new SophiaDatasetWrapper(this.base.union(other));
    }

    equals(other) {
        return this.base.equals(other);
    }

    contains(other) {
        return this.base.contains(other);
    }
}


module.exports.SophiaDatasetWrapper = SophiaDatasetWrapper;
