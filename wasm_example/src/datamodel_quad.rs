//! This modules contains the quad importation and exportation from Sophia and
//! javascript.

extern crate wasm_bindgen;

use crate::datamodel_term::*;

use sophia::term::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;


// ============================================================================
//   ==== IMPORTATION ==== IMPORTATION ==== IMPORTATION ==== IMPORTATION ====

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Quad)]
    pub type JsImportQuad;
    
    #[wasm_bindgen(method, getter)]
    pub fn subject(this: &JsImportQuad) -> JsImportTerm;

    #[wasm_bindgen(method, setter)]
    pub fn set_subject(this: &JsImportQuad, value: &JsImportTerm);

    #[wasm_bindgen(method, getter)]
    pub fn object(this: &JsImportQuad) -> JsImportTerm;

    #[wasm_bindgen(method, setter)]
    pub fn set_object(this: &JsImportQuad, value: &JsImportTerm);

    #[wasm_bindgen(method, getter)]
    pub fn predicate(this: &JsImportQuad) -> JsImportTerm;

    #[wasm_bindgen(method, setter)]
    pub fn set_predicate(this: &JsImportQuad, value: &JsImportTerm);

    #[wasm_bindgen(method, getter)]
    pub fn graph(this: &JsImportQuad) -> JsImportTerm;

    #[wasm_bindgen(method, setter)]
    pub fn set_graph(this: &JsImportQuad, value: &JsImportTerm);

    #[wasm_bindgen(js_name=equals)]
    pub fn quads_equals(this: &JsImportQuad, other_quad: &JsImportQuad);
    
    #[wasm_bindgen(method, getter=getRustPtr)]
    pub fn quads_get_rust_ptr(this: &JsImportQuad) -> *mut SophiaExportQuad;
}


// ============================================================================
//   ==== EXPORTATION ==== EXPORTATION ==== EXPORTATION ==== EXPORTATION ====

/// A SophiaExportQuad owns its data in the form of four RcTerms.
#[wasm_bindgen(js_name = Quad)]
pub struct SophiaExportQuad {
    #[wasm_bindgen(skip)]
    pub _subject: RcTerm,
    #[wasm_bindgen(skip)]
    pub _predicate: RcTerm,
    #[wasm_bindgen(skip)]
    pub _object: RcTerm,
    #[wasm_bindgen(skip)]
    pub _graph: Option<RcTerm>
}

// A SophiaExportQuad is a trivial to implement as a quad
impl sophia::quad::Quad for SophiaExportQuad {
    type TermData = Rc<str>;

    fn s(&self) -> &RcTerm { &self._subject }
    fn p(&self) -> &RcTerm { &self._predicate }
    fn o(&self) -> &RcTerm { &self._object }
    fn g(&self) -> Option<&RcTerm> { self._graph.as_ref() }
}

impl SophiaExportQuad {
    /// Creates a new quad by cloning the passed RcTerms
    pub fn new(s: &RcTerm, p: &RcTerm, o: &RcTerm, g: Option<&RcTerm>) -> SophiaExportQuad {
        SophiaExportQuad {
            _subject: s.clone(),
            _predicate: p.clone(),
            _object: o.clone(),
            _graph: g.cloned()
        }
    }
}

#[wasm_bindgen(js_class = Quad)]
impl SophiaExportQuad {
    #[wasm_bindgen(method, getter)]
    pub fn subject(&self) -> SophiaExportTerm {
        SophiaExportTerm::new(&self._subject)
    }

    #[wasm_bindgen(method, getter)]
    pub fn predicate(&self) -> SophiaExportTerm {
        SophiaExportTerm::new(&self._predicate)
    }

    #[wasm_bindgen(method, getter)]
    pub fn object(&self) -> SophiaExportTerm {
        SophiaExportTerm::new(&self._object)
    }

    #[wasm_bindgen(method, getter)]
    pub fn graph(&self) -> SophiaExportTerm {
        match &self._graph {
            None => SophiaExportTerm::default_graph(),
            Some(term) => SophiaExportTerm::new(term)
        }
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        match &self._graph {
            Some(g) => format!("{0} {1} {2} {3} .",
                            self._subject.n3(), self._predicate.n3(), self._object.n3(), g.n3()),
            None    => format!("{0} {1} {2} .",
                            self._subject.n3(), self._predicate.n3(), self._object.n3())
        }
    }

    #[wasm_bindgen(js_name = equals)]
    pub fn equals(&self, other: Option<JsImportQuad>) -> bool {
        match &other {
            None => false,
            Some(other_quad) => {
                let ptr = &other_quad.quads_get_rust_ptr();
                if ptr.is_null() {
                    self.subject().equals(Some(other_quad.subject()))
                    && self.predicate().equals(Some(other_quad.predicate()))
                    && self.object().equals(Some(other_quad.object()))
                    && self.graph().equals(Some(other_quad.graph()))
                } else {
                    unsafe {
                        if let Some(exported_rust_quad) = ptr.as_ref() {
                            self._subject == exported_rust_quad._subject
                            && self._predicate == exported_rust_quad._predicate
                            && self._object == exported_rust_quad._object
                            && self._graph == exported_rust_quad._graph
                        } else {
                            false
                        }
                    }
                }

                // TODO : Make a SophiaExportTerm that don't clone the items to reuse code without destroying performances
                // or use a cache mechanic (but it is worst)
            }
        }
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_subject(&mut self, other: &JsImportTerm) {
        self._subject = build_rcterm_from_js_import_term(other).unwrap();
    }
    
    #[wasm_bindgen(method, setter)]
    pub fn set_predicate(&mut self, other: &JsImportTerm) {
        self._predicate = build_rcterm_from_js_import_term(other).unwrap();
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_object(&mut self, other: &JsImportTerm) {
        self._object = build_rcterm_from_js_import_term(other).unwrap();
    }
    
    #[wasm_bindgen(method, setter)]
    pub fn set_graph(&mut self, other: &JsImportTerm) {
        self._graph = build_rcterm_from_js_import_term(other);
    }

    #[wasm_bindgen(method, getter=getRustPtr)]
    pub fn quads_get_rust_ptr(&mut self) -> *mut SophiaExportQuad {
        self
    }
}
