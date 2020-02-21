#![allow(unused_imports)]
extern crate wasm_bindgen;

// TODO : remove unused imports when finished

use maybe_owned::MaybeOwned;
use std::any;
use std::rc::Rc;
use std::iter;
use uuid::Uuid;
use sophia::dataset::Dataset;
use sophia::dataset::DQuadSource;
use sophia::dataset::inmem::FastDataset;
use sophia::dataset::inmem::LightDataset;
use sophia::quad::Quad;
use sophia::quad::stream::QuadSink;
use sophia::quad::stream::QuadSource;
use sophia::graph::inmem::LightGraph;
use sophia::parser::trig;
use sophia::term::*;
use sophia::term::matcher::GraphNameMatcher;
use sophia::term::matcher::TermMatcher;
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


// ====================================================================================================================
// ==== ITERATORS

/// An iterator on the elements contained by an exported container.
///
/// The iterator we provide is an iterator on the elements that are contained when we create the iterator : new and
/// deleted elements in the origian container do not change the state of the iterator.
///
/// This feature is implemented as earlier versions of NodeJs do not support `js_sys::Array::values()`
#[wasm_bindgen]
pub struct RustExportIterator {
    array: js_sys::Array
}

impl RustExportIterator {
    /// Build an iterator from a `js_sys::Array`
    pub fn new(array: js_sys::Array) -> RustExportIterator {
        // We reverse in place so we can think our iterator as a list of quads we have not iterated on yet
        array.reverse();
        RustExportIterator { array }
    }
}

#[wasm_bindgen]
impl RustExportIterator {
    /// Returns an `RustExportIteratorNext` that contains to the next element
    pub fn next(&mut self) -> RustExportIteratorNext {
        if self.array.length() != 0 {
            RustExportIteratorNext{ current_element: Some(self.array.pop()) }
        } else {
            RustExportIteratorNext{ current_element: None }
        }
    }
}

/// An object that contains an element returned by `RustExportIterator::next`
///
/// It follows the Javascript specification, having a `done` attribute that tells if the iterator is empty and a
/// `value` attribute that contains the eventual value. They are modelized with an optional `JsValue`.
#[wasm_bindgen]
pub struct RustExportIteratorNext {
    #[wasm_bindgen(skip)]
    /// The JsValue contained by this object
    pub current_element: Option<JsValue>
}

#[wasm_bindgen]
impl RustExportIteratorNext {
    /// Return true if the iterator is empty
    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.current_element.is_none()
    }

    /// Return the possessed `JsValue`
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        match self.current_element.as_ref() {
            None => JsValue::undefined(),
            Some(real_value) => real_value.clone()
        }
    }
}


// ====================================================================================================================
// ==== DATASET

#[wasm_bindgen]
extern "C" {
    // TODO : check if we should give to import and exported objects the same js name or not

    #[wasm_bindgen(js_name=DatasetCore)]
    pub type JsImportDataset;

    #[wasm_bindgen(method, getter=getSophiaDatasetPtr)]
    pub fn get_sophia_dataset_ptr(this: &JsImportDataset) -> *mut SophiaExportDataset;
}

/// A Sophia `FastDataset` adapter that can be exported to an object that is almost compliant to a
/// [RDF.JS dataset](https://rdf.js.org/dataset-spec/#dataset-interface)
#[wasm_bindgen(js_name="DatasetCore")]
pub struct SophiaExportDataset {
    dataset: FastDataset
}

pub enum AlmostOption<T> {
    Some(T), None
}


/// An enum capturing the different states of variable during query processing.
/// Stoled to sophia_rs/src/query.rs
enum Binding<T> {
    /// The variable is bound to the given term.
    Bound(T),
    /// The variable is free.
    Free,
}

impl<T> From<Option<T>> for Binding<T> {
    fn from(src: Option<T>) -> Binding<T> {
        match src {
            Some(t) => Binding::Bound(t),
            None => Binding::Free,
        }
    }
}

impl TermMatcher for Binding<RcTerm> {
    type TermData = std::rc::Rc<str>;
    fn constant(&self) -> Option<&Term<Self::TermData>> {
        match self {
            Binding::Bound(t) => Some(t),
            Binding::Free => None,
        }
    }
    fn matches<T>(&self, t: &Term<T>) -> bool
    where
        T: TermData,
    {
        match self {
            Binding::Bound(tself) => tself == t,
            Binding::Free => true,
        }
    }
}

impl GraphNameMatcher for Binding<Option<RcTerm>> {
    type TermData = std::rc::Rc<str>;

    fn constant(&self) -> Option<Option<&Term<Self::TermData>>> {
        match self {
            Binding::Bound(t) => Some(t.as_ref()),
            Binding::Free => None,
        }
    }

    fn matches<T>(&self, t: Option<&Term<T>>) -> bool
    where
        T: TermData,
    {
        match self {
            Binding::Bound(Some(term)) => match t {
                Some(arg_t) => arg_t == term,
                None => false
            },
            Binding::Bound(None) => t.is_none(),
            Binding::Free => true,
        }
    }
}


pub struct MatchRequestOnRcTerm {
    s: Binding<RcTerm>,
    p: Binding<RcTerm>,
    o: Binding<RcTerm>,
    g: Binding<Option<RcTerm>>
}

impl MatchRequestOnRcTerm {
    pub fn new(subject: Option<JsImportTerm>,predicate: Option<JsImportTerm>,
        object: Option<JsImportTerm>, graph: Option<JsImportTerm>) -> MatchRequestOnRcTerm {
        
        let build_and_unwrap = |x| build_rcterm_from_js_import_term(x).unwrap();
        let s = Binding::from(subject.as_ref().map(build_and_unwrap));
        let p = Binding::from(predicate.as_ref().map(build_and_unwrap));
        let o = Binding::from(object.as_ref().map(build_and_unwrap));
        let g = Binding::from(graph.as_ref().map(build_rcterm_from_js_import_term));
        
        MatchRequestOnRcTerm { s: s, p: p, o: o, g: g }
    }
}


impl SophiaExportDataset {
    /// Returns true if this dataset contained the passed `FastDataset`
    pub fn contains_dataset(&self, other_dataset: &FastDataset) -> bool {
        other_dataset.quads()
        .into_iter()
        .all(|element_result| {
            let element = element_result.unwrap();
            self.dataset.contains(
                element.s(),
                element.p(),
                element.o(),
                element.g()
            ).unwrap()
        })
    }
}

impl SophiaExportDataset {
    /// Tries to convert a `JsImportDataset` to a `SophiaExportDataset`
    ///
    /// This function makes the assumption that the get_sophia_dataset_ptr() returns a non null value only if the
    /// imported object is an exported object (ie this library is the only one to implement the binded function)
    fn try_from<'a>(imported: &'a JsImportDataset) -> Option<&'a SophiaExportDataset> {
        let ptr = imported.get_sophia_dataset_ptr();

        if !ptr.is_null() {
            unsafe { ptr.as_ref() }
        } else {
            None
        }
    }

    fn extract_dataset<'a>(imported: &'a JsImportDataset) -> MaybeOwned<'a, FastDataset> {
        let ptr = imported.get_sophia_dataset_ptr();

        if !ptr.is_null() {
            let ref_ = unsafe { &*ptr };
            MaybeOwned::Borrowed(&ref_.dataset)
        } else {
            // TODO : there is probably a better dataset structure to just add quads and then iterate on
            let mut exported_dataset = SophiaExportDataset::new();
            
            // We us the fact that we can iterate on the dataset
            let import_as_js_value = JsValue::from(imported);
            let iterator = js_sys::try_iter(&import_as_js_value);
            match iterator {
                Ok(Some(iter)) => {
                    for js_value in iter {
                        match js_value {
                            Ok(some_value) => exported_dataset.add(some_value.into()),
                            _ => {}
                        }
                    }
                },
                _ => {
                    // We panic as we should have received a RDF JS compliant graph
                    panic!("SophiaExportDataset::extract_dataset : Didn't receive an iterable");
                }
            }
        
            MaybeOwned::Owned(exported_dataset.dataset)
        }
    }
}

#[wasm_bindgen(js_class="DatasetCore")]
impl SophiaExportDataset {
    /// Constructs an empty `FastDataset` that have a RDF.JS interface.
    #[wasm_bindgen(constructor)]
    pub fn new() -> SophiaExportDataset {
        SophiaExportDataset{ dataset: FastDataset::new() }
    }

    /// Returns a javascript style iterator on every quads on this dataset.
    #[wasm_bindgen(js_name="getIterator")]
    pub fn get_iterator(&self) -> RustExportIterator {
        // TODO : bind this function call to this[Symbol.iterator]
        // a.values() is not supported by every version of nodejs so we are forced to design our own iterator
        RustExportIterator::new(self.quads())
    }

    /// Returns a pointer on this object.
    /// 
    /// It is used as a way to detect if a javascript object that we received is an exported object by this library.
    #[wasm_bindgen(method, getter=getSophiaDatasetPtr)]
    pub fn get_sophia_dataset_ptr(&mut self) -> *mut SophiaExportDataset {
        self
    }

    /// Loads the content of a rdf graph formatted following the [TriG syntax](https://www.w3.org/TR/trig/)
    pub fn load(&mut self, content: &str) {
        let r = sophia::parser::trig::parse_str(&content).in_dataset(&mut self.dataset);
        match r {
            Ok(_) => {},
            Err(error) => log(error.to_string().as_str())
        }
    }
    
    /// Returns every term contained by the dataset
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

    /// Adds the given quad to this dataset
    #[wasm_bindgen(js_name="add")]
    pub fn add(&mut self, quad: JsImportQuad) {
        // As we use RcTerms, copy should be cheap and simple enough to not
        // have too much performances issues
        let sophia_quad = SophiaExportDataFactory::from_quad(quad);
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

    /// Deletes the passed quad from this dataset
    #[wasm_bindgen(js_name="delete")]
    pub fn delete(&mut self, quad: JsImportQuad) {
        // Fastdataset implements the trait SetDataset so this function removes
        // every occurrences of the passed quad
        let sophia_quad = SophiaExportDataFactory::from_quad(quad);
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
    
    /// Returns `true` if this dataset have the passed quad
    #[wasm_bindgen(js_name="has")]
    pub fn has_quad(&self, quad: JsImportQuad) -> bool {
        let sophia_quad = SophiaExportDataFactory::from_quad(quad);
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

    /// Returns a new dataset that contains every quad that matches the passed arguments.
    #[wasm_bindgen(js_name="match")]
    pub fn match_quad(&self, subject: Option<JsImportTerm>, predicate: Option<JsImportTerm>,
        object: Option<JsImportTerm>, graph: Option<JsImportTerm>) -> SophiaExportDataset {
        let m = MatchRequestOnRcTerm::new(subject, predicate, object, graph);

        let mut quads_iter = self.dataset.quads_matching(&m.s, &m.p, &m.o, &m.g);

        let mut dataset = FastDataset::new();
        quads_iter.in_dataset(&mut dataset).unwrap();
        
        SophiaExportDataset{ dataset: dataset }
    }

    /// Returns the number of quads contained by this dataset
    #[wasm_bindgen(getter = size)]
    pub fn get_size(&self) -> usize {
        self.dataset.quads().into_iter().count()
    }

    /// Adds every quad contained in the passed dataset or sequence
    #[wasm_bindgen(js_name="addAll")]
    pub fn add_all(&mut self, quads_as_jsvalue: JsValue) {
        // this addAll ((Dataset or sequence<Quad>) quads);
        // TODO : return this
        if quads_as_jsvalue.is_null() || quads_as_jsvalue.is_undefined() {
            return;
        }

        // Try to detect a SophiaExportDataset
        let imported_dataset = JsImportDataset::from(quads_as_jsvalue);

        match SophiaExportDataset::try_from(&imported_dataset) {
            Some(exported) => {
                exported.dataset.quads().in_dataset(&mut self.dataset).unwrap();
            },
            None => {
                // We get back our jsvalue and we use the fact that both a dataset and a sequence<quad> can be iterated on to
                // receive quads.
                let quads_as_jsvalue = JsValue::from(imported_dataset);

                let iterator = js_sys::try_iter(&quads_as_jsvalue);

                match iterator {
                    Ok(Some(iter)) => {
                        for js_value in iter {
                            match js_value {
                                Ok(some_value) => self.add(some_value.into()),
                                _ => {}
                            }
                        }
                    },
                    _ => {
                        // TODO : error management
                        log("SophiaExportDataset::add_all did not receive an iterable");
                    }
                }
            }
        }
    }

    /// Returns true if imported_dataset is contained by this dataset
    #[wasm_bindgen(js_name="contains")]
    pub fn contains(&self, imported_dataset: JsImportDataset) -> bool {
        // TODO : RDF.JS - "Blank Nodes will be normalized."
        let maybe_dataset = SophiaExportDataset::extract_dataset(&imported_dataset);
        self.contains_dataset(maybe_dataset.as_ref())
    }

    /// Delete every quad that matches the given quad components
    
    #[wasm_bindgen(js_name="deleteMatches")]
    pub fn delete_matches(&mut self, subject: Option<JsImportTerm>, predicate: Option<JsImportTerm>,
        object: Option<JsImportTerm>, graph: Option<JsImportTerm>) {
        // this deleteMatches(optional Term, Optional Term, Optional Term, Optional Term)
        // TODO : return this
        /*
        let m = MatchRequestOnRcTerm::new(subject, predicate, object, graph);
        self.dataset.remove_matching(
            m._s,
            
        );
*/
    }
    

    // this                              deleteMatches (optional Term subject, optional Term predicate, optional Term object, optional Term graph);
    // Dataset                           difference (Dataset other);
    // boolean                           equals (Dataset other);
    // boolean                           every (QuadFilterIteratee iteratee);
    // Dataset                           filter (QuadFilterIteratee iteratee);
    // void                              forEach (QuadRunIteratee iteratee);
    // Promise<Dataset>                  import (Stream stream);
    // Dataset                           intersection (Dataset other);
    // Dataset                           map (QuadMapIteratee iteratee);
    // any                               reduce (QuadReduceIteratee iteratee, optional any initialValue);
    // boolean                           some (QuadFilterIteratee iteratee);

    /// Returns an array that contains every quad contained by this dataset
    #[wasm_bindgen(js_name="toArray")]
    pub fn to_array(&self) -> js_sys::Array {
        self.quads()
    }

    // String                            toCanonical ();
    // Stream                            toStream ();

    /// Returns a string representation of the quads contained in the dataset
    #[wasm_bindgen(js_name="toString")]
    pub fn to_string(&self) -> String {
        self.dataset
            .quads()
            .map_quads(|q| 
                match q.g().as_ref() {
                    None    => format!("{0} {1} {2} .",     q.s().n3(), q.p().n3(), q.o().n3()),
                    Some(g) => format!("{0} {1} {2} {3} .", q.s().n3(), q.p().n3(), q.o().n3(), g.n3())
                }
            )
            .into_iter()
            .collect::<Result<Vec<String>, _>>()
            .unwrap()
            .join("\n")
    }
    
    // Dataset                           union (Dataset quads);
}

impl SophiaExportDataset {


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



/// Exportation of rust terms using an adapter that owns the term
#[wasm_bindgen(js_name="Term")]
pub struct SophiaExportTerm {
    /// The encapsulated Sophia Term. If `None`, this term describes the default graph.
    term: Option<RcTerm>
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
            None => String::from("(DefaultGraph)")
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
    
    #[wasm_bindgen(method, getter=getRustPtr)]
    pub fn quads_get_rust_ptr(this: &JsImportQuad) -> *mut SophiaExportQuad;
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

/* ======= */
/* THE LAB */

/*
#[wasm_bindgen]
pub struct Animal {
    pub name: u32
}

#[wasm_bindgen]
impl Animal {
    pub fn common(&self) {
        log("Animal::Common-RUST");
    }

    pub fn export_only(&self) {
        log("Animal::ExportOnly-RUST");
    }
}


#[wasm_bindgen]
pub fn animalog(js_value: JsValue) {
    let animalust = Animal::try_from(&js_value);

    if let Ok(a) = animalust {
        a.common();
        a.export_only();
    } else {
        let animals = AnimalImport::try_from(&js_value);

        if let Ok(a) = animals {
            a.common();
            a.import_only();
        } else {
            log("c'est nul :(");
        }
    }


}


#[wasm_bindgen]
extern "C" {
    pub type AnimalImport;

    #[wasm_bindgen]
    pub fn common(this: &AnimalImport);

    #[wasm_bindgen]
    pub fn import_only(this: &AnimalImport);
}

*/