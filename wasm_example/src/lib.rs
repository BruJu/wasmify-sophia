#![allow(unused_imports)]
extern crate wasm_bindgen;

// TODO : remove unused imports when finished

use std::any;
use std::rc::Rc;
use std::iter;
use uuid::Uuid;
use sophia::dataset::Dataset;
use sophia::dataset::inmem::FastDataset;
use sophia::quad::Quad;
use sophia::quad::stream::QuadSource;
use sophia::graph::inmem::LightGraph;
use sophia::parser::trig;
use sophia::term::*;
use sophia::triple::stream::TripleSource;
use sophia::dataset::MutableDataset;
use wasm_bindgen::prelude::*;

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


#[wasm_bindgen(js_name="DatasetCore")]
pub struct SophiaExportDataset {
    dataset: FastDataset
}

impl SophiaExportDataset {
    pub fn convert_to_sophia_export_quad(&self, js_quad: JsImportQuad) -> SophiaExportQuad {
        SophiaExportDataFactory::from_quad(js_quad)
    }
}

/* DatasetIterator are implemented as earlier versions of NodeJs do not support js_sys::Array::values() */

// The iterator we provide is an iterator on the elements that are contained when we create the iterator
// New and deleted elements in the dataset do not change the state of the iterator

#[wasm_bindgen(js_name="DatasetCoreIterator")]
pub struct SophiaExportDatasetIterator {
    quads_array: js_sys::Array
}

impl SophiaExportDatasetIterator {
    pub fn new(quads_array: js_sys::Array) -> SophiaExportDatasetIterator {
        // We reverse in place so we can think our iterator as a list of quads we have not iterated on yet
        quads_array.reverse();
        SophiaExportDatasetIterator { quads_array }
    }
}

#[wasm_bindgen(js_class="DatasetCoreIterator")]
impl SophiaExportDatasetIterator {
    #[wasm_bindgen]
    pub fn next(&mut self) -> SophiaExportDatasetIteratorNext {
        if self.quads_array.length() != 0 {
            SophiaExportDatasetIteratorNext{ current_element: Some(self.quads_array.pop()) }
        } else {
            SophiaExportDatasetIteratorNext{ current_element: None }
        }
    }
}

#[wasm_bindgen(js_name="DatasetCoreIteratorNext")]
pub struct SophiaExportDatasetIteratorNext {
    #[wasm_bindgen(skip)]
    pub current_element: Option<JsValue>
}

#[wasm_bindgen(js_class="DatasetCoreIteratorNext")]
impl SophiaExportDatasetIteratorNext {
    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.current_element.is_none()
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        match self.current_element.as_ref() {
            None => JsValue::undefined(),
            Some(real_value) => real_value.clone()
        }
    }
}


/* Dataset export */

#[wasm_bindgen(js_class="DatasetCore")]
impl SophiaExportDataset {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SophiaExportDataset {
        SophiaExportDataset{ dataset: FastDataset::new() }
    }

    #[wasm_bindgen(js_name="getIterator")]
    pub fn get_iterator(&self) -> SophiaExportDatasetIterator {
        // TODO : bind this function call to this[Symbol.iterator]
        // a.values() is not supported by every version of nodejs so we are forced to design our own iterator
        SophiaExportDatasetIterator::new(self.quads())
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
        let graphs = self.dataset.graph_names().unwrap();

        let mut all_terms = subjects;
        all_terms.extend(predicates);
        all_terms.extend(objects);
        all_terms.extend(graphs);

        all_terms.into_iter()
                 .map(|term| SophiaExportTerm::new(&term.clone()))
                 .map(JsValue::from)
                 .collect()
    }

    #[wasm_bindgen(js_name="add")]
    pub fn add(&mut self, quad: JsImportQuad) {
        // As we use RcTerms, copy should be cheap and simple enough to not
        // have too much performances issues
        let sophia_quad = self.convert_to_sophia_export_quad(quad);
        self.dataset.insert(
            &sophia_quad._subject,
            &sophia_quad._predicate,
            &sophia_quad._object,
            match &sophia_quad._graph {
                None => None,
                Some(x) => Some(x)
            }
        ).unwrap();

        // TODO : return this
    }

    #[wasm_bindgen(js_name="delete")]
    pub fn delete(&mut self, quad: JsImportQuad) {
        // Fastdataset implements the trait SetDataset so this function removes
        // every occurrences of the passed quad
        let sophia_quad = self.convert_to_sophia_export_quad(quad);
        self.dataset.remove(
            &sophia_quad._subject,
            &sophia_quad._predicate,
            &sophia_quad._object,
            match &sophia_quad._graph {
                None => None,
                Some(x) => Some(x)
            }
        ).unwrap();

        // TODO : return this
    }
    
    #[wasm_bindgen(js_name="has")]
    pub fn has_quad(&self, quad: JsImportQuad) -> bool {
        let sophia_quad = self.convert_to_sophia_export_quad(quad);
        self.dataset.contains(
            &sophia_quad._subject,
            &sophia_quad._predicate,
            &sophia_quad._object,
            match &sophia_quad._graph {
                None => None,
                Some(x) => Some(x)
            }
        ).unwrap()
    }

    #[wasm_bindgen(js_name="match")]
    pub fn match_quad(&self, subject: Option<JsImportTerm>, predicate: Option<JsImportTerm>,
        object: Option<JsImportTerm>, graph: Option<JsImportTerm>) -> SophiaExportDataset {
        let mut dataset = FastDataset::new();

        let subject = subject.as_ref().map(|x| build_rcterm_from_js_import_term(x).unwrap());
        let predicate = predicate.as_ref().map(|x| build_rcterm_from_js_import_term(x).unwrap());
        let object = object.as_ref().map(|x| build_rcterm_from_js_import_term(x).unwrap());
        let graph = graph.as_ref().map(|x| build_rcterm_from_js_import_term(x));

        let mut quads_iter = match (&subject, &predicate, &object, graph.as_ref()) {
            (None   , None   , None   , None   ) => self.dataset.quads(),
            (None   , None   , Some(o), None   ) => self.dataset.quads_with_o(o),
            (None   , Some(p), None   , None   ) => self.dataset.quads_with_p(p),
            (None   , Some(p), Some(o), None   ) => self.dataset.quads_with_po(p, o),
            (Some(s), None   , None   , None   ) => self.dataset.quads_with_s(s),
            (Some(s), None   , Some(o), None   ) => self.dataset.quads_with_so(s, o),
            (Some(s), Some(p), None   , None   ) => self.dataset.quads_with_sp(s, p),
            (Some(s), Some(p), Some(o), None   ) => self.dataset.quads_with_spo(s, p, o),
            (None   , None   , None   , Some(g)) => self.dataset.quads_with_g(g.as_ref()),
            (None   , None   , Some(o), Some(g)) => self.dataset.quads_with_og(o, g.as_ref()),
            (None   , Some(p), None   , Some(g)) => self.dataset.quads_with_pg(p, g.as_ref()),
            (None   , Some(p), Some(o), Some(g)) => self.dataset.quads_with_pog(p, o, g.as_ref()),
            (Some(s), None   , None   , Some(g)) => self.dataset.quads_with_sg(s, g.as_ref()),
            (Some(s), None   , Some(o), Some(g)) => self.dataset.quads_with_sog(s, o, g.as_ref()),
            (Some(s), Some(p), None   , Some(g)) => self.dataset.quads_with_spg(s, p, g.as_ref()),
            (Some(s), Some(p), Some(o), Some(g)) => self.dataset.quads_with_spog(s, p, o, g.as_ref())
        };

        quads_iter.in_dataset(&mut dataset).unwrap();

        SophiaExportDataset{ dataset: dataset }
    }

    #[wasm_bindgen(getter = size)]
    pub fn get_size(&self) -> usize {
        self.dataset.quads().into_iter().count()
    }
}


// ============================================================================
// ==== RDF JS Term

// Importation of Javascript Term (JsImportTerm)
#[wasm_bindgen]
extern "C" {
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

fn build_rcterm_from_js_import_term(term: &JsImportTerm) -> Option<RcTerm> {
    let determine = |result_term : Result<RcTerm>| Some(result_term.unwrap());
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



// Exportation of rust terms using an adapter that owns the term

#[wasm_bindgen(js_name="Term")]
pub struct SophiaExportTerm {
    /// The encapsulated Sophia Term. If None, this term describes the default graph.
    term: Option<RcTerm>
}

// The new function is not exported into the Javascript interface
impl SophiaExportTerm {   
    // Returns a RDF JS compliant term based on Sophia's RcTerm
    pub fn new(term: &RcTerm) -> SophiaExportTerm {
        SophiaExportTerm { term: Some(term.clone()) }
    }

    // Returns a term that represents the default graph
    pub fn default_graph() -> SophiaExportTerm {
        SophiaExportTerm { term: None }
    }
}

// Implementation of the RDF JS specification
// https://rdf.js.org/data-model-spec/#term-interface
// Every term type is implemented as a SophiaExportTerm
#[wasm_bindgen(js_class="Term")]
impl SophiaExportTerm {
    #[wasm_bindgen]
    pub fn is_connected_to_rust(&self) -> bool {
        true
    }

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

    #[wasm_bindgen(getter = value)]
    pub fn value(&self) -> String {
        match &self.term {
            Some(t) => t.value(),
            None => "".into()
        }
    }

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

    #[wasm_bindgen(getter = language)]
    pub fn language(&self) -> String {
        match &self.term {
            Some(Literal(_, Lang(language))) => language.to_string(),
            _ => String::from("")
        }
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_language(&mut self, language: &str) {
        // In this implementation, if we set the language of a literal, it will be automatically
        // converted to the datatype langString regardless of its previous datatype.
        // Setting the language of any other term has no effect.
        if let Some(Literal(old_value, _)) = &self.term {
            let language: Rc<str> = language.into();
            self.term = Some(RcTerm::new_literal_lang(old_value.to_string(), language).unwrap());
        }
    }

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
            let literal_type: RcTerm = RcTerm::new_iri(named_node.value().as_str()).unwrap();
            self.term = Some(RcTerm::new_literal_dt(new_node_value, literal_type).unwrap());
        }
    }

    #[wasm_bindgen(js_name = equals)]
    pub fn equals(&self, other: Option<JsImportTerm>) -> bool {
        match other {
            None => false,
            Some(x) => {
                // We don't use the implementation of term_type / value to have better performances.
                let other_term_type = x.term_type();
                match &self.term {
                    Some(Iri(txt)) => other_term_type == "NamedNode" && x.value() == txt.to_string(),
                    Some(BNode(txt)) => other_term_type == "BlankNode" && x.value() == txt.to_string(),
                    Some(Variable(txt)) => other_term_type == "Variable" && x.value() == txt.to_string(),
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

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        match &self.term {
            Some(t) => t.n3(),
            None => "(DefaultGraph)".into()
        }
    }
}

// ============================================================================
// ==== RDF JS Quad

// Importation of Javascript Quad (JsImportQuad)

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
    
}


// Exportation of Sophia Quads


#[wasm_bindgen(js_class="DatasetCore")]
impl SophiaExportDataset {
    pub fn quads(&self) -> js_sys::Array {
        self.dataset
            .quads()
            .into_iter()
            .map(|quad| {
                let quad = quad.unwrap();
                SophiaExportQuad::new(quad.s(), quad.p(), quad.o(), quad.g())
            })
            .map(JsValue::from)
            .collect()
    }
}

// A SophiaExportQuad owns its data. I did not find an already existing of this kind of quads.

#[wasm_bindgen(js_name = Quad)]
pub struct SophiaExportQuad {
    _subject: RcTerm,
    _predicate: RcTerm,
    _object: RcTerm,
    _graph: Option<RcTerm>
}

impl sophia::quad::Quad for SophiaExportQuad {
    type TermData = Rc<str>;

    fn s(&self) -> &RcTerm { &self._subject }
    fn p(&self) -> &RcTerm { &self._predicate }
    fn o(&self) -> &RcTerm { &self._object }
    fn g(&self) -> Option<&RcTerm> { self._graph.as_ref() }
}

impl SophiaExportQuad {
    pub fn new(s: &RcTerm, p: &RcTerm, o: &RcTerm, g: Option<&RcTerm>) -> SophiaExportQuad {
        SophiaExportQuad {
            _subject: s.clone(),
            _predicate: p.clone(),
            _object: o.clone(),
            _graph: g.cloned()
        }
    }

    pub fn new_by_move(s: RcTerm, p: RcTerm, o: RcTerm, g: Option<RcTerm>) -> SophiaExportQuad {
        SophiaExportQuad {
            _subject: s,
            _predicate: p,
            _object: o,
            _graph: g
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
            Some(other_quad) =>
                // TODO : Make a SophiaExportTerm that don't clone the items to reuse code without destroying performances
                // or use a cache mechanic (but it is worst)
                self.subject().equals(Some(other_quad.subject()))
                && self.predicate().equals(Some(other_quad.predicate()))
                && self.object().equals(Some(other_quad.object()))
                && self.graph().equals(Some(other_quad.graph()))
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
}


// ============================================================================
// ==== RDF JS DataFactory

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
        SophiaExportQuad::new_by_move(
            build_rcterm_from_js_import_term(&subject).unwrap(),
            build_rcterm_from_js_import_term(&predicate).unwrap(),
            build_rcterm_from_js_import_term(&object).unwrap(),
            match graph {
                None => None,
                Some(g) => build_rcterm_from_js_import_term(&g)
            }
        )
    }

    #[wasm_bindgen(js_name="triple")]
    pub fn triple(subject: JsImportTerm, predicate: JsImportTerm, object: JsImportTerm) -> SophiaExportQuad {
        SophiaExportQuad::new_by_move(
            build_rcterm_from_js_import_term(&subject).unwrap(),
            build_rcterm_from_js_import_term(&predicate).unwrap(),
            build_rcterm_from_js_import_term(&object).unwrap(),
            None
        )
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

