
use sophia::dataset::Dataset;
use sophia::dataset::MutableDataset;
use sophia::term::matcher::AnyOrExactly;
use crate::datamodel_term::*;
use sophia::term::RcTerm;
use std::default::Default;
use sophia::quad::stream::QuadSource;
use crate::datamodel_quad::*;
use crate::datamodel_factory::*;
use wasm_bindgen::JsValue;
use sophia::quad::Quad;
use maybe_owned::MaybeOwned;
use sophia::term::BoxTerm;

use js_sys::Reflect;

fn build_anyorexactly_for_term(js_parameter: &JsImportTerm) -> AnyOrExactly<RcTerm> {
    if js_parameter.is_null() || js_parameter.is_undefined() {
        AnyOrExactly::Any
    } else {
        AnyOrExactly::Exactly(build_rcterm_from_js_import_term(js_parameter).unwrap())
    }
}

fn build_anyorexactly_for_graph(js_parameter: &JsImportTerm) -> AnyOrExactly<Option<RcTerm>> {
    if js_parameter.is_null() || js_parameter.is_undefined() {
        AnyOrExactly::Any
    } else {
        AnyOrExactly::Exactly(build_rcterm_from_js_import_term(js_parameter))
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
    pub fn new(subject: &JsImportTerm, predicate: &JsImportTerm, object: &JsImportTerm, graph: &JsImportTerm) -> MatchRequestOnRcTerm {
        
        MatchRequestOnRcTerm {
            s: build_anyorexactly_for_term(subject),
            p: build_anyorexactly_for_term(predicate),
            o: build_anyorexactly_for_term(object),
            g: build_anyorexactly_for_graph(graph)
        }
    }
}


pub struct ExportableDatasetBase<D> where D: Dataset + Default {
    pub inner_dataset: D
}

trait ExportableDataset<D>: Default
    where D: MutableDataset + Default {
    fn mutable_inner(&mut self) -> &mut ExportableDatasetBase<D>;
    fn inner(&self) -> &ExportableDatasetBase<D>;

    fn mutable_dataset(&mut self) -> &mut D {
        &mut self.mutable_inner().inner_dataset
    }

    fn dataset(&self) -> &D {
        &self.inner().inner_dataset
    }

    fn wrap(dataset: D) -> Self;

    // fn get_raw_pointer(&self) -> *const Self {
    //     self
    // }

    fn match_quad(&self, subject: &JsImportTerm, predicate: &JsImportTerm, object: &JsImportTerm, graph: &JsImportTerm) -> Self {
        let wrapped_match = self.inner().match_quad(subject, predicate, object, graph);
        Self::wrap(wrapped_match.inner_dataset)
    }

    fn size(&self) -> usize {
        self.inner().size()
    }

    fn add(&mut self, quad: &JsImportQuad);

    fn delete(&mut self, quad: &JsImportQuad);

    fn has_quad(&self, quad: &JsImportQuad) -> bool;

    fn add_all(&mut self, quads_as_jsvalue: &JsValue);
    
    fn try_from<'a>(imported: &'a JsValue) -> Option<&'a Self> {
        // TODO : in trait, put method get_uniqueid which is exported and compared here

        let rust_managed = Reflect::get(imported, &JsValue::from_str("rust_managed"));
        if rust_managed.is_err() {
            return None;
        }

        let rust_managed = rust_managed.unwrap();
        let rust_managed = rust_managed.as_f64().unwrap() as u32;

        unsafe {
            let ptr: *const Self = rust_managed as *const Self;
            
            if !ptr.is_null() {
                ptr.as_ref()
            } else {
                None
            }
        }
    }

    fn contains(&self, imported: &JsValue) -> bool;

    fn extract_dataset<'a>(imported: &'a JsValue) -> MaybeOwned<'a, Self> {
        let that = Self::try_from(imported);

        match that {
            Some(value) => MaybeOwned::Borrowed(value),
            None => {
                // TODO : there is probably a better dataset structure to just add quads and then iterate on
                let mut exported_dataset = Self::default();
                
                // We use the fact that we can iterate on the dataset
                let import_as_js_value = JsValue::from(imported);
                let iterator = js_sys::try_iter(&import_as_js_value);
                match iterator {
                    Ok(Some(iter)) => {
                        for js_value in iter {
                            match js_value {
                                Ok(some_value) => exported_dataset.add(&some_value.into()),
                                _ => {}
                            }
                        }
                    },
                    _ => {
                        // We panic as we should have received a RDF JS compliant graph
                        panic!("SophiaExportDataset::extract_dataset : Didn't receive an iterable");
                    }
                }
            
                MaybeOwned::Owned(exported_dataset)
            }
        }
    }

    fn delete_matches(&mut self, subject: &JsImportTerm, predicate: &JsImportTerm, object: &JsImportTerm, graph: &JsImportTerm);
}

impl<D> Default for ExportableDatasetBase<D>
    where D: Dataset + Default {
    fn default() -> Self {
        Self { inner_dataset: D::default() }
    }
}

impl<D> ExportableDataset<D> for ExportableDatasetBase<D>
    where D: MutableDataset + Default {

    fn mutable_inner(&mut self) -> &mut ExportableDatasetBase<D> {
        self
    }

    fn inner(&self) -> &ExportableDatasetBase<D> {
        self
    }

    fn wrap(dataset: D) -> Self {
        Self { inner_dataset: dataset }
    }

    fn match_quad(&self, subject: &JsImportTerm, predicate: &JsImportTerm, object: &JsImportTerm, graph: &JsImportTerm) -> Self {
        let m = MatchRequestOnRcTerm::new(subject, predicate, object, graph);
        let mut quads_iter = self.dataset().quads_matching(&m.s, &m.p, &m.o, &m.g);
        let mut dataset = D::default();
        quads_iter.in_dataset(&mut dataset).unwrap();
        Self::wrap(dataset)
    }

    fn size(&self) -> usize {
        match self.dataset().quads().into_iter().size_hint() {
            (v1, Some(v2)) if v1 == v2 => v1,
            _ => self.dataset().quads().count()
        }
    }

    fn add(&mut self, quad: &JsImportQuad) {
        self.mutable_dataset().insert(
            &build_stringterm_from_js_import_term(&quad.subject()).unwrap(),
            &build_stringterm_from_js_import_term(&quad.predicate()).unwrap(),
            &build_stringterm_from_js_import_term(&quad.object()).unwrap(),
            build_stringterm_from_js_import_term(&quad.graph()).as_ref(),
        ).unwrap();
    }

    fn delete(&mut self, quad: &JsImportQuad) {
        let sophia_quad = SophiaExportDataFactory::from_quad(quad);
        self.mutable_dataset().remove(
            &sophia_quad._subject,
            &sophia_quad._predicate,
            &sophia_quad._object,
            match &sophia_quad._graph {
                None => None,
                Some(x) => Some(x)
            }
        ).unwrap();
    }

    fn has_quad(&self, quad: &JsImportQuad) -> bool {
        let sophia_quad = SophiaExportDataFactory::from_quad(quad);
        self.dataset().contains(
            &sophia_quad._subject,
            &sophia_quad._predicate,
            &sophia_quad._object,
            match &sophia_quad._graph {
                None => None,
                Some(x) => Some(x)
            }
        ).unwrap()
    }

    fn add_all(&mut self, quads_as_jsvalue: &JsValue) {
        if quads_as_jsvalue.is_null() || quads_as_jsvalue.is_undefined() {
            return;
        }

        // Try to detect a SophiaExportDataset
        match Self::try_from(&quads_as_jsvalue.clone()) {
            Some(exported) => {
                exported.dataset().quads().in_dataset(self.mutable_dataset()).unwrap();
            },
            None => {
                // We get back our jsvalue and we use the fact that both a dataset and a sequence<quad> can be iterated on to
                // receive quads.
                let iterator = js_sys::try_iter(&quads_as_jsvalue);

                match iterator {
                    Ok(Some(iter)) => {
                        for js_value in iter {
                            match js_value {
                                Ok(some_value) => self.add(&some_value.into()),
                                _ => {}
                            }
                        }
                    },
                    _ => {
                        // TODO : error management
                        // log("SophiaExportDataset::add_all did not receive an iterable");
                    }
                }
            }
        }
    }

    fn contains(&self, imported: &JsValue) -> bool {
        let maybe_dataset = Self::extract_dataset(imported);
        self.contains_dataset(maybe_dataset.dataset())
    }

    fn delete_matches(&mut self, subject: &JsImportTerm, predicate: &JsImportTerm, object: &JsImportTerm, graph: &JsImportTerm) {
        let m = MatchRequestOnRcTerm::new(subject, predicate, object, graph);
        // self.mutable_dataset().remove_matching(&m.s, &m.p, &m.o, &m.g).unwrap();
    }
}


impl<D> ExportableDatasetBase<D> where D: Dataset + Default {
    pub fn contains_dataset<OD>(&self, other_dataset: &OD) -> bool
        where OD: Dataset {
        other_dataset.quads()
            .into_iter()
            .all(|element_result| {
                let element = element_result.unwrap();
                self.inner_dataset.contains(
                    element.s(),
                    element.p(),
                    element.o(),
                    element.g()
                ).unwrap()
            })
    }
}

// TODO : new_from_trig
// TODO : add_trigs
// TODO : export_trig

