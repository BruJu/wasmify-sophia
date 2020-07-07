
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
            // use crate::dataset_exportableds::ExportableDataset;

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
                    // ExportableDataset::match_quad(&self.base, subject, predicate, object, graph)
                    // or
                    // self.base.match_quad(subject, predicate, object, graph)
                }
            }
        }
        


    };
}

type BLAHFastDataset = crate::dataset_exportableds::ExportableDatasetBase<FastDataset>;

export_sophia_ds!(BLAHFastDataset, BLOUFastDataset, "FFastDataset");

