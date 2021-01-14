
use crate::{ Block, Identifier, Position };
use crate::{ MaybeTree4, LazyStructure };

use once_cell::unsync::OnceCell;
use std::collections::BTreeSet;

/// A (sometimes) tree of quads for which order is defined at compile time.
pub struct OnceTreeSet<I, A, B, C, D>
where I: Identifier, A: Position, B: Position, C: Position, D: Position 
{
    /// The underlying tree, if it is instancied
    v: OnceCell<BTreeSet<Block<I, A, B, C, D>>>,
}


impl<I, A, B, C, D> LazyStructure for OnceTreeSet<I, A, B, C, D>
where I: Identifier, A: Position, B: Position, C: Position, D: Position
{
    fn new() -> Self {
        Self { v: OnceCell::new() }
    }

    fn new_instanciated() -> Self {
        Self {
            v: {
                let x = OnceCell::<BTreeSet<Block<I, A, B, C, D>>>::new();
                x.set(BTreeSet::new()).ok();
                x
            }
        }
    }
}

// TODO: implements MaybeTree4



#[cfg(test)]
mod test {
    use crate::{Subject, Predicate, Object, Graph};
    use super::*;

    type SPOG = OnceTreeSet<u32, Subject, Predicate, Object, Graph>;
    type OGPS = OnceTreeSet<u32, Object, Graph, Predicate, Subject>;

    #[test]
    fn instanciation() {
        assert!(SPOG::new().v.get().is_none());
        assert!(OGPS::new().v.get().is_none());
        assert!(SPOG::new_instanciated().v.get().is_some());
        assert!(OGPS::new_instanciated().v.get().is_some());
    }



}