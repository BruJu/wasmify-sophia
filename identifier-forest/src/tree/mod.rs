//! This module defines the features offered by a structure that stores arrays
//! of fours identifiers.
//!
//! It provides:
//! - A trait, [`MaybeTree4`] which enables to insert, remove, ... quads.
//! The specifity of this trait is that it accepts implementation that can
//! produce no result, and also a method that forces to populate the tree
//! - Implementations using both OnceCell and BTreeSet to produce BTreeSet
//! based implementation that can be populated while still using a const
//! reference. The main purpose is to provide duplicates version of the same
//! list of quads, and to pick the most efficient order when a certain
//! quad pattern is requested
//! - As the implementation of [`OnceTreeSet`] have an order defined at compile
//! time, [`DynamicOnceTreeSet`] is also provided which enables to choose
//! which order to pick at execution time.


mod tree_trait;
mod tree_enum;
mod tree_predefined;

pub use self::tree_trait::*;
pub use self::tree_enum::*;
pub use self::tree_predefined::*;
