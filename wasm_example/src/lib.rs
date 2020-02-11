#![allow(unused_imports)]
extern crate wasm_bindgen;

// TODO : remove unused imports when finished

use sophia::quad::Quad;
use std::any;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use sophia::dataset::Dataset;
use sophia::dataset::inmem::FastDataset;
use sophia::quad::stream::QuadSource;
use sophia::graph::inmem::LightGraph;
use sophia::parser::trig;
use sophia::term::*;
use sophia::triple::stream::TripleSource;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
      // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_i32(a: i32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[allow(dead_code)]
fn print_type<T>(_: T) {
    log(any::type_name::<T>())
}


///////////////////////////////
/// A sample JSTerm to be used in the RDF/JS interface
/// David Pojunas
/// Pierre-Antoine Champin
///////////////////////////////

#[wasm_bindgen]
pub struct JSDataset {
    dataset: FastDataset
}

#[wasm_bindgen]
impl JSDataset{
    #[wasm_bindgen(constructor)]
    pub fn new() -> JSDataset {
        JSDataset{ dataset: FastDataset::new() }
    }

    pub fn load(&mut self, content: &str) {
        let r = sophia::parser::trig::parse_str(&content).in_dataset(&mut self.dataset);
        match r {
            Ok(_) => {},
            Err(error) => log(error.to_string().as_str())
        }
    }
    
    #[wasm_bindgen(js_name="getTerms")]
    pub fn get_terms(&self) -> js_sys::Array {
        let subjects = self.dataset.subjects().unwrap();
        let predicates = self.dataset.predicates().unwrap();
        let objects = self.dataset.objects().unwrap();

        let mut all_terms = subjects;
        all_terms.extend(predicates);
        all_terms.extend(objects);

        all_terms.into_iter()
                 .map(|term| BJTerm::new(&term.clone()))
                 .map(JsValue::from)
                 .collect()
    }
}

// ============================================================================
// ==== RDF JS Term

// Importation of Javascript Term (JssTerm)
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Term)]
    pub type JssTerm;

    #[wasm_bindgen(method, getter = termType)]
    pub fn term_type(this: &JssTerm) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &JssTerm) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &JssTerm, value: String);

    #[wasm_bindgen(js_name=equals)]
    pub fn equals(this: &JssTerm, other_term: &JssTerm);

    #[wasm_bindgen(method, getter)]
    pub fn language(this: &JssTerm) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_language(this: &JssTerm, value: String);

    #[wasm_bindgen(method, getter)]
    pub fn datatype(this: &JssTerm) -> JssTerm;
    // Returning a copy of a Jssterm is acceptable by RDFJS standard

    #[wasm_bindgen(method, setter)]
    pub fn set_datatype(this: &JssTerm, named_node: &JssTerm);
}

// Exportation of rust terms using an adapter that owns the term

#[wasm_bindgen(js_name="Term")]
pub struct BJTerm {
    term: RcTerm
}

// The new function is not exported into the Javascript interface
impl BJTerm {   
    pub fn new(term: &RcTerm) -> BJTerm {
        BJTerm { term: term.clone() }
    }
}

// Implementation of the RDF JS specification
// https://rdf.js.org/data-model-spec/#term-interface
// Every term type is implemented as a BJTerm except DefaultGraph
#[wasm_bindgen(js_class="Term")]
impl BJTerm {
    #[wasm_bindgen]
    pub fn is_connected_to_rust(&self) -> bool {
        true
    }

    #[wasm_bindgen(getter = termType)]
    pub fn term_type(&self) -> String {
        match &self.term {
            Iri(_) => "NamedNode".into(),
            BNode(_) => "BlankNode".into(),
            Literal(_1, _2) => "Literal".into(),
            Variable(_) => "Variable".into()
        }
    }

    #[wasm_bindgen(getter = value)]
    pub fn value(&self) -> String {
        self.term.value()
    }

    #[wasm_bindgen(setter = value)]
    pub fn set_value(&mut self, new_value: String) {
        self.term = match &self.term {
            RcTerm::Iri(_) => RcTerm::new_iri(new_value).unwrap(),
            RcTerm::BNode(_) => RcTerm::new_bnode(new_value).unwrap(),
            RcTerm::Variable(_) => RcTerm::new_variable(new_value).unwrap(),
            RcTerm::Literal(_, Lang(lang)) => RcTerm::new_literal_lang(new_value, lang.clone()).unwrap(),
            RcTerm::Literal(_, Datatype(dt))
                => {
                    RcTerm::new_literal_dt(new_value, RcTerm::new_iri(dt.to_string()).unwrap()).unwrap()
                }
        }
    }

    #[wasm_bindgen(getter = language)]
    pub fn language(&self) -> String {
        match &self.term {
            Literal(_, Lang(language)) => language.to_string(),
            _ => String::from("")
        }
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_language(&mut self, language: String) {
        // In this implementation, if we set the language of a literal, it will be automatically
        // converted to the datatype langString regardless of its previous datatype.
        // Setting the language of any other term has no effect.
        if let Literal(old_value, _) = &self.term {
            let language: Rc<str> = language.as_str().into();
            self.term = RcTerm::new_literal_lang(old_value.to_string(), language).unwrap();
        }
    }

    #[wasm_bindgen(getter)]
    pub fn datatype(&self) -> Option<BJTerm> {
        match &self.term {
            Literal(_1, Lang(_2)) =>
                Option::Some(BJTerm {
                    term: RcTerm::new_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString").unwrap()
                }),
            // TODO : check if iri always has a type (especially for string)
            Literal(_1, Datatype(iri)) =>
                Option::Some(BJTerm {
                    term: RcTerm::new_iri(iri.to_string()).unwrap()
                }),
            _ => Option::None
        }
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_datatype(&mut self, named_node: &JssTerm) {
        if let Literal(_, literal_kind) = &self.term {
            if let Lang(_) = literal_kind {
                if named_node.value() == "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString" {
                    // Do not change datatype to langString of literals that are already langString
                    return
                }
            }
            let new_node_value = self.value();
            let literal_type: RcTerm = RcTerm::new_iri(named_node.value().as_str()).unwrap();
            self.term = RcTerm::new_literal_dt(new_node_value, literal_type).unwrap();
        }
    }

    #[wasm_bindgen(js_name = equals)]
    pub fn equals(&self, other: Option<JssTerm>) -> bool {
        match other {
            None => false,
            Some(x) => {
                // We don't use the implementation of term_type / value to have better performances.
                let other_term_type = x.term_type();
                match &self.term {
                    Iri(_) => other_term_type == "NamedNode" && x.value() == self.term.value(),
                    BNode(_) => other_term_type == "BlankNode" && x.value() == self.term.value(),
                    Variable(_) => other_term_type == "Variable" && x.value() == self.term.value(),
                    Literal(_1, literal_kind) => 
                        other_term_type == "Literal" && x.value() == self.term.value()
                            && BJTerm::equals_to_literal(literal_kind, &x)
                }
            }
        }
    }

    fn equals_to_literal(literal_kind: &LiteralKind<Rc<str>>, other : &JssTerm) -> bool {
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

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.term.n3()
    }
}

// Default graph implementation
#[wasm_bindgen(js_name="DefaultGraph")]
pub struct BJDefaultGraph {}

#[wasm_bindgen(js_class="DefaultGraph")]
impl BJDefaultGraph {
    #[wasm_bindgen(getter = termType)]
    pub fn term_type(&self) -> String {
        String::from("DefaultGraph")
    }

    #[wasm_bindgen(getter = value)]
    pub fn value(&self) -> String {
        String::from("")
    }

    #[wasm_bindgen()]
    pub fn equals(&self, other: Option<JssTerm>) -> bool {
        match other {
            None => false,
            Some(term) => term.term_type() == "DefaultGraph"
            // We don't check value as all RDFJS compliant terms have an empty value
        }
    }
}

// ============================================================================
// ==== RDF JS Quad

#[wasm_bindgen]
impl JSDataset {
    pub fn quads(&self) -> js_sys::Array {
        self.dataset
            .quads()
            .into_iter()
            .map(|quad| {
                let quad = quad.unwrap();
                BJQuad::new(quad.s(), quad.p(), quad.o(), quad.g())
            })
            .map(JsValue::from)
            .collect()
    }
}

// A BJQuad owns its data. I did not find an already existing of this kind of quads.

#[wasm_bindgen(js_name="Quad")]
pub struct BJQuad {
    _subject: RcTerm,
    _predicate: RcTerm,
    _object: RcTerm,
    _graph: Option<RcTerm>
}

impl sophia::quad::Quad for BJQuad {
    type TermData = Rc<str>;

    fn s(&self) -> &RcTerm { &self._subject }
    fn p(&self) -> &RcTerm { &self._predicate }
    fn o(&self) -> &RcTerm { &self._object }
    fn g(&self) -> Option<&RcTerm> {
        match &self._graph {
            None => None,
            Some(some_graph) => Some(some_graph)
        }
    }
}

impl BJQuad {
    pub fn new(s: &RcTerm, p: &RcTerm, o: &RcTerm, g: Option<&RcTerm>) -> BJQuad {
        BJQuad {
            _subject: s.clone(),
            _predicate: p.clone(),
            _object: o.clone(),
            _graph: match g {
                None => None,
                Some(iri) => Some(iri.clone())
            }
        }
    }
}

#[wasm_bindgen(js_class="Quad")]
impl BJQuad {
    pub fn is_connected_to_rust(&self) -> bool {
        true
    }

    #[wasm_bindgen]
    pub fn subject(&self) -> BJTerm {
        BJTerm::new(&self._subject)
    }

    #[wasm_bindgen]
    pub fn predicate(&self) -> BJTerm {
        BJTerm::new(&self._predicate)
    }

    #[wasm_bindgen]
    pub fn object(&self) -> BJTerm {
        BJTerm::new(&self._object)
    }

    /*
    #[wasm_bindgen]
    pub fn graph(&self) -> BJDefaultGraph { // oops
        match self.graph() {
            None => BJDefaultGraph{},
            Some(iri) => BJTerm::new(iri)
        }
    }
    */

    // TODO LIST :
    // get subject
    // set subject
    // get predicate
    // set predicate
    // get object
    // set object
    // set graph
    // get graph
    // equals
    // toString()
}












