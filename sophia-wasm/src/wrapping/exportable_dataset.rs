
use sophia::dataset::Dataset;
use sophia::dataset::MutableDataset;
use sophia::term::matcher::AnyOrExactly;
use crate::datamodel::term::*;
use sophia::term::RcTerm;
use std::default::Default;
use sophia::quad::stream::QuadSource;
use crate::datamodel::quad::*;
use crate::datamodel::factory::*;
use wasm_bindgen::JsValue;
use sophia::quad::Quad;
use maybe_owned::MaybeOwned;
use crate::exportiterator::RustExportIterator;
use sophia::serializer::QuadSerializer;
use sophia::serializer::Stringifier;

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

/// A trait that describes a wrapper that implements the logic to exports a 
/// Sophia Dataset to a RDF.JS Dataset
/// 
/// This trait is intended to wrap a structure that implements the `Dataset` from
/// the Sophia library and implement the logic for every operation that is expected
/// for a RDF.JS Dataset.
/// 
/// This structure is expected to be wrapped by another structure using the
/// `wasm_bindgen_wrappeddataset` macro.
/// 
/// Default implementations are provided for every method, so the only operation
/// the developer is expected to provide are the access operations to the dataset.
/// 
/// If no operation is redefined, the `wasm_bindgen_dataset` macro can be used
/// instead, which builds both a default implementation for this trait and an export
/// for wasm_bindgen.
pub trait ExportableDataset<D>: Default
    where D: MutableDataset + Default,
        <D as MutableDataset>::MutationError: From<<D as Dataset>::Error>,
        <D as MutableDataset>::MutationError: From<std::convert::Infallible> {
    
    /// Builds an instance that wraps the given dataset
    fn wrap(dataset: D) -> Self;

    /// Returns a mutable reference of the contained dataset
    fn mutable_dataset(&mut self) -> &mut D;

    /// Returns a const reference to the contained dataset
    fn dataset(&self) -> &D;


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
        match Self::try_from(&quads_as_jsvalue) {
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
                        crate::util::log("ExportableDataset::add_all did not receive an iterable");
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
        self.mutable_dataset().remove_matching(&m.s, &m.p, &m.o, &m.g).unwrap();
    }

    fn difference(&self, imported: &JsValue) -> Self {
        let other = Self::extract_dataset(imported);

        let mut dest = D::default();

        self.dataset().quads()
            .filter(|quad| {
                let quad = quad.as_ref().unwrap();
                !other.dataset().contains(quad.s(), quad.p(), quad.o(), quad.g()).unwrap()
            })
            .in_dataset(&mut dest).unwrap();

        Self::wrap(dest)
    }
    
    fn intersection(&self, imported: &JsValue) -> Self {
        let other = Self::extract_dataset(imported);

        let mut dest = D::default();

        self.dataset().quads()
            .filter(|quad| {
                let quad = quad.as_ref().unwrap();
                other.dataset().contains(quad.s(), quad.p(), quad.o(), quad.g()).unwrap()
            })
            .in_dataset(&mut dest).unwrap();

        Self::wrap(dest)
    }

    fn union(&self, imported: &JsValue) -> Self {
        let other = Self::extract_dataset(imported);

        let mut ds = D::default();

        self.dataset().quads().in_dataset(&mut ds).unwrap();
        other.dataset().quads().in_dataset(&mut ds).unwrap();

        Self::wrap(ds)
    }

    fn equals(&self, imported: &JsValue) -> bool {
        let other = Self::extract_dataset(imported);
        self.size() == other.size() && self.contains_dataset(other.dataset())
    }

    fn for_each(&self, quad_run_iteratee: &js_sys::Function) {
        self.dataset().quads()
            .into_iter()
            .for_each(|quad| {
                let quad = quad.unwrap();
                let export_quad = SophiaExportQuad::new_from_quad(&quad);
                let js_value = JsValue::from(export_quad);
                quad_run_iteratee.call1(&JsValue::NULL, &js_value).unwrap();
            });
    }

    fn some(&self, filter_function: &js_sys::Function) -> bool {
        self.dataset().quads()
            .into_iter()
            .any(|quad| {
                let quad = quad.unwrap();
                let export_quad = SophiaExportQuad::new_from_quad(&quad);
                let js_value = JsValue::from(export_quad);
                filter_function.call1(&JsValue::NULL, &js_value).unwrap().is_truthy()
            })
    }

    fn every(&self, filter_function: &js_sys::Function) -> bool {
        self.dataset().quads()
            .into_iter()
            .all(|quad| {
                let quad = quad.unwrap();
                let export_quad = SophiaExportQuad::new_from_quad(&quad);
                let js_value = JsValue::from(export_quad);
                filter_function.call1(&JsValue::NULL, &js_value).unwrap().is_truthy()
            })
    }

    fn filter(&self, filter_function: &js_sys::Function) -> Self {
        let mut ds = D::default();

        self.dataset().quads()
            .filter_quads(|quad| {
                let export_quad = SophiaExportQuad::new_from_quad(&quad);
                let js_value = JsValue::from(export_quad);
                filter_function.call1(&JsValue::NULL, &js_value).unwrap().is_truthy()
            })
            .in_dataset(&mut ds)
            .unwrap();

        Self::wrap(ds)
    }

    fn reduce(&self, reducer: js_sys::Function, initial_value: &JsValue) -> JsValue {
        let mut iterator = self.dataset().quads();
        let mut accumulated_value = initial_value.clone();

        if accumulated_value.as_ref().is_undefined() {
            let first_iter = iterator.next();
            match first_iter {
                None => return accumulated_value,
                Some(quad_result) => {
                    let quad = quad_result.unwrap();
                    let export_quad = SophiaExportQuad::new_from_quad(&quad);

                    accumulated_value = JsValue::from(export_quad);
                }
            }
        }

        while let Some(quad_result) = iterator.next() {
            let quad = quad_result.unwrap();
            let export_quad = SophiaExportQuad::new_from_quad(&quad);
            let to_accumulate = JsValue::from(export_quad);
            accumulated_value = reducer.call2(&JsValue::NULL, &accumulated_value, &to_accumulate).unwrap();
        }

        accumulated_value
    }

    fn map(&self, map_function: &js_sys::Function) -> Self {
        let mut ds = Self::wrap(D::default());

        self.dataset().quads()
            .for_each_quad(|quad| {
                let export_quad = SophiaExportQuad::new_from_quad(&quad);
                let js_value = JsValue::from(export_quad);
                let mapped_js_quad = map_function.call1(&JsValue::NULL, &js_value).unwrap();
                let mapped_quad = JsImportQuad::from(mapped_js_quad);
                ds.add(&mapped_quad);
            })
            .unwrap();

        ds
    }

    fn to_string(&self) -> String {
        self.tonquads()
    }

    // Non RDF.JS exported functions

    fn quads(&self) -> js_sys::Array {
        self.dataset()
            .quads()
            .into_iter()
            .map(|quad| {
                let quad = quad.unwrap();
                SophiaExportQuad::new_from_quad(&quad)
            })
            .map(JsValue::from)
            .collect()
    }

    fn get_iterator(&self) -> RustExportIterator {
        RustExportIterator::new(self.quads())
    }

    // ==== Utility functions

    fn try_from<'a>(imported: &'a JsValue) -> Option<&'a Self> {
        // TODO : in trait, put method get_uniqueid which is exported and compared here

        let rust_managed = Reflect::get(imported, &JsValue::from_str("rust_managed"));
        if rust_managed.is_err() {
            return None;
        }

        let rust_managed = rust_managed.unwrap().as_f64();

        match rust_managed {
            None => None,
            Some(rust_managed) => unsafe {
                let ptr: *const Self = (rust_managed as u32) as *const Self;
                
                if !ptr.is_null() {
                    ptr.as_ref()
                } else {
                    None
                }
            }
        }
    }

    fn extract_dataset<'a>(imported: &'a JsValue) -> MaybeOwned<'a, Self> {
        let that = Self::try_from(imported);

        match that {
            Some(value) => MaybeOwned::Borrowed(value),
            None => {
                // TODO : there is probably a better dataset structure to just add quads and then iterate on
                let mut exported_dataset = Self::wrap( D::default() );
                
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

    ///
    /// 
    /// Used in contains and equals basic implementation
    fn contains_dataset<OD>(&self, other_dataset: &OD) -> bool
        where OD: Dataset {
        other_dataset.quads()
        .into_iter()
        .all(|element_result| {
            let element = element_result.unwrap();
            self.dataset().contains(
                element.s(),
                element.p(),
                element.o(),
                element.g()
            ).unwrap()
        })
    }


    // ====
    // For the wrapper approach

    /// Adds every quads from `nquads` (which is a N-Quad serialization of the quads to add)
    fn add_nquads(&mut self, nquads: &str) {
        let mut source = sophia::parser::nq::parse_str(nquads);
        source.in_dataset(self.mutable_dataset()).unwrap();
    }

    /// Adds every quads from `text` (which is a TriG serialization of the quads)
    fn add_trig(&mut self, text: &str) {
        let mut source = sophia::parser::trig::parse_str(text);
        source.in_dataset(self.mutable_dataset()).unwrap();
    }
    
    /// Returns a N-Quad serialization of the contained dataset
    fn tonquads(&self) -> String {
        let mut serializer = sophia::serializer::nq::NqSerializer::new_stringifier();
        serializer.serialize_dataset(self.dataset()).unwrap();
        serializer.to_string()
    }
}
