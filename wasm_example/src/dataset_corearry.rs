//! This file implements the DatarrysetCore interface from the RDF.JS
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
use sophia::term::*;
use sophia::quad::Quad;
use sophia::quad::stream::QuadSource;

use wasm_bindgen::prelude::*;

use crate::arrydataset::ArryDataset;


#[macro_use]
use crate::export_sophia_dataset;

export_sophia_dataset!(SophiaExportDatarryset, JsImportDatarryset, "Datarry", ArryDataset);
