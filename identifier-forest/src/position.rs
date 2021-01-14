//! Define the trait [`Position`]. 

/// A Position is an index in an array of identifiers.
///
/// This trait is mainly used as a workaround as const integers in generics are
/// not supported in Rust. It also provides added semantic on the indexes.
/// 
/// The semantic is never actually enforced by the implementation, and each
/// position implementation with the same VALUE can be used interchangeably.
pub trait Position {
    /// Index of this Position in an array of identifiers
    const VALUE: usize;
    /// Name of the position. Provided for debugging purpose.
    const NAME: &'static str;
}


// ----------------------------------------------------------------------------
// ---- Zero abstraction
/*

// We do not actually provide these implementation as the trees and forests
// actually use the RDF related ones.
//
// For users that do not know RDF, and do not need to know about RDF details,
// Subject can be used as IndexZero, Predicate as IndexOne...

/// The zero-th element
pub struct IndexZero {}
impl Position for IndexZero {
    const VALUE: usize = 0;
    const NAME: &'static str = "[0]";
}

/// The first element
pub struct IndexOne {}
impl Position for IndexOne {
    const VALUE: usize = 1;
    const NAME: &'static str = "[1]";
}

/// The second element
pub struct IndexTwo {}
impl Position for IndexTwo {
    const VALUE: usize = 2;
    const NAME: &'static str = "[2]";
}

/// The third element
pub struct IndexThree {}
impl Position for IndexThree {
    const VALUE: usize = 3;
    const NAME: &'static str = "[3]";
}

*/


// ----------------------------------------------------------------------------
// ---- Array of identifiers seen as a SPOG ordered RDF triple or quad.

/// The subject position in a SPO RDF triple / SPOG RDF quad
pub struct Subject {}
impl Position for Subject {
    const VALUE: usize = 0;
    const NAME: &'static str = "Subject";
}

/// The predicate position in a SPO RDF triple / SPOG RDF quad
pub struct Predicate {}
impl Position for Predicate {
    const VALUE: usize = 1;
    const NAME: &'static str = "Predicate";
}

/// The object position in a SPO RDF triple / SPOG RDF quad
pub struct Object {}
impl Position for Object {
    const VALUE: usize = 2;
    const NAME: &'static str = "Object";
}

/// The graph position in a SPOG RDF quad
pub struct Graph {}
impl Position for Graph {
    const VALUE: usize = 3;
    const NAME: &'static str = "Graph";
}
