//! - As the implementation of [`OnceTreeSet`] have an order defined at compile
//! time, [`DynamicOnceTreeSet`] is also provided which enables to choose
//! which order to pick at execution time.

use crate::Identifier;
use crate::tree::{ MaybeTree4, Tree4Iterator, Forest4 };
use crate::order::{ Position, Subject, Predicate, Object, Graph };

mod _dynamic;
pub use self::_dynamic::*;

/// A forest of identifier trees. It is able to store arrays of four u32
pub struct IndexingForest4<I>
where I: Identifier {
    trees: Vec<DynamicOnceTreeSet<I>>
}

impl<I> Default for IndexingForest4<I>
where I: Identifier {
    fn default() -> Self {
        Self::new()
    }
}

impl<I> IndexingForest4<I>
where I: Identifier {
    /// Build an `IndexingForest4` with maximum indexing capacity (5 lazy indexes).
    pub fn new() -> Self {
        const S: usize = Subject::VALUE;
        const P: usize = Predicate::VALUE;
        const O: usize = Object::VALUE;
        const G: usize = Graph::VALUE;

        Self::new_with_indexes(
            &[[O, G, P, S]],
            &[
                [S, P, O, G],
                [G, P, S, O],
                [P, O, G, S],
                [G, S, P, O],
                [O, S, G, P]
            ],
        )
    }

    /// Build an `IndexingForest4` with a tree for each `default_initialize`
    /// order built from initialization and lazy trees for each
    /// `optional_indexes` order.
    pub fn new_with_indexes(
        default_initialized: &[[usize; 4]],
        will_be_initialized: &[[usize; 4]]
    ) -> Self {
        // TODO: check validity of indexes

        let mut retval = Self {
            trees : Vec::default()
        };

        assert!(!default_initialized.is_empty());

        for order in default_initialized {
            retval.trees.push(DynamicOnceTreeSet::<I>::new_instanciated(order).unwrap());
        }

        for order in will_be_initialized {
            retval.trees.push(DynamicOnceTreeSet::<I>::new(order).unwrap());
        }

        retval
    }

    fn best_tree_for<'a>(&'a self, pattern: &[Option<I>; 4]) -> &'a DynamicOnceTreeSet<I> {
        let mut best_tree: Option<(usize, usize)> = None;

        for (i, tree) in self.trees.iter().enumerate() {
            let opt_conformance = tree.index_conformance(true, &pattern);

            if let Some(conformance) = opt_conformance {
                if best_tree.is_none() || best_tree.unwrap().1 < conformance {
                    best_tree = Some((i, conformance));
                }
            }
        }

        // As the first tree should always exist, (see `new_with_indexes`), we can unwrap it
        &self.trees[best_tree.unwrap().0]
    }

    fn ensure_exists(my_tree: &DynamicOnceTreeSet<I>, reference_tree: &DynamicOnceTreeSet<I>) {
        if !my_tree.exists() {
            my_tree.ensure_exists(|| reference_tree.get_quads([None, None, None, None]));
        }
    }
}


impl<I> MaybeTree4<I> for IndexingForest4<I>
where I: Identifier
{
    fn exists(&self) -> bool {
        true
    }

    fn ensure_exists<'a, F>(&self, _f: F) where F: FnOnce() -> Tree4Iterator<'a, I> {
        // noop
    }

    fn get_quads<'a>(&'a self, pattern: [Option<I>; 4]) -> Tree4Iterator<'a, I> {
        let best_btree = self.best_tree_for(&pattern);
        Self::ensure_exists(best_btree, &self.trees[0]);
        best_btree.get_quads(pattern)
    }

    fn index_conformance(&self, can_build: bool, pattern_layout: &[Option<I>; 4]) -> Option<usize> {
        let mut res = None;

        for tree in &self.trees {
            let this_tree_conformance = tree.index_conformance(can_build, &pattern_layout);

            if this_tree_conformance.is_some() && (res.is_none() || res.unwrap() > this_tree_conformance.unwrap()) {
                res = this_tree_conformance;
            }
        }

        res
    }

    fn insert(&mut self, id_quad: &[I; 4]) -> Option<bool> {
        let mut opt = None;

        for tree in &mut self.trees {
            let x = tree.insert(&id_quad);

            if opt.is_none() {
                opt = x;
            }
        }

        opt
    }

    fn delete(&mut self, id_quad: &[I; 4]) -> Option<bool> {
        let mut opt = None;

        for tree in &mut self.trees {
            let x = tree.delete(&id_quad);
            
            if opt.is_none() {
                opt = x;
            }
        }

        opt
    }

    fn size(&self) -> Option<usize> {
        self.trees[0].size()
    }

    fn has(&self, id_quad: &[I; 4]) -> Option<bool> {
        self.trees[0].has(&id_quad)
    }
}

impl<I> Forest4<I> for IndexingForest4<I>
where I: Identifier {
    fn get_number_of_living_trees(&self) -> usize {
        self.trees
            .iter()
            .filter(|tree| tree.exists())
            .count()
    }

    fn ensure_has_index_for(&self, pattern: &[Option<I>; 4]) {
        let best_btree = self.best_tree_for(&pattern);
        Self::ensure_exists(best_btree, &self.trees[0]);
    }
}

#[cfg(test)]
mod test {
    use super::*;


    const S: usize = Subject::VALUE;
    const P: usize = Predicate::VALUE;
    const O: usize = Object::VALUE;
    const G: usize = Graph::VALUE;

    type IndexingForest4u32 = IndexingForest4<u32>;

    #[test]
    fn forest_instanciation() {
        let forest_new = IndexingForest4u32::new();
        assert!(forest_new.get_number_of_living_trees() >= 1);

        let forest_full = IndexingForest4u32::new_with_indexes(
            &[
                [O, G, P, S],
                [S, P, O, G],
                [G, P, S, O],
                [P, O, G, S],
                [G, S, P, O],
                [O, S, G, P]
            ],
            &[]
        );

        assert_eq!(forest_full.get_number_of_living_trees(), 6);
    }

    #[test]
    fn test_implem_() {
        type T = IndexingForest4u32;

        // Insertion
        {
            let mut tree = T::default();
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
            let mut tree = T::default();
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
            let mut tree = T::default();
            tree.insert(&[0_u32, 1_u32, 2_u32, 3_u32]);
            tree.insert(&[0_u32, 1_u32, 2_u32, 4_u32]);
            assert!(tree.has(&[0_u32, 1_u32, 2_u32, 3_u32]).unwrap());
            assert!(!tree.has(&[8_u32, 1_u32, 2_u32, 8_u32]).unwrap());
        }

        // Consistancy of new
        {
            let quad = [0_u32, 1_u32, 2_u32, 3_u32];

            let mut tree = T::default();
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

        // Filter
        {
            let mut tree = T::default();
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

            assert!(tree.get_number_of_living_trees() >= 2);
        }
    }

}