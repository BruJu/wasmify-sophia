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
use sophia::term::*;
use sophia::quad::Quad;
use sophia::quad::stream::QuadSource;

use wasm_bindgen::prelude::*;

use crate::fulldataset::FullIndexDataset;

use crate::export_sophia_dataset;

export_sophia_dataset!(SophiaExportFastDataset, JsImportFastDataset, "FastDatasetcore", FastDataset);

export_sophia_dataset!(SophiaExportDataset, JsImportDataset, "Datasetcore", FullIndexDataset);
