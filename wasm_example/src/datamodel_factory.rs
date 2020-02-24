//! Imports a static factory that is able to instanciate the different eleents
//! described by the RDF.JS data model.

extern crate wasm_bindgen;

use crate::datamodel_term::*;
use crate::datamodel_quad::*;
use crate::SophiaExportDataset;

use uuid::Uuid;
use sophia::term::*;
use wasm_bindgen::prelude::*;


// ============================================================================
//   ==== EXPORTATION ==== EXPORTATION ==== EXPORTATION ==== EXPORTATION ====

/// Wrapper struct for every data factory function
#[wasm_bindgen(js_name=DataFactory)]
pub struct SophiaExportDataFactory { }

#[wasm_bindgen(js_class=DataFactory)]
impl SophiaExportDataFactory {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SophiaExportDataFactory {
        SophiaExportDataFactory{ }
    }

    #[wasm_bindgen(js_name="namedNode")]
    pub fn named_node(value: &str) -> SophiaExportTerm {
        SophiaExportTerm { term: Some(RcTerm::new_iri(value).unwrap()) }
    }

    #[wasm_bindgen(js_name="blankNode")]
    pub fn blank_node(value: Option<String>) -> SophiaExportTerm {
        let blank_node_name = match value {
            Some(determined_name) => determined_name.to_string(),
            None => Uuid::new_v4().to_hyphenated().to_string()
        };

        SophiaExportTerm { term: Some(RcTerm::new_bnode(blank_node_name).unwrap()) }
    }

    #[wasm_bindgen(js_name="literal")]
    pub fn literal(value_string: Option<String>, language_or_datatype: JsValue) -> SophiaExportTerm {
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
                None => Self::literal_from_named_node(value, language_or_datatype.into())
            }
        }
    }

    #[wasm_bindgen(js_name="literalFromString")]
    pub fn literal_from_string(value: &str, language: &str) -> SophiaExportTerm {
        SophiaExportTerm {
            term: Some(RcTerm::new_literal_lang(value, language).unwrap())
        }
    }

    #[wasm_bindgen(js_name="literalFromNamedNode")]
    pub fn literal_from_named_node(value: &str, named_node: JsImportTerm) -> SophiaExportTerm {
        let rcterm = build_rcterm_from_js_import_term(&named_node);
        SophiaExportTerm { term: Some(RcTerm::new_literal_dt(value, rcterm.unwrap()).unwrap()) }
    }

    #[wasm_bindgen(js_name="variable")]
    pub fn variable(optional_value: Option<String>) -> SophiaExportTerm {
        let value = match optional_value.as_ref() {
            None => "__NoVariableName__",
            Some(contained_value) => contained_value.as_str()
        };

        SophiaExportTerm { term: Some(RcTerm::new_variable(value).unwrap()) }
    }

    #[wasm_bindgen(js_name="defaultGraph")]
    pub fn default_graph() -> SophiaExportTerm {
        SophiaExportTerm { term: None }
    }

    #[wasm_bindgen(js_name="quad")]
    pub fn quad(subject: JsImportTerm, predicate: JsImportTerm, object: JsImportTerm, graph: Option<JsImportTerm>) -> SophiaExportQuad {
        SophiaExportQuad {
            _subject: build_rcterm_from_js_import_term(&subject).unwrap(),
            _predicate: build_rcterm_from_js_import_term(&predicate).unwrap(),
            _object: build_rcterm_from_js_import_term(&object).unwrap(),
            _graph: match graph {
                None => None,
                Some(g) => build_rcterm_from_js_import_term(&g)
            }
        }
    }

    #[wasm_bindgen(js_name="triple")]
    pub fn triple(subject: JsImportTerm, predicate: JsImportTerm, object: JsImportTerm) -> SophiaExportQuad {
        SophiaExportQuad {
            _subject: build_rcterm_from_js_import_term(&subject).unwrap(),
            _predicate: build_rcterm_from_js_import_term(&predicate).unwrap(),
            _object: build_rcterm_from_js_import_term(&object).unwrap(),
            _graph: None
        }
    }

    #[wasm_bindgen(js_name="fromTerm")]
    pub fn from_term(original: JsImportTerm) -> SophiaExportTerm {
        if original.term_type().as_str() == "DefaultGraph" {
            SophiaExportDataFactory::default_graph()
        } else {
            SophiaExportTerm { term: build_rcterm_from_js_import_term(&original) }
        }
    }

    #[wasm_bindgen(js_name="fromQuad")]
    pub fn from_quad(original: JsImportQuad) -> SophiaExportQuad {
        Self::quad(
            original.subject(),
            original.predicate(),
            original.object(),
            Some(original.graph())
        )
    }

    #[wasm_bindgen(js_name="dataset")]
    pub fn dataset(parameter: JsValue) -> SophiaExportDataset {
        let mut ds = SophiaExportDataset::new();

        if !parameter.is_null() && !parameter.is_undefined() {
            // TODO : Error Management - Do we want to crash if the parameter is not an array ?
            // TODO : Manage every iterable (js_array::Array probably only manager javascript arrays)
            let js_array: js_sys::Array = parameter.into();

            js_array.iter().for_each(|js_value| {
                // TODO : Error Management - What if the cast from JsValue to JsImportQuad fails ?
                let js_quad: JsImportQuad = js_value.into();
                ds.add(js_quad);
            });
        }

        ds
    }
}
