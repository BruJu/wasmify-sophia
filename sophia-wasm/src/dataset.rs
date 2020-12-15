//! This modules implements the DatasetCore interface from the RDF.JS
//! specification. The implementation also offers many services from the
//! Dataset class.

//#![deny(missing_docs)]

use crate::wasm_bindgen_dataset;
use crate::wasm_bindgen_wrappeddataset;
use crate::wrappers_example::{ TreeDatasetAntiWrapper, VecOrDatasetWrapper };

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use sophia::dataset::inmem::FastDataset;
use sophia::dataset::inmem::LightDataset;
use sophia::term::BoxTerm;

use bjdatasets::treeddataset::TreeDataset;
use bjdatasets::fulldataset::FullIndexDataset;


// Dataset structure created by the factory
wasm_bindgen_dataset!(TreeDataset, "TreeDataset", SophiaExportDataset);

// Other usable datasets
wasm_bindgen_dataset!(FastDataset, "FastDataset");
wasm_bindgen_dataset!(LightDataset, "LightDataset");
wasm_bindgen_dataset!(FullIndexDataset, "FullDataset");

// Array Dataset (which is not a real set)
type ArrayDataset = Vec<([BoxTerm; 3], Option<BoxTerm>)>;
wasm_bindgen_dataset!(ArrayDataset, "ArrayDataset");

// A dataset that redefines the match method
wasm_bindgen_wrappeddataset!(TreeDatasetAntiWrapper, "AntiTreeDataset");

// Datasets that fills an array instead of the base complicated structure
type TreeDatasetIntoArrayWrapper = VecOrDatasetWrapper<TreeDataset>;
type FastDatasetIntoArrayWrapper = VecOrDatasetWrapper<FastDataset>;
type LightDatasetIntoArrayWrapper = VecOrDatasetWrapper<LightDataset>;
type FullIndexDatasetIntoArrayWrapper = VecOrDatasetWrapper<FullIndexDataset>;

wasm_bindgen_wrappeddataset!(TreeDatasetIntoArrayWrapper, "TreeDatasetToA");
wasm_bindgen_wrappeddataset!(FastDatasetIntoArrayWrapper, "FastDatasetToA");
wasm_bindgen_wrappeddataset!(LightDatasetIntoArrayWrapper, "LightDatasetToA");
wasm_bindgen_wrappeddataset!(FullIndexDatasetIntoArrayWrapper, "FullDatasetToA");
