
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

pub mod btreeddataset_anti;

pub mod dataset_exportableds;
pub mod dataset_exportableconcrete;
pub mod dataset_exportablemacro;

pub mod dataset_into_vector_wrapper;
