//! This module exports some wrappers of datasets that have some methods
//! redefined

mod btreeddataset_anti;
mod dataset_into_vector_wrapper;

pub use btreeddataset_anti::BTreedDatasetAntiWrapper;
pub use dataset_into_vector_wrapper::VecOrDatasetWrapper;
