extern crate wasm_bindgen;

// TODO : find how to use folder instead of using _

// Data Model
pub mod datamodel_term;
pub mod datamodel_quad;
pub mod datamodel_factory;

// Dataset Core
pub mod matchableterm;
pub mod exportiterator;
pub mod dataset_core;

// Debug / Log
pub mod util;
pub mod parser;

// Required by factory
use crate::dataset_core::*;

