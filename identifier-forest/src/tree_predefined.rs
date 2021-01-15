
use crate::order::{ Block, Position};
use crate::{ Identifier };
use crate::{ Tree4Iterator, MaybeTree4, LazyStructure };
use crate::order::FixedOrder4;
use crate::order::pattern_match;

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

impl<I, A, B, C, D> MaybeTree4<I> for OnceTreeSet<I, A, B, C, D>
where I: Identifier, A: Position, B: Position, C: Position, D: Position
{
    fn exists(&self) -> bool {
        self.v.get().is_some()
    }

    fn ensure_exists<'a, F>(&self, f: F) where F: FnOnce() -> Tree4Iterator<'a, I> {
        self.v.get_or_init(move || {
            let mut tree = BTreeSet::<Block<I, A, B, C, D>>::new();

            for element in f() {
                tree.insert(Block::<I, A, B, C, D>::new(element));
            }

            tree
        });
    }

    fn get_quads<'a>(&'a self, pattern: [Option<I>; 4]) -> Tree4Iterator<'a, I> {
        let my_tree = self.v.get().unwrap();

        let (range, filter) = FixedOrder4::<A, B, C, D>::range(pattern);

        Box::new(
            my_tree
                .range(range)
                .filter(move |&elem| pattern_match(&elem.values, &filter))
                .map(|elem| elem.values)
        )
    }

    fn index_conformance(&self, can_build: bool, pattern_layout: &[Option<I>; 4]) -> Option<usize> {
        if !can_build && self.v.get().is_none() {
            None
        } else {
            Some(FixedOrder4::<A, B, C, D>::index_conformance(pattern_layout))
        }
    }

    fn insert(&mut self, id_quad: &[I; 4]) -> Option<bool> {
        let opt_tree = self.v.get_mut();
        match opt_tree {
            None => None,
            Some(tree) => Some(tree.insert(Block::<I, A, B, C, D>::new(*id_quad)))
        }
    }

    fn delete(&mut self, id_quad: &[I; 4]) -> Option<bool> {
        let opt_tree = self.v.get_mut();
        match opt_tree {
            None => None,
            Some(tree) => Some(tree.remove(&Block::<I, A, B, C, D>::new(*id_quad)))
        }
    }

    fn size(&self) -> Option<usize> {
        let opt_tree = self.v.get();
        match opt_tree {
            None => None,
            Some(tree) => Some(tree.len())
        }
    }

    fn has(&self, id_quad: &[I; 4]) -> Option<bool> {
        let opt_tree = self.v.get();
        match opt_tree {
            None => None,
            Some(tree) => Some(tree.contains(&Block::<I, A, B, C, D>::new(*id_quad)))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::order::{Subject, Predicate, Object, Graph};
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

    fn test_implem_<T>()
    where T: MaybeTree4<u32> + LazyStructure {
        // Testing the fact that the implementation does something
        let _not_instanciated = T::new();

        // Testing existance of a newly instanciated structure
        {
            let instanciated = T::new_instanciated();
            assert!(instanciated.exists());
            assert_eq!(instanciated.size(), Some(0_usize));
        }

        // Testing populating with an empty iterator
        {
            let mut tree = T::new();
            tree.ensure_exists(|| Box::new(std::iter::empty()));
            assert!(tree.exists());
            assert_eq!(tree.size(), Some(0_usize));
        }

        // Insertion
        {
            let mut tree = T::new_instanciated();
            assert_eq!(tree.size(), Some(0_usize));
            tree.insert(&[0_u32, 1_u32, 2_u32, 3_u32]);
            assert_eq!(tree.size(), Some(1_usize));
            tree.insert(&[0_u32, 1_u32, 2_u32, 3_u32]);
            assert_eq!(tree.size(), Some(1_usize), "Duplicates should not be stored");
            tree.insert(&[0_u32, 1_u32, 2_u32, 4_u32]);
            assert_eq!(tree.size(), Some(2_usize));
        }

        // Deletion
        {
            let mut tree = T::new_instanciated();
            tree.insert(&[0_u32, 1_u32, 2_u32, 3_u32]);
            tree.insert(&[0_u32, 1_u32, 2_u32, 4_u32]);
            assert_eq!(tree.size(), Some(2_usize));
            assert!(tree.delete(&[0_u32, 1_u32, 2_u32, 4_u32]).unwrap(), "Should return true if the quad was present");
            assert_eq!(tree.size(), Some(1_usize));
            assert!(!tree.delete(&[0_u32, 1_u32, 2_u32, 4_u32]).unwrap(), "Should return true if the quad was not present");
            assert_eq!(tree.size(), Some(1_usize));
        }

        // Has
        {
            let mut tree = T::new_instanciated();
            tree.insert(&[0_u32, 1_u32, 2_u32, 3_u32]);
            tree.insert(&[0_u32, 1_u32, 2_u32, 4_u32]);
            assert!(tree.has(&[0_u32, 1_u32, 2_u32, 3_u32]).unwrap());
            assert!(!tree.has(&[8_u32, 1_u32, 2_u32, 8_u32]).unwrap());
        }

        // Consistancy of new
        {
            let quad = [0_u32, 1_u32, 2_u32, 3_u32];

            let mut tree = T::new();
            if tree.exists() {
                assert!(tree.size().is_some());
                assert!(tree.insert(&quad).is_some());
                assert!(tree.has(&quad).is_some());
                assert!(tree.delete(&quad).is_some());
                assert!(tree.index_conformance(true, &[None, None, None, None]).is_some());
            } else {
                assert!(tree.size().is_none());
                assert!(tree.insert(&quad).is_none());
                assert!(tree.has(&quad).is_none());
                assert!(tree.delete(&quad).is_none());
                assert!(tree.index_conformance(false, &[None, None, None, None]).is_none());
            }
        }

        // Iteration
        {
            let mut original_data = BTreeSet::new();
            original_data.insert([10_u32, 20_u32, 30_u32, 40_u32]);
            original_data.insert([10_u32, 21_u32, 30_u32, 40_u32]);
            original_data.insert([10_u32, 20_u32, 31_u32, 40_u32]);
            original_data.insert([10_u32, 20_u32, 30_u32, 41_u32]);
            original_data.insert([11_u32, 20_u32, 30_u32, 40_u32]);
            original_data.insert([11_u32, 21_u32, 30_u32, 40_u32]);
            original_data.insert([11_u32, 20_u32, 31_u32, 40_u32]);
            original_data.insert([11_u32, 20_u32, 30_u32, 41_u32]);

            let mut tree = T::new();
            tree.ensure_exists(|| Box::new(original_data.iter().map(|&e| e)));

            assert_eq!(tree.size(), Some(original_data.len()));

            let mut extracted_data = BTreeSet::new();
            for quad in tree.get_quads([None, None, None, None]) {
                extracted_data.insert(quad);
            }

            assert_eq!(extracted_data, original_data);
        }

        // Filter
        {
            let mut tree = T::new_instanciated();
            tree.insert(&[10_u32, 20_u32, 30_u32, 40_u32]);
            tree.insert(&[10_u32, 21_u32, 30_u32, 40_u32]);
            tree.insert(&[10_u32, 20_u32, 31_u32, 40_u32]);
            tree.insert(&[10_u32, 20_u32, 30_u32, 41_u32]);
            tree.insert(&[11_u32, 20_u32, 30_u32, 40_u32]);
            tree.insert(&[11_u32, 21_u32, 30_u32, 40_u32]);
            tree.insert(&[11_u32, 20_u32, 31_u32, 40_u32]);
            tree.insert(&[11_u32, 20_u32, 30_u32, 41_u32]);
            tree.insert(&[11_u32, 20_u32, 30_u32, 42_u32]);

            assert_eq!(tree.get_quads([None, None, None, None]).count(), 9);
            assert_eq!(tree.get_quads([Some(10_u32), None, None, None]).count(), 4);
            assert_eq!(tree.get_quads([Some(11_u32), None, None, None]).count(), 5);
            assert_eq!(tree.get_quads([Some(77_u32), None, None, None]).count(), 0);
            assert_eq!(tree.get_quads([None, Some(20_u32), None, None]).count(), 7);
            assert_eq!(tree.get_quads([None, Some(20_u32), None, Some(41_u32)]).count(), 2);
        }
    }


    #[test]
    fn test_impl() {
        test_implem_::<OnceTreeSet<u32, Subject, Predicate, Object, Graph>>();
        test_implem_::<OnceTreeSet<u32, Object, Graph, Predicate, Subject>>();
    }


}