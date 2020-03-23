//! This module contains every terms that can either be imported or exported to
//! the Javascript world.

#![deny(missing_docs)]

extern crate wasm_bindgen;

use sophia::term::*;
use sophia::term::Term::BNode;
use sophia::term::Term::Iri;
use sophia::term::Term::Literal;
use sophia::term::Term::Variable;
use wasm_bindgen::prelude::*;

// ============================================================================
//   ==== IMPORTATION ==== IMPORTATION ==== IMPORTATION ==== IMPORTATION ====

#[wasm_bindgen]
pub extern "C" {
    /// Importation of a Term from the Javascript world that follows the RDF.JS
    /// specification
    #[wasm_bindgen(js_name = Term)]
    pub type JsImportTerm;
    
    #[wasm_bindgen(method, getter = termType)]
    pub fn term_type(this: &JsImportTerm) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &JsImportTerm) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &JsImportTerm, value: String);

    #[wasm_bindgen(method, getter)]
    pub fn language(this: &JsImportTerm) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_language(this: &JsImportTerm, value: String);

    #[wasm_bindgen(method, getter)]
    pub fn datatype(this: &JsImportTerm) -> JsImportTerm;
    // Returning a copy of a JsImportTerm is acceptable by RDFJS standard

    #[wasm_bindgen(method, setter)]
    pub fn set_datatype(this: &JsImportTerm, named_node: &JsImportTerm);

    #[wasm_bindgen(js_name=equals)]
    pub fn terms_equals(this: &JsImportTerm, other_term: &JsImportTerm);
}

/// Builds a new RcTerm that has the representation used by Sophia for the give
/// JsImportTerm
pub fn build_rcterm_from_js_import_term(term: &JsImportTerm) -> Option<RcTerm> {
    match term.term_type().as_str() {
        "NamedNode" => Some(RcTerm::new_iri(term.value()).unwrap()),
        "BlankNode" => Some(RcTerm::new_bnode(term.value()).unwrap()),
        "Literal" => {
            let value = term.value();
            let language = term.language();

            let a = if language != "" { // Lang
                RcTerm::new_literal_lang(value, language)
            } else {
                let datatype = term.datatype();
                RcTerm::new_literal_dt(value, build_rcterm_from_js_import_term(&datatype).unwrap())
            };
            
            Some(a.unwrap())
        },
        "Variable" => Some(RcTerm::new_variable(term.value()).unwrap()),
        "DefaultGraph" => None,
        _ => None
    }
}

/// Builds a StringTerm from a JsImportTerm
/// 
/// This is the fastest way to build a Term from an object imported from Javascript
pub fn build_stringterm_from_js_import_term(term: &JsImportTerm) -> Option<Term<String>> {
    match term.term_type().as_str() {
        "NamedNode" => Some(Term::<String>::new_iri(term.value()).unwrap()),
        "BlankNode" => Some(Term::<String>::new_bnode(term.value()).unwrap()),
        "Literal" => {
            let value = term.value();
            let language = term.language();

            let literal_result = if language != "" { // Lang
                Term::<String>::new_literal_lang(value, language)
            } else {
                let datatype = term.datatype();
                Term::<String>::new_literal_dt(value, build_stringterm_from_js_import_term(&datatype).unwrap())
            };
            
            Some(literal_result.unwrap())
        },
        "Variable" => Some(Term::<String>::new_variable(term.value()).unwrap()),
        "DefaultGraph" => None,
        _ => None
    }
}


// ============================================================================
//   ==== EXPORTATION ==== EXPORTATION ==== EXPORTATION ==== EXPORTATION ====

/// Exportation of rust terms using an adapter that owns the term
#[wasm_bindgen(js_name="Term")]
pub struct SophiaExportTerm {
    /// The encapsulated Sophia Term. If `None`, this term describes the default graph.
    #[wasm_bindgen(skip)]
    pub term: Option<RcTerm>
}

impl SophiaExportTerm {
    /// Returns a RDF JS compliant term based on Sophia's RcTerm
    pub fn new(term: &RcTerm) -> SophiaExportTerm {
        SophiaExportTerm { term: Some(term.clone()) }
    }

    /// Returns a term that represents the default graph
    pub fn default_graph() -> SophiaExportTerm {
        SophiaExportTerm { term: None }
    }
}

// Implementation of the RDF JS specification
// https://rdf.js.org/data-model-spec/#term-interface
// Every term type is implemented as a SophiaExportTerm
#[wasm_bindgen(js_class="Term")]
impl SophiaExportTerm {
    /// Returns the term type of this term.
    /// 
    /// The returned value can either be NamedNode, BlankNode, Literal,
    /// Variable or DefaultGraph.
    #[wasm_bindgen(getter = termType)]
    pub fn term_type(&self) -> String {
        match &self.term {
            Some(Iri(_)) => "NamedNode".into(),
            Some(BNode(_)) => "BlankNode".into(),
            Some(Literal(_)) => "Literal".into(),
            Some(Variable(_)) => "Variable".into(),
            None => "DefaultGraph".into()
        }
    }

    /// Returns the value of this term
    #[wasm_bindgen(getter = value)]
    pub fn value(&self) -> String {
        match &self.term {
            Some(t) => t.value(),
            None => "".into()
        }
    }

    /// Modifies the value of this term
    #[wasm_bindgen(setter = value)]
    pub fn set_value(&mut self, new_value: &str) {
        match &self.term {
            None => { /* can't reassign a Default Graph */ },
            Some(real_term) => self.term = Some(match real_term {
                RcTerm::Iri(_) => RcTerm::new_iri(new_value).unwrap(),
                RcTerm::BNode(_) => RcTerm::new_bnode(new_value).unwrap(),
                RcTerm::Variable(_) => RcTerm::new_variable(new_value).unwrap(),
                RcTerm::Literal(former_literal) => {
                    match former_literal.lang() {
                        None => RcTerm::new_literal_dt(new_value, &former_literal.dt()).unwrap(),
                        Some(lang) => RcTerm::new_literal_lang_unchecked(new_value, lang.as_ref())
                    }
                }
            })
        }
    }

    /// Returns the language of this term or an empty string if not applicable
    #[wasm_bindgen(getter = language)]
    pub fn language(&self) -> String {
        match &self.term {
            Some(Literal(literal)) =>
                match literal.lang() {
                    Some(l) => l.to_string(),
                    None => String::from("")
                },
            _ => String::from("")
        }
    }

    /// Modifies the language of this term if applicable
    #[wasm_bindgen(method, setter)]
    pub fn set_language(&mut self, language: &str) {
        // In this implementation, if we set the language of a literal, it will be automatically
        // converted to the datatype langString regardless of its previous datatype.
        // Setting the language of any other term has no effect.
        if let Some(Literal(literal)) = &self.term {
            self.term = Some(RcTerm::new_literal_lang(literal.value(), language).unwrap());
        }
    }

    /// Returns the datatype of this term if applicable
    #[wasm_bindgen(getter)]
    pub fn datatype(&self) -> Option<SophiaExportTerm> {
        match &self.term {
            Some(Literal(literal)) =>
                // TODO : check if iri always has a type (especially for string)
                Option::Some(SophiaExportTerm {
                    term: Some(RcTerm::new_iri_unchecked(literal.dt().value(), true))
                }),
            _ => Option::None
        }
    }

    /// Modifies the dataset of this literal if applicable
    #[wasm_bindgen(method, setter)]
    pub fn set_datatype(&mut self, named_node: &JsImportTerm) {
        if let Some(Literal(literal)) = &self.term {
            let new_node_value = self.value();
            let literal_type: RcTerm = RcTerm::new_iri(named_node.value()).unwrap();
            self.term = Some(RcTerm::new_literal_dt(new_node_value, literal_type).unwrap());
        }
    }

    /// Returns true if this term and the given term are equals according to
    /// RDF.JS specification.
    /// 
    /// Two terms are identical if their termType, value and eventual language
    /// or datatype are the same.
    #[wasm_bindgen(js_name = equals)]
    pub fn equals(&self, other: Option<JsImportTerm>) -> bool {
        match other {
            None => false,
            Some(x) => {
                // We don't use the implementation of term_type / value to have better performances.
                let other_term_type = x.term_type();
                match &self.term {
                    Some(Iri(txt)) => other_term_type == "NamedNode" && x.value() == txt.value(),
                    Some(BNode(txt)) => other_term_type == "BlankNode" && x.value() == txt.value(),
                    Some(Variable(txt)) => other_term_type == "Variable" && x.value() == txt.value(),
                    Some(Literal(literal)) => 
                        other_term_type == "Literal" && x.value() == literal.value()
                            && literal.lang().map_or_else(
                                || x.language() == "",
                                |language| language.as_ref() == x.language().as_str()
                            )
                            && literal.dt().value() == x.datatype().value()
                        ,
                    None => other_term_type == "DefaultGraph" // value should be "" if it is RDFJS compliant
                }
            }
        }
    }

    /// Returns the n3 representation of this term
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        match &self.term {
            Some(t) => format!("{}", t),
            None => String::from("(DefaultGraph)")
        }
    }
}
