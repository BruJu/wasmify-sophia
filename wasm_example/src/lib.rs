#![allow(unused_imports)]
extern crate wasm_bindgen;

// TODO : remove unused imports when finished
// TODO : find how to use folder instead of using _
mod datamodel_term;
use crate::datamodel_term::*;

mod matchableterm;
use crate::matchableterm::MatchableTerm;

mod exportiterator;
use crate::exportiterator::RustExportIterator;


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




pub struct MatchRequestOnRcTerm {
    s: MatchableTerm<RcTerm>,
    p: MatchableTerm<RcTerm>,
    o: MatchableTerm<RcTerm>,
    g: MatchableTerm<Option<RcTerm>>
}

impl MatchRequestOnRcTerm {
    pub fn new(subject: Option<JsImportTerm>,predicate: Option<JsImportTerm>,
        object: Option<JsImportTerm>, graph: Option<JsImportTerm>) -> MatchRequestOnRcTerm {
        
        let build_and_unwrap = |x| build_rcterm_from_js_import_term(x).unwrap();
        let s = MatchableTerm::from(subject.as_ref().map(build_and_unwrap));
        let p = MatchableTerm::from(predicate.as_ref().map(build_and_unwrap));
        let o = MatchableTerm::from(object.as_ref().map(build_and_unwrap));
        let g = MatchableTerm::from(graph.as_ref().map(build_rcterm_from_js_import_term));
        
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
            
            // We use the fact that we can iterate on the dataset
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
        
        let m = MatchRequestOnRcTerm::new(subject, predicate, object, graph);
        self.dataset.remove_matching(&m.s, &m.p, &m.o, &m.g).unwrap();
    }
    
    /// Returns a new dataset which contains the elements of this dataset that are not included in the imported_dataset
    #[wasm_bindgen(js_name="difference")]
    pub fn difference(&self, imported_dataset: JsImportDataset) -> SophiaExportDataset {
        let other = SophiaExportDataset::extract_dataset(&imported_dataset);

        let mut ds = FastDataset::new();

        self.dataset.quads()
            .filter(|quad| {
                let quad = quad.as_ref().unwrap();
                !other.contains(quad.s(), quad.p(), quad.o(), quad.g()).unwrap()
            })
            .in_dataset(&mut ds);

        SophiaExportDataset { dataset: ds }
    }

    /// Returns true if the two datasets are equals
    #[wasm_bindgen(js_name="equals")]
    pub fn equals(&self, imported_dataset: JsImportDataset) -> bool {
        let other = SophiaExportDataset::extract_dataset(&imported_dataset);

        self.get_size() == other.quads().into_iter().count()
            && self.contains_dataset(&other)
    }

    /// Returns a dataset with the elements that are contained by both dataset
    #[wasm_bindgen(js_name="intersection")]
    pub fn intersection(&self, imported_dataset: JsImportDataset) -> SophiaExportDataset {
        let other = SophiaExportDataset::extract_dataset(&imported_dataset);

        let mut ds = FastDataset::new();

        self.dataset.quads()
            .filter(|quad| {
                let quad = quad.as_ref().unwrap();
                other.contains(quad.s(), quad.p(), quad.o(), quad.g()).unwrap()
            })
            .in_dataset(&mut ds);

        SophiaExportDataset { dataset: ds }
    }

    /// Returns a dataset that contains all quads from the two graphs
    #[wasm_bindgen(js_name="union")]
    pub fn union(&self, imported_dataset: JsImportDataset) -> SophiaExportDataset {
        let other = SophiaExportDataset::extract_dataset(&imported_dataset);

        let mut ds = FastDataset::new();

        self.dataset.quads().in_dataset(&mut ds);
        other.quads().in_dataset(&mut ds);

        SophiaExportDataset { dataset: ds }
    }

    #[wasm_bindgen(js_name="forEach")]
    pub fn for_each(&self, quad_run_iteratee: &js_sys::Function) {
        self.dataset.quads()
            .into_iter()
            .for_each(|quad| {
            let quad = quad.unwrap();
            let export_quad = SophiaExportQuad::new(quad.s(), quad.p(), quad.o(), quad.g());
            let js_value = JsValue::from(export_quad);
            quad_run_iteratee.call1(&JsValue::NULL, &js_value).unwrap();
        });
    }

    #[wasm_bindgen(js_name="some")]
    pub fn some(&self, filter_function: &js_sys::Function) -> bool {
        self.dataset.quads()
            .into_iter()
            .any(|quad| {
            let quad = quad.unwrap();
            let export_quad = SophiaExportQuad::new(quad.s(), quad.p(), quad.o(), quad.g());
            let js_value = JsValue::from(export_quad);
            filter_function.call1(&JsValue::NULL, &js_value).unwrap().is_truthy()
        })
    }

    #[wasm_bindgen(js_name="every")]
    pub fn every(&self, filter_function: &js_sys::Function) -> bool {
        self.dataset.quads()
            .into_iter()
            .all(|quad| {
            let quad = quad.unwrap();
            let export_quad = SophiaExportQuad::new(quad.s(), quad.p(), quad.o(), quad.g());
            let js_value = JsValue::from(export_quad);
            filter_function.call1(&JsValue::NULL, &js_value).unwrap().is_truthy()
        })
    }

    // Dataset          filter (QuadFilterIteratee iteratee);
    #[wasm_bindgen(js_name="filter")]
    pub fn filter(&self, filter_function: &js_sys::Function) -> SophiaExportDataset {
        let mut ds = FastDataset::new();

        self.dataset.quads()
            .filter_quads(|quad| {
            let export_quad = SophiaExportQuad::new(quad.s(), quad.p(), quad.o(), quad.g());
            let js_value = JsValue::from(export_quad);
            filter_function.call1(&JsValue::NULL, &js_value).unwrap().is_truthy()
        })
            .in_dataset(&mut ds);

        SophiaExportDataset { dataset: ds }
    }


    // Dataset          map (QuadMapIteratee iteratee);
    #[wasm_bindgen(js_name="map")]
    pub fn map(&self, map_function: &js_sys::Function) -> SophiaExportDataset {
        let mut ds = SophiaExportDataset::new();

        self.dataset.quads()
            .for_each_quad(|quad| {
                let export_quad = SophiaExportQuad::new(quad.s(), quad.p(), quad.o(), quad.g());
                let js_value = JsValue::from(export_quad);
                let mapped_js_quad = map_function.call1(&JsValue::NULL, &js_value).unwrap();
                let mapped_quad = JsImportQuad::from(mapped_js_quad);
                ds.add(mapped_quad);
            })
            .unwrap();

        ds
    }

    // Promise<Dataset> import (Stream stream);
    // any              reduce (QuadReduceIteratee iteratee, optional any initialValue);

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
    
}


// ============================================================================
// ==== RDF JS Term


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
