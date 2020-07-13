
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// Datamodel implementation
pub mod datamodel;

/// DatasetCore / Dataset implementations
pub mod dataset;

/// Examples of partial redefined implementations
pub mod wrappers_example;

/// Iterator that contains exported elements
pub mod exportiterator;

/// Convenience debug and log functions
pub mod util;

/// This modules exposes the details to export a Sophia Dataset to Web Assembly
pub mod wrapping;
