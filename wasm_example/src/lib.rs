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
use sophia::dataset::MutableDataset;

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
    dataset: FastDataset,
    datafactory: BJDataFactory
}

impl JSDataset {
    pub fn convert_to_bj_quad(&self, js_quad: JsImpQuad) -> BJQuad {
        self.datafactory.from_quad(js_quad)
    }
}

#[wasm_bindgen]
impl JSDataset{
    #[wasm_bindgen(constructor)]
    pub fn new() -> JSDataset {
        JSDataset{
            dataset: FastDataset::new(),
            datafactory: BJDataFactory::new()
        }
    }

    #[wasm_bindgen(js_name="cloneFactory")]
    pub fn clone_factory(&self) -> BJDataFactory {
        self.datafactory.clone()
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
                 .map(|term| BJTerm::new(&term.clone()))
                 .map(JsValue::from)
                 .collect()
    }

    #[wasm_bindgen(js_name="add")]
    pub fn add(&mut self, quad: JsImpQuad) {
        // As we use RcTerms, copy should be cheap and simple enough to not
        // have too much performances issues
        let sophia_quad = self.convert_to_bj_quad(quad);
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
    pub fn delete(&mut self, quad: JsImpQuad) {
        // Fastdataset implements the trait SetDataset so this function removes
        // every occurrences of the passed quad
        let sophia_quad = self.convert_to_bj_quad(quad);
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
    pub fn has_quad(&self, quad: JsImpQuad) -> bool {
        let sophia_quad = self.convert_to_bj_quad(quad);
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
    pub fn match_quad(&self, subject: Option<JssTerm>, predicate: Option<JssTerm>,
        object: Option<JssTerm>, graph: Option<JssTerm>) -> JSDataset {
            let mut dataset = FastDataset::new();

            let subject = subject.as_ref().map(|x| { build_rcterm_from_jss_term(x).unwrap() });
            let predicate = predicate.as_ref().map(|x| { build_rcterm_from_jss_term(x).unwrap() });
            let object = object.as_ref().map(|x| { build_rcterm_from_jss_term(x).unwrap() });
            let graph = graph.as_ref().map(|x| { build_rcterm_from_jss_term(x) });

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

            JSDataset{
                dataset: dataset,
                datafactory: BJDataFactory::new()
            }
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

    #[wasm_bindgen(method, getter)]
    pub fn language(this: &JssTerm) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_language(this: &JssTerm, value: String);

    #[wasm_bindgen(method, getter)]
    pub fn datatype(this: &JssTerm) -> JssTerm;
    // Returning a copy of a JssTerm is acceptable by RDFJS standard

    #[wasm_bindgen(method, setter)]
    pub fn set_datatype(this: &JssTerm, named_node: &JssTerm);

    #[wasm_bindgen(js_name=equals)]
    pub fn terms_equals(this: &JssTerm, other_term: &JssTerm);
}

fn build_rcterm_from_jss_term(term: &JssTerm) -> Option<RcTerm> {
    let determine = |result_term : Result<RcTerm>| Some(result_term.unwrap());
    // TODO : check if defining build_literal here can cause performances issues
    let build_literal = |term: &JssTerm| {
        let value = term.value();
        let language = term.language();
        if language != "" { // Lang
            RcTerm::new_literal_lang(value, language)
        } else {
            let datatype = term.datatype();
            RcTerm::new_literal_dt(value, build_rcterm_from_jss_term(&datatype).unwrap())
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
pub struct BJTerm {
    term: Option<RcTerm>
}

// The new function is not exported into the Javascript interface
impl BJTerm {   
    // Returns a RDF JS compliant term based on Sophia's RcTerm
    pub fn new(term: &RcTerm) -> BJTerm {
        BJTerm { term: Some(term.clone()) }
    }

    // Returns a term that represents the default graph
    pub fn default_graph() -> BJTerm {
        BJTerm { term: None }
    }
}

// Implementation of the RDF JS specification
// https://rdf.js.org/data-model-spec/#term-interface
// Every term type is implemented as a BJTerm
#[wasm_bindgen(js_class="Term")]
impl BJTerm {
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
    pub fn set_value(&mut self, new_value: String) {
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
    pub fn set_language(&mut self, language: String) {
        // In this implementation, if we set the language of a literal, it will be automatically
        // converted to the datatype langString regardless of its previous datatype.
        // Setting the language of any other term has no effect.
        if let Some(Literal(old_value, _)) = &self.term {
            let language: Rc<str> = language.as_str().into();
            self.term = Some(RcTerm::new_literal_lang(old_value.to_string(), language).unwrap());
        }
    }

    #[wasm_bindgen(getter)]
    pub fn datatype(&self) -> Option<BJTerm> {
        match &self.term {
            Some(Literal(_1, Lang(_2))) =>
                Option::Some(BJTerm {
                    term: Some(RcTerm::new_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString").unwrap())
                }),
            // TODO : check if iri always has a type (especially for string)
            Some(Literal(_1, Datatype(iri))) =>
                Option::Some(BJTerm {
                    term: Some(RcTerm::new_iri(iri.to_string()).unwrap())
                }),
            _ => Option::None
        }
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_datatype(&mut self, named_node: &JssTerm) {
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
    pub fn equals(&self, other: Option<JssTerm>) -> bool {
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
                            && BJTerm::equals_to_literal(literal_kind, &x),
                    None => other_term_type == "DefaultGraph" // value should be "" if it is RDFJS compliant
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
        match &self.term {
            Some(t) => t.n3(),
            None => "(DefaultGraph)".into()
        }
    }
}

// ============================================================================
// ==== RDF JS Quad

// Importation of Javascript Quad (JsImpQuad)

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Quad)]
    pub type JsImpQuad;
    
    #[wasm_bindgen(method, getter)]
    pub fn subject(this: &JsImpQuad) -> JssTerm;

    #[wasm_bindgen(method, setter)]
    pub fn set_subject(this: &JsImpQuad, value: &JssTerm);

    #[wasm_bindgen(method, getter)]
    pub fn object(this: &JsImpQuad) -> JssTerm;

    #[wasm_bindgen(method, setter)]
    pub fn set_object(this: &JsImpQuad, value: &JssTerm);

    #[wasm_bindgen(method, getter)]
    pub fn predicate(this: &JsImpQuad) -> JssTerm;

    #[wasm_bindgen(method, setter)]
    pub fn set_predicate(this: &JsImpQuad, value: &JssTerm);

    #[wasm_bindgen(method, getter)]
    pub fn graph(this: &JsImpQuad) -> JssTerm;

    #[wasm_bindgen(method, setter)]
    pub fn set_graph(this: &JsImpQuad, value: &JssTerm);

    #[wasm_bindgen(js_name=equals)]
    pub fn quads_equals(this: &JsImpQuad, other_quad: &JsImpQuad);
    
}


// Exportation of Sophia Quads


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

#[wasm_bindgen(js_name = Quad)]
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

    pub fn new_by_move(s: RcTerm, p: RcTerm, o: RcTerm, g: Option<RcTerm>) -> BJQuad {
        BJQuad {
            _subject: s,
            _predicate: p,
            _object: o,
            _graph: g
        }
    }
}

#[wasm_bindgen(js_class = Quad)]
impl BJQuad {
    pub fn is_connected_to_rust(&self) -> bool {
        true
    }

    #[wasm_bindgen(method, getter)]
    pub fn subject(&self) -> BJTerm {
        BJTerm::new(&self._subject)
    }

    #[wasm_bindgen(method, getter)]
    pub fn predicate(&self) -> BJTerm {
        BJTerm::new(&self._predicate)
    }

    #[wasm_bindgen(method, getter)]
    pub fn object(&self) -> BJTerm {
        BJTerm::new(&self._object)
    }

    #[wasm_bindgen(method, getter)]
    pub fn graph(&self) -> BJTerm {
        match &self._graph {
            None => BJTerm::default_graph(),
            Some(term) => BJTerm::new(term)
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
    pub fn equals(&self, other: Option<JsImpQuad>) -> bool {
        match &other {
            None => false,
            Some(other_quad) =>
                // TODO : Make a BJTerm that don't clone the items to reuse code without destroying performances
                // or use a cache mechanic (but it is worst)
                self.subject().equals(Some(other_quad.subject()))
                && self.predicate().equals(Some(other_quad.predicate()))
                && self.object().equals(Some(other_quad.object()))
                && self.graph().equals(Some(other_quad.graph()))
        }
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_subject(&mut self, other: &JssTerm) {
        self._subject = build_rcterm_from_jss_term(other).unwrap();
    }
    
    #[wasm_bindgen(method, setter)]
    pub fn set_predicate(&mut self, other: &JssTerm) {
        self._predicate = build_rcterm_from_jss_term(other).unwrap();
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_object(&mut self, other: &JssTerm) {
        self._object = build_rcterm_from_jss_term(other).unwrap();
    }
    
    #[wasm_bindgen(method, setter)]
    pub fn set_graph(&mut self, other: &JssTerm) {
        self._graph = build_rcterm_from_jss_term(other);
    }
}


// ============================================================================
// ==== RDF JS DataFactory

#[wasm_bindgen(js_name=DataFactory)]
pub struct BJDataFactory { }

// TODO : Determine if for dataset, the factory used should be the factory tied
// to the dataset instead of this factory

impl Clone for BJDataFactory {
    fn clone(&self) -> BJDataFactory {
        BJDataFactory{ }
    }
}

#[wasm_bindgen(js_class=DataFactory)]
impl BJDataFactory {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BJDataFactory {
        BJDataFactory{ }
    }

    #[wasm_bindgen(js_name="namedNode")]
    pub fn named_node(&self, value: String) -> BJTerm {
        BJTerm { term: Some(RcTerm::new_iri(value.to_string()).unwrap()) }
    }

    #[wasm_bindgen(js_name="blankNode")]
    pub fn blank_node(&self, value: Option<String>) -> BJTerm {
        // TODO : support optionnal
        // TODO : "If the value parameter is undefined a new identifier for the blank node is generated for each call."
        BJTerm { term: Some(RcTerm::new_bnode(value.unwrap().to_string()).unwrap()) }
    }

    #[wasm_bindgen(js_name="literal")]
    pub fn literal(&self, value: String, language_or_datatype: JsValue) -> BJTerm {
        if language_or_datatype.is_null() || language_or_datatype.is_undefined() {
            self.literal_from_string(value, "http://www.w3.org/2001/XMLSchema#string".into())
        } else {
            match language_or_datatype.as_string() {
                Some(language) => self.literal_from_string(value, language),
                None => self.literal_from_named_node(value, language_or_datatype.into())
            }
        }
    }

    #[wasm_bindgen(js_name="literalFromString")]
    pub fn literal_from_string(&self, value: String, language: String) -> BJTerm {
        BJTerm {
            term: Some(RcTerm::new_literal_lang(value, language).unwrap())
        }
    }

    #[wasm_bindgen(js_name="literalFromNamedNode")]
    pub fn literal_from_named_node(&self, value: String, named_node: JssTerm) -> BJTerm {
        let rcterm = build_rcterm_from_jss_term(&named_node);
        BJTerm { term: Some(RcTerm::new_literal_dt(value, rcterm.unwrap()).unwrap()) }
    }

    #[wasm_bindgen(js_name="variable")]
    pub fn variable(&self, value: String) -> BJTerm {
        BJTerm { term: Some(RcTerm::new_variable(value.to_string()).unwrap()) }
    }

    #[wasm_bindgen(js_name="defaultGraph")]
    pub fn default_graph(&self) -> BJTerm {
        BJTerm { term: None }
    }

    #[wasm_bindgen(js_name="quad")]
    pub fn quad(&self, subject: JssTerm, predicate: JssTerm, object: JssTerm, graph: Option<JssTerm>) -> BJQuad {
        BJQuad::new_by_move(
            build_rcterm_from_jss_term(&subject).unwrap(),
            build_rcterm_from_jss_term(&predicate).unwrap(),
            build_rcterm_from_jss_term(&object).unwrap(),
            match graph {
                None => None,
                Some(g) => build_rcterm_from_jss_term(&g)
            }
        )
    }

    #[wasm_bindgen(js_name="fromTerm")]
    pub fn from_term(&self, original: JssTerm) -> BJTerm {
        if original.term_type().as_str() == "DefaultGraph" {
            self.default_graph()
        } else {
            BJTerm { term: build_rcterm_from_jss_term(&original) }
        }
    }

    #[wasm_bindgen(js_name="fromQuad")]
    pub fn from_quad(&self, original: JsImpQuad) -> BJQuad {
        self.quad(
            original.subject(),
            original.predicate(),
            original.object(),
            Some(original.graph())
        )
    }
}


