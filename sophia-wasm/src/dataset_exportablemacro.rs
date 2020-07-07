
use sophia::dataset::inmem::FastDataset;
use wasm_bindgen::prelude::*;

// This file provides a macro to export Sophia's dataset in Javascript


#[macro_export]
macro_rules! export_sophia_ds {
    ($base_class: ident, $rust_export_name: ident, $js_name: expr) => {

        #[wasm_bindgen(js_name=$js_name)]
        pub struct $rust_export_name {
            base: $base_class
        }
        
        #[wasm_bindgen(js_class=$js_name)]
        impl $rust_export_name {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self { base: $base_class::default() }
            }

            #[wasm_bindgen(js_name="match")]
            pub fn match_quad(&self,
                subject: &crate::datamodel_term::JsImportTerm,
                predicate: &crate::datamodel_term::JsImportTerm,
                object: &crate::datamodel_term::JsImportTerm,
                graph: &crate::datamodel_term::JsImportTerm) -> Self {
                Self { base: 
                    crate::dataset_exportableds::ExportableDataset::match_quad(&self.base, subject, predicate, object, graph)
                }
            }

            #[wasm_bindgen]
            pub fn quads(&self) -> js_sys::Array {
                crate::dataset_exportableds::ExportableDataset::quads(&self.base)
            }

            #[wasm_bindgen(getter)]
            pub fn size(&self) -> usize {
                crate::dataset_exportableds::ExportableDataset::size(&self.base)
            }

            #[wasm_bindgen(js_name="getIterator")]
            pub fn get_iterator(&self) -> crate::exportiterator::RustExportIterator {
                crate::dataset_exportableds::ExportableDataset::get_iterator(&self.base)
            }

            #[wasm_bindgen]
            pub fn add(&mut self, quad: &crate::datamodel_quad::JsImportQuad) {
                crate::dataset_exportableds::ExportableDataset::add(&mut self.base, quad);
            }

            #[wasm_bindgen]
            pub fn delete(&mut self, quad: &crate::datamodel_quad::JsImportQuad) {
                crate::dataset_exportableds::ExportableDataset::delete(&mut self.base, quad);
            }

            #[wasm_bindgen]
            pub fn has(&self, quad: &crate::datamodel_quad::JsImportQuad) -> bool {
                crate::dataset_exportableds::ExportableDataset::has_quad(&self.base, quad)
            }

            #[wasm_bindgen(js_name="addAll")]
            pub fn add_all(&mut self, quads_as_jsvalue: &JsValue) {
                crate::dataset_exportableds::ExportableDataset::add_all(&mut self.base, quads_as_jsvalue);
            }

        }

        #[wasm_bindgen(js_class=$js_name)]
        impl $rust_export_name {
            pub fn contains(&self, imported: &JsValue) -> bool {
                crate::dataset_exportableds::ExportableDataset::contains(&self.base, imported)
            }

            pub fn delete_matches(&mut self,
                subject: &crate::datamodel_term::JsImportTerm,
                predicate: &crate::datamodel_term::JsImportTerm,
                object: &crate::datamodel_term::JsImportTerm,
                graph: &crate::datamodel_term::JsImportTerm) {
                crate::dataset_exportableds::ExportableDataset::delete_matches(&mut self.base, subject, predicate, object, graph);
            }

            pub fn difference(&self, imported: &JsValue) -> Self {
                Self { base: crate::dataset_exportableds::ExportableDataset::difference(&self.base, imported) }
            }

            pub fn intersection(&self, imported: &JsValue) -> Self {
                Self { base: crate::dataset_exportableds::ExportableDataset::intersection(&self.base, imported) }
            }

            pub fn union(&self, imported: &JsValue) -> Self {
                Self { base: crate::dataset_exportableds::ExportableDataset::union(&self.base, imported) }
            }

            pub fn equals(&self, imported: &JsValue) -> bool {
                crate::dataset_exportableds::ExportableDataset::equals(&self.base, imported)
            }

        }

        #[wasm_bindgen(js_class=$js_name)]
        impl $rust_export_name {
            #[wasm_bindgen(js_name="forEach")]
            pub fn for_each(&self, quad_run_iteratee: &js_sys::Function) {
                crate::dataset_exportableds::ExportableDataset::for_each(&self.base, quad_run_iteratee)
            }

            #[wasm_bindgen]
            pub fn some(&self, filter_function: &js_sys::Function) -> bool {
                crate::dataset_exportableds::ExportableDataset::some(&self.base, filter_function)
            }

            #[wasm_bindgen]
            pub fn every(&self, filter_function: &js_sys::Function) -> bool {
                crate::dataset_exportableds::ExportableDataset::every(&self.base, filter_function)
            }

            #[wasm_bindgen]
            pub fn filter(&self, filter_function: &js_sys::Function) -> Self {
                Self { base: crate::dataset_exportableds::ExportableDataset::filter(&self.base, filter_function) }
            }

            #[wasm_bindgen]
            pub fn map(&self, map_function: &js_sys::Function) -> Self {
                Self { base: crate::dataset_exportableds::ExportableDataset::map(&self.base, map_function) }
            }

            #[wasm_bindgen(js_name="toArray")]
            pub fn to_array(&self) -> js_sys::Array {
                crate::dataset_exportableds::ExportableDataset::quads(&self.base)
            }

            #[wasm_bindgen(js_name="toString")]
            pub fn to_string(&self) -> String {
                crate::dataset_exportableds::ExportableDataset::to_string(&self.base)
            }
        }

        impl $rust_export_name {
            pub fn reduce(&self, reducer: js_sys::Function, initial_value: &JsValue) -> JsValue {
                crate::dataset_exportableds::ExportableDataset::reduce(&self.base, reducer, initial_value)
            }
        }
    };
}

type BLAHFastDataset = crate::dataset_exportableds::ExportableDatasetBase<FastDataset>;

//export_sophia_ds!(BLAHFastDataset, SophiaExportDataset, "FFastDataset");

export_sophia_ds!(BLAHFastDataset, BLOUFastDataset, "FFastDataset");
