pub mod treedstructure;

#[cfg(feature = "impl_sophia")]
pub mod fulldataset;

#[cfg(feature = "impl_sophia")]
pub mod vecordataset;

#[cfg(feature = "impl_sophia")]
pub mod treeddataset;

#[cfg(feature = "impl_sophia")]
mod rcquad;

#[cfg(feature = "impl_sophia")]
pub use rcquad::RcQuad;
