(function() {
    const __exports = {};
    let wasm;

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
*/
class DataFactory {

    static __wrap(ptr) {
        const obj = Object.create(DataFactory.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_datafactory_free(ptr);
    }
    /**
    */
    constructor() {
        var ret = wasm.datafactory_new();
        return DataFactory.__wrap(ret);
    }
    /**
    * @param {string} value
    * @returns {Term}
    */
    namedNode(value) {
        var ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.datafactory_namedNode(this.ptr, ptr0, len0);
        return Term.__wrap(ret);
    }
    /**
    * @param {string | undefined} value
    * @returns {Term}
    */
    blankNode(value) {
        var ptr0 = isLikeNone(value) ? 0 : passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.datafactory_blankNode(this.ptr, ptr0, len0);
        return Term.__wrap(ret);
    }
    /**
    * @param {string} value
    * @param {string} language
    * @returns {Term}
    */
    literalFromString(value, language) {
        var ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passStringToWasm0(language, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ret = wasm.datafactory_literalFromString(this.ptr, ptr0, len0, ptr1, len1);
        return Term.__wrap(ret);
    }
    /**
    * @param {string} value
    * @param {any} named_node
    * @returns {Term}
    */
    literalFromNamedNode(value, named_node) {
        var ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.datafactory_literalFromNamedNode(this.ptr, ptr0, len0, addHeapObject(named_node));
        return Term.__wrap(ret);
    }
    /**
    * @param {string} value
    * @returns {Term}
    */
    variable(value) {
        var ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.datafactory_variable(this.ptr, ptr0, len0);
        return Term.__wrap(ret);
    }
    /**
    * @returns {Term}
    */
    defaultGraph() {
        var ret = wasm.datafactory_defaultGraph(this.ptr);
        return Term.__wrap(ret);
    }
    /**
    * @param {any} subject
    * @param {any} predicate
    * @param {any} object
    * @param {any | undefined} graph
    * @returns {Quad}
    */
    quad(subject, predicate, object, graph) {
        var ret = wasm.datafactory_quad(this.ptr, addHeapObject(subject), addHeapObject(predicate), addHeapObject(object), isLikeNone(graph) ? 0 : addHeapObject(graph));
        return Quad.__wrap(ret);
    }
    /**
    * @param {any} original
    * @returns {Term}
    */
    fromTerm(original) {
        var ret = wasm.datafactory_fromTerm(this.ptr, addHeapObject(original));
        return Term.__wrap(ret);
    }
    /**
    * @param {any} original
    * @returns {Quad}
    */
    fromQuad(original) {
        var ret = wasm.datafactory_fromQuad(this.ptr, addHeapObject(original));
        return Quad.__wrap(ret);
    }
}
__exports.DataFactory = DataFactory;
/**
* A sample JSTerm to be used in the RDF/JS interface
* David Pojunas
* Pierre-Antoine Champin
*/
class JSDataset {

    static __wrap(ptr) {
        const obj = Object.create(JSDataset.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_jsdataset_free(ptr);
    }
    /**
    */
    constructor() {
        var ret = wasm.jsdataset_new();
        return JSDataset.__wrap(ret);
    }
    /**
    * @returns {DataFactory}
    */
    cloneFactory() {
        var ret = wasm.jsdataset_cloneFactory(this.ptr);
        return DataFactory.__wrap(ret);
    }
    /**
    * @param {string} content
    */
    load(content) {
        var ptr0 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.jsdataset_load(this.ptr, ptr0, len0);
    }
    /**
    * @returns {any}
    */
    getTerms() {
        var ret = wasm.jsdataset_getTerms(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} quad
    */
    add(quad) {
        wasm.jsdataset_add(this.ptr, addHeapObject(quad));
    }
    /**
    * @param {any} quad
    */
    delete(quad) {
        wasm.jsdataset_delete(this.ptr, addHeapObject(quad));
    }
    /**
    * @param {any} quad
    * @returns {boolean}
    */
    has(quad) {
        var ret = wasm.jsdataset_has(this.ptr, addHeapObject(quad));
        return ret !== 0;
    }
    /**
    * @param {any | undefined} subject
    * @param {any | undefined} predicate
    * @param {any | undefined} object
    * @param {any | undefined} graph
    * @returns {JSDataset}
    */
    match(subject, predicate, object, graph) {
        var ret = wasm.jsdataset_match(this.ptr, isLikeNone(subject) ? 0 : addHeapObject(subject), isLikeNone(predicate) ? 0 : addHeapObject(predicate), isLikeNone(object) ? 0 : addHeapObject(object), isLikeNone(graph) ? 0 : addHeapObject(graph));
        return JSDataset.__wrap(ret);
    }
    /**
    * @returns {any}
    */
    quads() {
        var ret = wasm.jsdataset_quads(this.ptr);
        return takeObject(ret);
    }
}
__exports.JSDataset = JSDataset;
/**
*/
class Quad {

    static __wrap(ptr) {
        const obj = Object.create(Quad.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_quad_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    is_connected_to_rust() {
        var ret = wasm.quad_is_connected_to_rust(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {Term}
    */
    get subject() {
        var ret = wasm.quad_subject(this.ptr);
        return Term.__wrap(ret);
    }
    /**
    * @returns {Term}
    */
    get predicate() {
        var ret = wasm.quad_predicate(this.ptr);
        return Term.__wrap(ret);
    }
    /**
    * @returns {Term}
    */
    get object() {
        var ret = wasm.quad_object(this.ptr);
        return Term.__wrap(ret);
    }
    /**
    * @returns {Term}
    */
    get graph() {
        var ret = wasm.quad_graph(this.ptr);
        return Term.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    toString() {
        try {
            wasm.quad_toString(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {any | undefined} other
    * @returns {boolean}
    */
    equals(other) {
        var ret = wasm.quad_equals(this.ptr, isLikeNone(other) ? 0 : addHeapObject(other));
        return ret !== 0;
    }
    /**
    * @param {any} other
    */
    set subject(other) {
        try {
            wasm.quad_set_subject(this.ptr, addBorrowedObject(other));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * @param {any} other
    */
    set predicate(other) {
        try {
            wasm.quad_set_predicate(this.ptr, addBorrowedObject(other));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * @param {any} other
    */
    set object(other) {
        try {
            wasm.quad_set_object(this.ptr, addBorrowedObject(other));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * @param {any} other
    */
    set graph(other) {
        try {
            wasm.quad_set_graph(this.ptr, addBorrowedObject(other));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }
}
__exports.Quad = Quad;
/**
*/
class Term {

    static __wrap(ptr) {
        const obj = Object.create(Term.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_term_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    is_connected_to_rust() {
        var ret = wasm.term_is_connected_to_rust(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {string}
    */
    get termType() {
        try {
            wasm.term_term_type(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {string}
    */
    get value() {
        try {
            wasm.term_value(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} new_value
    */
    set value(new_value) {
        var ptr0 = passStringToWasm0(new_value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.term_set_value(this.ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get language() {
        try {
            wasm.term_language(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} language
    */
    set language(language) {
        var ptr0 = passStringToWasm0(language, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.term_set_language(this.ptr, ptr0, len0);
    }
    /**
    * @returns {Term | undefined}
    */
    get datatype() {
        var ret = wasm.term_datatype(this.ptr);
        return ret === 0 ? undefined : Term.__wrap(ret);
    }
    /**
    * @param {any} named_node
    */
    set datatype(named_node) {
        try {
            wasm.term_set_datatype(this.ptr, addBorrowedObject(named_node));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * @param {any | undefined} other
    * @returns {boolean}
    */
    equals(other) {
        var ret = wasm.term_equals(this.ptr, isLikeNone(other) ? 0 : addHeapObject(other));
        return ret !== 0;
    }
    /**
    * @returns {string}
    */
    toString() {
        try {
            wasm.term_toString(8, this.ptr);
            var r0 = getInt32Memory0()[8 / 4 + 0];
            var r1 = getInt32Memory0()[8 / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
__exports.Term = Term;

function init(module) {
    if (typeof module === 'undefined') {
        let src;
        if (self.document === undefined) {
            src = self.location.href;
        } else {
            src = self.document.currentScript.src;
        }
        module = src.replace(/\.js$/, '_bg.wasm');
    }
    let result;
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_term_new = function(arg0) {
        var ret = Term.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_quad_new = function(arg0) {
        var ret = Quad.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_log_c2cfdc4ec52b14ce = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_termtype_df52cc89b73266e6 = function(arg0, arg1) {
        var ret = getObject(arg1).termType;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_value_bb51eead7a4a4148 = function(arg0, arg1) {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_language_ce2f1802fabf948b = function(arg0, arg1) {
        var ret = getObject(arg1).language;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_datatype_051506832a0fb627 = function(arg0) {
        var ret = getObject(arg0).datatype;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subject_fafd19b657951b96 = function(arg0) {
        var ret = getObject(arg0).subject;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_object_3042f1e4ca4ee79d = function(arg0) {
        var ret = getObject(arg0).object;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_predicate_55046aa42cec6b17 = function(arg0) {
        var ret = getObject(arg0).predicate;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_graph_ebb145b94fe1b3cf = function(arg0) {
        var ret = getObject(arg0).graph;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_3c32f9cd3d7f4595 = function() {
        var ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_push_446cc0334a2426e8 = function(arg0, arg1) {
        var ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if ((typeof URL === 'function' && module instanceof URL) || typeof module === 'string' || (typeof Request === 'function' && module instanceof Request)) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                return response
                .then(r => {
                    if (r.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                        return r.arrayBuffer();
                    } else {
                        throw e;
                    }
                })
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}


    /* ==== ADDED BY js_integrate_polymorphism.py ==== */

    DataFactory.prototype.literal = function(value, languageOrDatatype) {
        if (languageOrDatatype === null || languageOrDatatype === undefined) {
            return undefined;
        } else if (Object.prototype.toString.call(languageOrDatatype) === "[object String]") {
            return this.literalFromString(value, languageOrDatatype);
        } else {
            return this.literalFromNamedNode(value, languageOrDatatype);
        }
    }

    /* ==== END ADDED BY js_integrate_polymorphism.py ==== */

    self.wasm_bindgen = Object.assign(init, __exports);

})();
