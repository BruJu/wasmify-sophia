//! This module let users use a macro to export any Sophia Dataset into an
//! almost RDF.JS compliant web assembly class.

#![deny(missing_docs)]

extern crate wasm_bindgen;

use crate::datamodel_term::*;
use sophia::term::*;
use sophia::term::matcher::AnyOrExactly;

// -----------------------------------------------------
// ==== Implementation Detail : Match Request Conversion

fn build_anyorexactly_for_term(js_parameter: Option<JsImportTerm>) -> AnyOrExactly<RcTerm> {
    match js_parameter {
        None => AnyOrExactly::Any,
        Some(js_term) => AnyOrExactly::Exactly(build_rcterm_from_js_import_term(&js_term).unwrap())
    }
}

fn build_anyorexactly_for_graph(js_parameter: Option<JsImportTerm>) -> AnyOrExactly<Option<RcTerm>> {
    match js_parameter {
        None => AnyOrExactly::Any,
        Some(js_term) => match build_rcterm_from_js_import_term(&js_term) {
            Some(re_js_term) => AnyOrExactly::Exactly(Some(re_js_term)),
            None => AnyOrExactly::Exactly(None)
        }
    }
}


/// A list of AnyOrExactly MatchTerms to build a match request on a Sophia dataset
pub struct MatchRequestOnRcTerm {
    /// The any of exactly subject matcher
    pub s: AnyOrExactly<RcTerm>,
    /// The any of exactly predicate matcher
    pub p: AnyOrExactly<RcTerm>,
    /// The any of exactly object matcher
    pub o: AnyOrExactly<RcTerm>,
    /// The any of exactly graph matcher
    pub g: AnyOrExactly<Option<RcTerm>>
}

impl MatchRequestOnRcTerm {
    /// Builds a `MatchRequestOnRcTerm` from `JsImportTerms`
    pub fn new(subject: Option<JsImportTerm>, predicate: Option<JsImportTerm>,
        object: Option<JsImportTerm>, graph: Option<JsImportTerm>) -> MatchRequestOnRcTerm {
        
        MatchRequestOnRcTerm {
            s: build_anyorexactly_for_term(subject),
            p: build_anyorexactly_for_term(predicate),
            o: build_anyorexactly_for_term(object),
            g: build_anyorexactly_for_graph(graph)
        }
    }
}


#[macro_export]
macro_rules! export_sophia_dataset {
    ($rust_export_name:ident, $rust_import_name: ident, $js_name: expr,
        $wrapped_dataset: ident) => (
        // =====================================================================
        //   ---- IMPORT ---- IMPORT ---- IMPORT ---- IMPORT ---- IMPORT ----
        #[wasm_bindgen]
        extern "C" {
            // TODO : check if we should give to import and exported objects the same js name or not
        
            /// A dataset managed that is imported from Javascript
            
            #[wasm_bindgen(js_name=$js_name)]
            pub type $rust_import_name;
        
            /// Returns a pointer to the intern memory of Rust if this dataset is
            /// managed by it.
            /// 
            /// This method can be very unsafe because it supposes only
            /// `SophiaExportDataset` has this method.
            #[wasm_bindgen(method, getter=getSophiaDatasetPtr)]
            pub fn get_sophia_dataset_ptr(this: &$rust_import_name) -> *const $rust_export_name;
        }

        /// A Sophia `FastDataset` adapter that can be exported to an object that is almost compliant to a
        /// [RDF.JS dataset](https://rdf.js.org/dataset-spec/#dataset-interface)
        #[wasm_bindgen(js_name=$js_name)]
        pub struct $rust_export_name {
            dataset: $wrapped_dataset
        }

        // =====================================================================
        //               ---- EXPORT NON RDF JS FEATURES ----

        #[wasm_bindgen(js_class=$js_name)]
        impl $rust_export_name {
            /// Constructs an empty `FastDataset` that have a RDF.JS interface.
            #[wasm_bindgen(constructor)]
            pub fn new() -> $rust_export_name {
                $rust_export_name{ dataset: $wrapped_dataset::new() }
            }
        
            /// Returns a pointer on this object.
            /// 
            /// It is used as a way to detect if a javascript object that we received is an exported object by this library.
            #[wasm_bindgen(method, getter=getSophiaDatasetPtr)]
            pub fn get_sophia_dataset_ptr(&self) -> *const $rust_export_name {
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
        
            /// Returns an array that contains every quads contained by this dataset
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
        }

        // =====================================================================
        //               ---- EXPORT DATASETCORE ----


        #[wasm_bindgen(js_class=$js_name)]
        impl $rust_export_name {
            /// Returns the number of quads contained by this dataset
            #[wasm_bindgen(getter=size)]
            pub fn get_size(&self) -> usize {
                // We try to use hint to avoid iterating on every quads, currently it doesnt work
                match self.dataset.quads().into_iter().size_hint() {
                    (v1, Some(v2)) if v1 == v2 => v1,
                    _ => self.dataset.quads().count()
                }
            }

            /// Returns a javascript style iterator on every quads on this dataset.
            #[wasm_bindgen(js_name="getIterator")]
            pub fn get_iterator(&self) -> RustExportIterator {
                // TODO : bind this function call to this[Symbol.iterator]
                // a.values() is not supported by every version of nodejs so we are forced to design our own iterator
                RustExportIterator::new(self.quads())
            }

            /// Adds the given quad to this dataset
            #[wasm_bindgen(js_name="add")]
            pub fn add(&mut self, quad: JsImportQuad) {
                self.dataset.insert(
                    &build_stringterm_from_js_import_term(&quad.subject()).unwrap(),
                    &build_stringterm_from_js_import_term(&quad.predicate()).unwrap(),
                    &build_stringterm_from_js_import_term(&quad.object()).unwrap(),
                    build_stringterm_from_js_import_term(&quad.graph()).as_ref(),
                ).unwrap();

                // TODO : return this
            }

            /// Deletes the passed quad from this dataset
            #[wasm_bindgen(js_name="delete")]
            pub fn delete(&mut self, quad: JsImportQuad) {
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
                object: Option<JsImportTerm>, graph: Option<JsImportTerm>) -> $rust_export_name {
                let m = $crate::dataset_macro::MatchRequestOnRcTerm::new(subject, predicate, object, graph);

                let mut quads_iter = self.dataset.quads_matching(&m.s, &m.p, &m.o, &m.g);

                let mut dataset = $wrapped_dataset::new();
                quads_iter.in_dataset(&mut dataset).unwrap();
                
                $rust_export_name{ dataset: dataset }
            }
        }

                
        #[wasm_bindgen(js_class=$js_name)]
        impl $rust_export_name {
            /// Adds every quad contained in the passed dataset or sequence
            #[wasm_bindgen(js_name="addAll")]
            pub fn add_all(&mut self, quads_as_jsvalue: JsValue) {
                // this addAll ((Dataset or sequence<Quad>) quads);
                // TODO : return this
                if quads_as_jsvalue.is_null() || quads_as_jsvalue.is_undefined() {
                    return;
                }

                // Try to detect a SophiaExportDataset
                let imported_dataset = $rust_import_name::from(quads_as_jsvalue);

                match $rust_export_name::try_from(&imported_dataset) {
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
            pub fn contains(&self, imported_dataset: &$rust_import_name) -> bool {
                // TODO : RDF.JS - "Blank Nodes will be normalized."
                let maybe_dataset = $rust_export_name::extract_dataset(imported_dataset);
                self.contains_dataset(maybe_dataset.as_ref())
            }

            /// Delete every quad that matches the given quad components
            #[wasm_bindgen(js_name="deleteMatches")]
            pub fn delete_matches(&mut self, subject: Option<JsImportTerm>, predicate: Option<JsImportTerm>,
                object: Option<JsImportTerm>, graph: Option<JsImportTerm>) {
                // this deleteMatches(optional Term, Optional Term, Optional Term, Optional Term)
                // TODO : return this
                
                let m = $crate::dataset_macro::MatchRequestOnRcTerm::new(subject, predicate, object, graph);
                self.dataset.remove_matching(&m.s, &m.p, &m.o, &m.g).unwrap();
            }
            
            /// Returns a new dataset which contains the elements of this dataset that are not included in the imported_dataset
            #[wasm_bindgen(js_name="difference")]
            pub fn difference(&self, imported_dataset: &$rust_import_name) -> $rust_export_name {
                let other = $rust_export_name::extract_dataset(imported_dataset);

                let mut ds = $wrapped_dataset::new();

                self.dataset.quads()
                    .filter(|quad| {
                        let quad = quad.as_ref().unwrap();
                        !other.contains(quad.s(), quad.p(), quad.o(), quad.g()).unwrap()
                    })
                    .in_dataset(&mut ds).unwrap();

                $rust_export_name { dataset: ds }
            }

            /// Returns true if the two datasets are equals
            #[wasm_bindgen(js_name="equals")]
            pub fn equals(&self, imported_dataset: &$rust_import_name) -> bool {
                let other = $rust_export_name::extract_dataset(imported_dataset);

                self.get_size() == other.quads().into_iter().count()
                    && self.contains_dataset(&other)
            }

            /// Returns a dataset with the elements that are contained by both dataset
            #[wasm_bindgen(js_name="intersection")]
            pub fn intersection(&self, imported_dataset: &$rust_import_name) -> $rust_export_name {
                let other = $rust_export_name::extract_dataset(imported_dataset);

                let mut ds = $wrapped_dataset::new();

                self.dataset.quads()
                    .filter(|quad| {
                        let quad = quad.as_ref().unwrap();
                        other.contains(quad.s(), quad.p(), quad.o(), quad.g()).unwrap()
                    })
                    .in_dataset(&mut ds).unwrap();

                $rust_export_name { dataset: ds }
            }

            /// Returns a dataset that contains all quads from the two graphs
            #[wasm_bindgen(js_name="union")]
            pub fn union(&self, imported_dataset: &$rust_import_name) -> $rust_export_name {
                let other = $rust_export_name::extract_dataset(imported_dataset);

                let mut ds = $wrapped_dataset::new();

                self.dataset.quads().in_dataset(&mut ds).unwrap();
                other.quads().in_dataset(&mut ds).unwrap();

                $rust_export_name { dataset: ds }
            }

            /// Produces an action for each quad of the dataset
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

            /// Returns true if the result of `filter_function` is true for at least
            /// one quad of the dataset
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

            /// Returns true if the result of the passed function is true for every
            /// quad of the dataset
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

            /// Returns a new dataset which contains every quads of this dataset that
            /// verifies the `filter_functio`
            #[wasm_bindgen(js_name="filter")]
            pub fn filter(&self, filter_function: &js_sys::Function) -> $rust_export_name {
                let mut ds = $wrapped_dataset::new();

                self.dataset.quads()
                    .filter_quads(|quad| {
                        let export_quad = SophiaExportQuad::new(quad.s(), quad.p(), quad.o(), quad.g());
                        let js_value = JsValue::from(export_quad);
                        filter_function.call1(&JsValue::NULL, &js_value).unwrap().is_truthy()
                    })
                    .in_dataset(&mut ds)
                    .unwrap();

                $rust_export_name { dataset: ds }
            }


            /// Builds a new dataset which contains quads that have been built by
            /// applying the map function to every quad of this dataset
            #[wasm_bindgen(js_name="map")]
            pub fn map(&self, map_function: &js_sys::Function) -> $rust_export_name {
                let mut ds = $rust_export_name::new();

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

            /// Returns an array that contains every quad contained by this dataset
            #[wasm_bindgen(js_name="toArray")]
            pub fn to_array(&self) -> js_sys::Array {
                self.quads()
            }

            /// Returns a string representation of the quads contained in the dataset
            #[wasm_bindgen(js_name="toString")]
            pub fn to_string(&self) -> String {
                self.dataset
                    .quads()
                    .map_quads(|q| 
                        match q.g().as_ref() {
                            None    => format!("{0} {1} {2} .",     q.s(), q.p(), q.o()),
                            Some(g) => format!("{0} {1} {2} {3} .", q.s(), q.p(), q.o(), g)
                        }
                    )
                    .into_iter()
                    .collect::<Result<Vec<String>, _>>()
                    .unwrap()
                    .join("\n")
            }

            /// Reduces the whole dataset to a single element
            /// 
            /// The behavior of this function matches the `Array.prototype.reduce`
            /// function from EcmaScript.
            #[wasm_bindgen(js_name="reduce")]
            pub fn reduce(&self, reducer: js_sys::Function, initial_value: JsValue) -> JsValue {
                let mut iterator = self.dataset.quads();
                let mut accumulated_value = initial_value;

                if accumulated_value.as_ref().is_undefined() {
                    let first_iter = iterator.next();
                    match first_iter {
                        None => return accumulated_value,
                        Some(quad_result) => {
                            let quad = quad_result.unwrap();
                            let export_quad = SophiaExportQuad::new(
                                quad.s(), quad.p(), quad.o(), quad.g()
                            );

                            accumulated_value = JsValue::from(export_quad);
                        }
                    }
                }

                while let Some(quad_result) = iterator.next() {
                    let quad = quad_result.unwrap();
                    let export_quad = SophiaExportQuad::new(
                        quad.s(), quad.p(), quad.o(), quad.g()
                    );
                    let to_accumulate = JsValue::from(export_quad);
                    accumulated_value = reducer.call2(&JsValue::NULL, &accumulated_value, &to_accumulate).unwrap();
                }

                accumulated_value
            }

            // Promise<Dataset> import (Stream stream);
            // Not implemented : String toCanonical ();
            // Not implemented : Stream toStream ();
        }


        impl $rust_export_name {
            /// Returns true if this dataset contained the passed `FastDataset`
            pub fn contains_dataset(&self, other_dataset: &$wrapped_dataset) -> bool {
                // TODO : contains any dataset
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
        
            /// Tries to convert a `JsImportDataset` to a `SophiaExportDataset`
            ///
            /// This function makes the assumption that the get_sophia_dataset_ptr() returns a non null value only if the
            /// imported object is an exported object (ie this library is the only one to implement the binded function)
            fn try_from<'a>(imported: &'a $rust_import_name) -> Option<&'a $rust_export_name> {
                let ptr = imported.get_sophia_dataset_ptr();
        
                if !ptr.is_null() {
                    unsafe { ptr.as_ref() }
                } else {
                    None
                }
            }
        
            fn extract_dataset<'a>(imported: &'a $rust_import_name) -> MaybeOwned<'a, $wrapped_dataset> {
                let ptr = imported.get_sophia_dataset_ptr();
        
                if !ptr.is_null() {
                    let ref_ = unsafe { &*ptr };
                    MaybeOwned::Borrowed(&ref_.dataset)
                } else {
                    // TODO : there is probably a better dataset structure to just add quads and then iterate on
                    let mut exported_dataset = $rust_export_name::new();
                    
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
    )
}
