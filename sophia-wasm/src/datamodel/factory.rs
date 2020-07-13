//! Imports a static factory that is able to instanciate the different eleents
//! described by the RDF.JS data model.

#![deny(missing_docs)]

extern crate wasm_bindgen;

use crate::datamodel::term::*;
use crate::datamodel::quad::*;
use crate::dataset::SophiaExportDataset;

use sophia::term::*;
use uuid::Uuid;
use wasm_bindgen::prelude::*;


// ============================================================================
//   ==== EXPORTATION ==== EXPORTATION ==== EXPORTATION ==== EXPORTATION ====

/// Wrapper struct for every data factory function
#[wasm_bindgen(js_name=DataFactory)]
pub struct SophiaExportDataFactory { }

#[wasm_bindgen(js_class=DataFactory)]
impl SophiaExportDataFactory {
    /// Builds an instance of a factory that builds RDF.JS compliant objects
    /// managed by Sophia 
    #[wasm_bindgen(constructor)]
    pub fn new() -> SophiaExportDataFactory {
        SophiaExportDataFactory{ }
    }

    /// Returns a named node that concerns the given URL
    #[wasm_bindgen(js_name="namedNode")]
    pub fn named_node(value: &str) -> SophiaExportTerm {
        SophiaExportTerm { term: Some(RcTerm::new_iri(value).unwrap()) }
    }

    /// Returns a blank node that either contains the given URL or a randomly
    /// generated ID using Uuid v4
    #[wasm_bindgen(js_name="blankNode")]
    pub fn blank_node(value: Option<String>) -> SophiaExportTerm {
        let blank_node_name = match value {
            Some(determined_name) => determined_name.to_string(),
            None => Uuid::new_v4().to_hyphenated().to_string()
        };

        SophiaExportTerm { term: Some(RcTerm::new_bnode(blank_node_name).unwrap()) }
    }

    /// Returns a new ltieral. If `language_or_data` is a Named Node, it will
    /// be used as the datatype. If it is a string formatted using the BCP 47
    /// spefification il will be used as the language. If undefined, it will be
    /// string.
    #[wasm_bindgen(js_name="literal")]
    pub fn literal(value_string: Option<String>, language_or_datatype: &JsValue) -> SophiaExportTerm {
        let value = match value_string.as_ref() {
            None => "",
            Some(contained_value) => contained_value.as_str()
        };

        if language_or_datatype.is_null() || language_or_datatype.is_undefined() {
            let string_term = RcTerm::new_iri("http://www.w3.org/2001/XMLSchema#string").unwrap();
            SophiaExportTerm { term: Some(RcTerm::new_literal_dt(value, string_term).unwrap()) }
        } else {
            match language_or_datatype.as_string() {
                Some(language) => Self::literal_from_string(value, language.as_str()),
                None => {
                    let language_or_datatype = language_or_datatype.clone();
                    Self::literal_from_named_node(value, &language_or_datatype.into())
                }
            }
        }
    }

    /// Builds a literal using the passed BCP 47 formatted language
    #[wasm_bindgen(js_name="literalFromString")]
    pub fn literal_from_string(value: &str, language: &str) -> SophiaExportTerm {
        SophiaExportTerm {
            term: Some(RcTerm::new_literal_lang(value, language).unwrap())
        }
    }

    /// Builds a literal of passed the datatype
    #[wasm_bindgen(js_name="literalFromNamedNode")]
    pub fn literal_from_named_node(value: &str, named_node: &JsImportTerm) -> SophiaExportTerm {
        let rcterm = build_rcterm_from_js_import_term(named_node);
        SophiaExportTerm { term: Some(RcTerm::new_literal_dt(value, rcterm.unwrap()).unwrap()) }
    }

    /// Builds a new variable
    #[wasm_bindgen(js_name="variable")]
    pub fn variable(optional_value: Option<String>) -> SophiaExportTerm {
        let value = match optional_value.as_ref() {
            None => "__NoVariableName__",
            Some(contained_value) => contained_value.as_str()
        };

        SophiaExportTerm { term: Some(RcTerm::new_variable(value).unwrap()) }
    }

    /// Returns a term that represents the default graph
    #[wasm_bindgen(js_name="defaultGraph")]
    pub fn default_graph() -> SophiaExportTerm {
        SophiaExportTerm { term: None }
    }

    /// Retuns a quad managed by Sophia's back end with the given subject,
    /// predicate, object and graph. If no graph is passed, the default graph
    /// will be used.
    #[wasm_bindgen(js_name="quad")]
    pub fn quad(subject: &JsImportTerm, predicate: &JsImportTerm, object: &JsImportTerm, graph: &JsImportTerm) -> SophiaExportQuad {
        SophiaExportQuad {
            _subject: build_rcterm_from_js_import_term(subject).unwrap(),
            _predicate: build_rcterm_from_js_import_term(predicate).unwrap(),
            _object: build_rcterm_from_js_import_term(object).unwrap(),
            _graph: if graph.is_null() || graph.is_undefined() {
                None
            } else {
                build_rcterm_from_js_import_term(graph)
            }
        }
    }

    /// Returns a quad managed by Sophia's back end with the given subject,
    /// predicate and object. The assigned graph will be the default graph.
    #[wasm_bindgen(js_name="triple")]
    pub fn triple(subject: &JsImportTerm, predicate: &JsImportTerm, object: &JsImportTerm) -> SophiaExportQuad {
        SophiaExportQuad {
            _subject: build_rcterm_from_js_import_term(subject).unwrap(),
            _predicate: build_rcterm_from_js_import_term(predicate).unwrap(),
            _object: build_rcterm_from_js_import_term(object).unwrap(),
            _graph: None
        }
    }

    /// Returns a new term managed by Sophia's back end that is identical wrt
    /// RDF.JS specification
    #[wasm_bindgen(js_name="fromTerm")]
    pub fn from_term(original: &JsImportTerm) -> SophiaExportTerm {
        if original.term_type().as_str() == "DefaultGraph" {
            SophiaExportDataFactory::default_graph()
        } else {
            SophiaExportTerm { term: build_rcterm_from_js_import_term(original) }
        }
    }

    /// Returns a new quad managed by Sophia's back end that is identical wrt
    /// RDF.JS specification
    #[wasm_bindgen(js_name="fromQuad")]
    pub fn from_quad(original: &JsImportQuad) -> SophiaExportQuad {
        Self::quad(
            &original.subject(),
            &original.predicate(),
            &original.object(),
            &original.graph()
        )
    }

    /// Returns a dataset managed by Sophia's back end. `parameter` can either
    /// be undefined, another dataset or a sequence of quads. If `parameter`
    /// isn't undefined, the returned dataset will contain every quad in the
    /// given argument.
    #[wasm_bindgen(js_name="dataset")]
    pub fn dataset(parameter: &JsValue) -> SophiaExportDataset {
        let mut ds = SophiaExportDataset::new();

        if !parameter.is_null() && !parameter.is_undefined() {
            // TODO : Error Management - Do we want to crash if the parameter is not an array ?
            // TODO : Manage every iterable (js_array::Array probably only manager javascript arrays)
            let parameter_copy: JsValue = parameter.clone();
            let js_array: js_sys::Array = parameter_copy.into();

            js_array.iter().for_each(|js_value| {
                // TODO : Error Management - What if the cast from JsValue to JsImportQuad fails ?
                let js_quad: JsImportQuad = js_value.into();
                ds.add(&js_quad);
            });
        }

        ds
    }
}
