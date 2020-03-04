//! This module contains every terms that can either be imported or exported to
//! the Javascript world.

#![deny(missing_docs)]

extern crate wasm_bindgen;

use sophia::term::*;
use std::rc::Rc;
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
    let determine = |result_term: Result<RcTerm>| Some(result_term.unwrap());
    // TODO : check if defining build_literal here can cause performances issues
    let build_literal = |term: &JsImportTerm| {
        let value = term.value();
        let language = term.language();
        if language != "" { // Lang
            RcTerm::new_literal_lang(value, language)
        } else {
            let datatype = term.datatype();
            RcTerm::new_literal_dt(value, build_rcterm_from_js_import_term(&datatype).unwrap())
        }
    };

    match term.term_type().as_str() {
        "NamedNode" => determine(RcTerm::new_iri(term.value())),
        "BlankNode" => determine(RcTerm::new_bnode(term.value())),
        "Literal" => determine(build_literal(term)),
        "Variable" => determine(RcTerm::new_variable(term.value())),
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
    // TODO : Wouldn't it be better to make a proper enum { OnwedRcTerm(A), DefaultGraph } ?
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
            Some(Literal(_1, _2)) => "Literal".into(),
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
                RcTerm::Literal(_, Lang(lang)) => RcTerm::new_literal_lang(new_value, lang.clone()).unwrap(),
                RcTerm::Literal(_, Datatype(dt)) =>
                        RcTerm::new_literal_dt(new_value, RcTerm::new_iri(dt.to_string()).unwrap()).unwrap()
                })
        }
    }

    /// Returns the language of this term or an empty string if not applicable
    #[wasm_bindgen(getter = language)]
    pub fn language(&self) -> String {
        match &self.term {
            Some(Literal(_, Lang(language))) => language.to_string(),
            _ => String::from("")
        }
    }

    /// Modifies the language of this term if applicable
    #[wasm_bindgen(method, setter)]
    pub fn set_language(&mut self, language: &str) {
        // In this implementation, if we set the language of a literal, it will be automatically
        // converted to the datatype langString regardless of its previous datatype.
        // Setting the language of any other term has no effect.
        if let Some(Literal(old_value, _)) = &self.term {
            self.term = Some(RcTerm::new_literal_lang(old_value.to_string(), language).unwrap());
        }
    }

    /// Returns the datatype of this term if applicable
    #[wasm_bindgen(getter)]
    pub fn datatype(&self) -> Option<SophiaExportTerm> {
        match &self.term {
            Some(Literal(_1, Lang(_2))) =>
                Option::Some(SophiaExportTerm {
                    term: Some(RcTerm::new_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString").unwrap())
                }),
            // TODO : check if iri always has a type (especially for string)
            Some(Literal(_1, Datatype(iri))) =>
                Option::Some(SophiaExportTerm {
                    term: Some(RcTerm::new_iri(iri.to_string()).unwrap())
                }),
            _ => Option::None
        }
    }

    /// Modifies the dataset of this literal if applicable
    #[wasm_bindgen(method, setter)]
    pub fn set_datatype(&mut self, named_node: &JsImportTerm) {
        if let Some(Literal(_, literal_kind)) = &self.term {
            if let Lang(_) = literal_kind {
                if named_node.value() == "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString" {
                    // Do not change datatype to langString of literals that are already langString
                    return
                }
            }
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
                    Some(Iri(txt)) => other_term_type == "NamedNode" && x.value() == txt.to_string(),
                    Some(BNode(txt)) => other_term_type == "BlankNode" && x.value() == txt.value(),
                    Some(Variable(txt)) => other_term_type == "Variable" && x.value() == txt.value(),
                    Some(Literal(txt, literal_kind)) => 
                        other_term_type == "Literal" && x.value() == txt.to_string()
                            && SophiaExportTerm::equals_to_literal(literal_kind, &x),
                    None => other_term_type == "DefaultGraph" // value should be "" if it is RDFJS compliant
                }
            }
        }
    }

    fn equals_to_literal(literal_kind: &LiteralKind<Rc<str>>, other : &JsImportTerm) -> bool {
        // The standard ensures us that other has the language and datatype attributes
        
        // Documentation questionning :
        // Otherwise, if no datatype is explicitly specified, the datatype has the IRI
        // "http://www.w3.org/2001/XMLSchema#string". -> ????
        match literal_kind {
            Lang(language) => language.to_string() == other.language()
                && "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString" == other.datatype().value(),
            Datatype(iri) => other.language() == "" && other.datatype().value() == iri.to_string()
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
