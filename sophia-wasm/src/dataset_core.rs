//! This file implements the DatasetCore interface from the RDF.JS
//! specification. The implementation also offers many services from the
//! Dataset class.

#![deny(missing_docs)]

extern crate wasm_bindgen;

use crate::datamodel_term::*;
use crate::datamodel_quad::*;
use crate::datamodel_factory::*;
use crate::exportiterator::RustExportIterator;
use crate::util::*;

use maybe_owned::MaybeOwned;
use sophia::dataset::Dataset;
use sophia::dataset::MutableDataset;
use sophia::dataset::inmem::FastDataset;
use sophia::dataset::inmem::LightDataset;
use sophia::term::*;
use sophia::quad::Quad;
use sophia::quad::stream::QuadSource;

use wasm_bindgen::prelude::*;

use crate::btreeddataset::TreedDataset;
use crate::export_sophia_dataset;
use crate::fulldataset::FullIndexDataset;

use crate::arrydataset::ArryDataset;

//use crate::sortabledataset::SortableDataset;

export_sophia_dataset!(SophiaExportTDataset, JsImportDataset, "Datasetcore", TreedDataset);


export_sophia_dataset!(SophiaExportTreeDataset, JsImportTreeDataset, "TreeDataset", TreedDataset,
    SophiaExportTreeDataset, TreedDataset,
    |that: &mut SophiaExportTreeDataset, m: crate::dataset_macro::MatchRequestOnRcTerm| {
        let s_is_some = match &m.s { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        let p_is_some = match &m.p { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        let o_is_some = match &m.o { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        let g_is_some = match &m.g { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        
        let mut quads_iter = that.dataset.quads_matching(&m.s, &m.p, &m.o, &m.g);

        let mut dataset = TreedDataset::new_anti(s_is_some, p_is_some, o_is_some, g_is_some);
        quads_iter.in_dataset(&mut dataset).unwrap();

        dataset
    }
);

export_sophia_dataset!(SophiaExportFastDataset, JsImportFastDataset, "FastDataset", FastDataset);
export_sophia_dataset!(SophiaExportLightDataset, JsImportLightDataset, "LightDataset", LightDataset);
export_sophia_dataset!(SophiaExportFullDataset, JsImportFullDataset, "FullDataset", FullIndexDataset);
export_sophia_dataset!(SophiaExportArrayDataset, JsImportArrayDataset, "ArrayDataset", ArryDataset);

export_sophia_dataset!(SophiaExportTreeDataset2, JsImportTreeDataset2, "TreeDatasetToA", TreedDataset, SophiaExportArrayDataset, ArryDataset);
export_sophia_dataset!(SophiaExportFastDataset2, JsImportFastDataset2, "FastDatasetToA", FastDataset, SophiaExportArrayDataset, ArryDataset);
export_sophia_dataset!(SophiaExportLightDataset2, JsImportLightDataset2, "LightDatasetToA", LightDataset, SophiaExportArrayDataset, ArryDataset);
export_sophia_dataset!(SophiaExportFullDataset2, JsImportFullDataset2, "FullDatasetToA", FullIndexDataset, SophiaExportArrayDataset, ArryDataset);

/*
export_sophia_dataset!(SophiaExportSDataset, JsImportSDataset, "SDataset", SortableDataset,
    SophiaExportSDataset, SortableDataset,

    |that: &mut SophiaExportSDataset, m: crate::dataset_macro::MatchRequestOnRcTerm| {
        let s_is_some = match &m.s { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        let p_is_some = match &m.p { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        let o_is_some = match &m.o { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        let g_is_some = match &m.g { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };

        that.dataset.sort_for(&[s_is_some, p_is_some, o_is_some, g_is_some]);
        
        let mut quads_iter = that.dataset.quads_matching(&m.s, &m.p, &m.o, &m.g);

        let mut dataset = SortableDataset::new();
        quads_iter.in_dataset(&mut dataset).unwrap();

        dataset
    }
);
*/



