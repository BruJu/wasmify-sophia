
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// TODO : find how to use folder instead of using _

// Data Model
pub mod datamodel_term;
pub mod datamodel_quad;
pub mod datamodel_factory;

// Dataset Core
pub mod exportiterator;
pub mod dataset_core;

// Debug / Log
pub mod util;

pub mod btreeddataset;
pub mod fulldataset;

pub mod arrydataset;
pub mod dataset_macro;

pub mod dataset_exportableds;
pub mod dataset_exportableconcrete;
pub mod dataset_exportablemacro;

