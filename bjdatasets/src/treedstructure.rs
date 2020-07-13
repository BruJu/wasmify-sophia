﻿use once_cell::unsync::OnceCell;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TermRole {
    Subject = 0,
    Predicate = 1,
    Object = 2,
    Graph = 3,
}

/// A block is a structure that can be stored in a BTreeSet to store quads in
/// a certain order
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub struct Block<T> {
    data: [T; 4],
}

impl <T> Block<T> where T: Clone {
    /// Creates a block with the given values
    pub fn new(values: [T; 4]) -> Block<T> {
        Block { data: values }
    }
}

impl <T> Block<T> where T: Clone + PartialEq {
    /// Returns true if the non None values of the given filter_block are equals
    /// to the values of this block
    pub fn match_option_block(&self, filter_block: &Block<Option<T>>) -> bool {
        for i in 0..filter_block.data.len() {
            if let Some(filter_data) = filter_block.data[i].as_ref() {
                if self.data[i] != *filter_data {
                    return false;
                }
            }
        }

        true
    }
}

/// A block order enables to convert a SPOG quad into a block and get back
/// the SPOG quad.
/// 
/// It provides methods to manipulate the elements of a `BTreeSet<Block>`
/// by using functions that takes as input or returns an array of four u32
/// representing the quad indexes
#[derive(Clone, Copy)]
pub struct BlockOrder {
    term_roles: [TermRole; 4],
    to_block_index_to_destination: [usize; 4],
    to_indices_index_to_destination: [usize; 4]
}

impl BlockOrder {
    /// Returns a string that represents the block order
    pub fn name(&self) -> String {
        format!(
            "{:?} {:?} {:?} {:?}",
            self.term_roles[0],
            self.term_roles[1],
            self.term_roles[2],
            self.term_roles[3]
        )
    }

    pub fn get_term_roles(&self) -> &[TermRole; 4] {
        &self.term_roles
    }

    /// Builds a block builder from an order of SPOG
    pub fn new(term_roles: [TermRole; 4]) -> BlockOrder {
        debug_assert!({
            let mut present = [false; 4];
            for tr in term_roles.iter() {
                present[*tr as usize] = true;
            }
            present.iter().all(|x| *x)
        });
        let mut to_block_index_to_destination = [0; 4];
        let mut to_indices_index_to_destination = [0; 4];

        for (position, term_role) in term_roles.iter().enumerate() {
            to_indices_index_to_destination[*term_role as usize] = position;
            to_block_index_to_destination[position] = *term_role as usize;
        }
        
        BlockOrder { term_roles, to_block_index_to_destination, to_indices_index_to_destination }
    }

    /// Builds a block from SPOG indices
    pub fn to_block<T>(&self, indices: &[T; 4]) -> Block<T> where T: Copy {
        Block{
            data: [
                indices[self.to_block_index_to_destination[0]],
                indices[self.to_block_index_to_destination[1]],
                indices[self.to_block_index_to_destination[2]],
                indices[self.to_block_index_to_destination[3]]
            ]
        }
    }

    /// Builds a block from SPOG indices
    pub fn to_filter_block<T>(&self, indices: &[Option<T>; 4]) -> Block<Option<T>> where T: Copy + PartialEq {
        Block{
            data: [
                indices[self.to_block_index_to_destination[0]],
                indices[self.to_block_index_to_destination[1]],
                indices[self.to_block_index_to_destination[2]],
                indices[self.to_block_index_to_destination[3]]
            ]
        }
    }

    /// Buids SPOG indices from a block
    pub fn to_indices<T>(&self, block: &Block<T>) -> [T; 4] where T: Copy {
        return [
            block.data[self.to_indices_index_to_destination[0]],
            block.data[self.to_indices_index_to_destination[1]],
            block.data[self.to_indices_index_to_destination[2]],
            block.data[self.to_indices_index_to_destination[3]],
        ]
    }

    /// Returns the number of term kinds in the array request_terms that can be
    /// used as a prefix
    pub fn index_conformance(&self, request: &[&Option<u32>; 4]) -> usize {
        self.term_roles
            .iter()
            .take_while(|tr| request[**tr as usize].is_some())
            .count()
    }

    /// Returns a range on every block that matches the given spog. The range
    /// is restricted as much as possible. Returned indexes are the spog indexes
    /// that are not strictly filtered by the range (other spog that do not
    /// match can be returned)
    pub fn range(&self, spog: [Option<u32>; 4]) -> (std::ops::RangeInclusive<Block<u32>>, Block<Option<u32>>) {
        // Restrict range as much as possible
        let mut min = [u32::min_value(); 4];
        let mut max = [u32::max_value(); 4];

        for (i, term_role) in self.term_roles.iter().enumerate() {
            match spog[*term_role as usize] {
                None => { break; }
                Some(set_value) => {
                    min[i] = set_value;
                    max[i] = set_value;
                }
            }
        }

	    // Return range + filter block
	    (Block::new(min)..=Block::new(max), self.to_filter_block(&spog))
    }

    /// Inserts the given quad in the passed tree, using this quad ordering
    /// 
    /// Returns true if the quad was already present
    pub fn insert_into(&self, tree: &mut BTreeSet<Block<u32>>, spog: &[u32; 4]) -> bool {
        let block = self.to_block(spog);
        !tree.insert(block)
    }

    /// Deletes the given quad from the passed tree, using this quad ordering
    /// 
    /// Returns true if the quad has been deleted
    pub fn delete_from(&self, tree: &mut BTreeSet<Block<u32>>, spog: &[u32; 4]) -> bool {
        let block = self.to_block(spog);
        tree.remove(&block)
    }

    /// Returns true if the passed tree contains the passed quad
    pub fn contains(&self, tree: &BTreeSet<Block<u32>>, spog: &[u32; 4]) -> bool {
        let block = self.to_block(spog);
        tree.contains(&block)
    }

    /// Inserts every quads in iterator in the passed tree
    pub fn insert_all_into<'a>(&self, tree: &mut BTreeSet<Block<u32>>, iterator: FilteredIndexQuads<'a>) {
        for block in iterator.map(|spog| self.to_block(&spog)) {
            tree.insert(block);
        }
    }

    /// Returns an iterator on every quads that matches the given filter.
    /// 
    /// The filter in an array of four optional quad indexes, None means every
    /// quad must be matched, a given value on a term position that only quads
    /// that have the specified value have to be returned.
    /// 
    /// The filtering tries to be smart by iterating on the less possible number
    /// of quads in the tree. For several trees, the result of
    /// `index_conformance` indicates how many quads will be iterated on : for
    /// two block order, the block order that returns the greater
    /// `index_conformance` will return an iterator that looks over less
    /// different quads.
    pub fn filter<'a>(&'a self, tree: &'a BTreeSet<Block<u32>>, spog: [Option<u32>; 4]) -> FilteredIndexQuads {
        let (range, term_filter) = self.range(spog);
        let tree_range = tree.range(range);

        FilteredIndexQuads {
            range: tree_range,
            block_order: self,
            term_filter: term_filter
        }
    }
    
    /// ???
    pub fn build_new_tree_by_filtering(&self, source: &BTreeSet<Block<u32>>, spog: &[Option<u32>; 4]) -> BTreeSet<Block<u32>> {
        let filter_block = self.to_filter_block(spog);

        source.into_iter()
            .filter(|block| !block.match_option_block(&filter_block))
            .map(|b| b.clone())
            .collect()
    }
}

/// An iterator on a sub tree
pub struct FilteredIndexQuads<'a> {
    /// Iterator
    range: std::collections::btree_set::Range<'a, Block<u32>>,
    /// Used block order to retrived SPOG quad indexes
    block_order: &'a BlockOrder,
    /// Term filter for quads that can't be restricted by the range
    term_filter: Block<Option<u32>>
}

impl<'a> Iterator for FilteredIndexQuads<'a> {
    type Item = [u32; 4];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.range.next();

            match next.as_ref() {
                None => { return None; },
                Some(block) => {
                    if block.match_option_block(&self.term_filter) {
                        return Some(self.block_order.to_indices(block));
                    }
                }
            }
        }
    }
}

/// A treed dataset is a forest of trees that implements the Dataset trait
/// of Sophia.
/// 
/// It is composed of several trees, with a main tree and several optional
/// subtrees.
/// 
/// 
/// The trees are sorted in different orders, so for each (S?, P?, O?, G?)
/// combinaison (when each S, P ... can be specified or not), we have an
/// efficient way to find every quad that matches the query.
/// 
/// Up to 6 different trees are built, with the OGPS tree being built by
/// default
pub struct IndexTrees {
    /// The tree that is always instancied
    pub base_tree: (BlockOrder, BTreeSet<Block<u32>>),
    /// A list of optional trees that can be instancied ot improve look up
    /// performances at the cost of further insert and deletions
    pub optional_trees: Vec<(BlockOrder, OnceCell<BTreeSet<Block<u32>>>)>,
}

impl Default for IndexTrees {
    fn default() -> Self {
        IndexTrees::new_with_indexes(
            &vec!([TermRole::Object, TermRole::Graph, TermRole::Predicate, TermRole::Subject]),
            Some(&vec!(
                [TermRole::Subject, TermRole::Predicate, TermRole::Object, TermRole::Graph],
                [TermRole::Graph, TermRole::Predicate, TermRole::Subject, TermRole::Object],
                [TermRole::Predicate, TermRole::Object, TermRole::Graph, TermRole::Subject],
                [TermRole::Graph, TermRole::Subject, TermRole::Predicate, TermRole::Object],
                [TermRole::Object, TermRole::Subject, TermRole::Graph, TermRole::Predicate]
            ))
        )
    }
}

impl IndexTrees {
    pub fn new_with_indexes(default_initialized: &Vec<[TermRole; 4]>, optional_indexes: Option<&Vec<[TermRole; 4]>>) -> Self {
        assert!(!default_initialized.is_empty());

        // Base tree
        let base_tree = (
            BlockOrder::new(default_initialized[0]),
            BTreeSet::new()
        );

        // Redundant trees
        let mut optional_trees = Vec::new();

        // Default initialized
        for i in 1..default_initialized.len() {
            let cell = OnceCell::new();
            let set_result = cell.set(BTreeSet::new());
            assert!(set_result.is_ok());

            let new_tree = (
                BlockOrder::new(default_initialized[i]),
                cell
            );

            optional_trees.push(new_tree);
        }

        // Optionals
        if let Some(optional_indexes) = optional_indexes {
            for optional_index in optional_indexes {
                optional_trees.push((BlockOrder::new(*optional_index), OnceCell::new()));
            }
        }

        Self {
            base_tree: base_tree,
            optional_trees: optional_trees
        }
    }

    pub fn new() -> Self {
        Self::new_with_indexes(
            &vec!([TermRole::Object, TermRole::Graph, TermRole::Predicate, TermRole::Subject]),
            Some(&vec!(
                [TermRole::Graph, TermRole::Predicate, TermRole::Subject, TermRole::Object],
                [TermRole::Predicate, TermRole::Object, TermRole::Graph, TermRole::Subject],
                [TermRole::Subject, TermRole::Predicate, TermRole::Object, TermRole::Graph],
                [TermRole::Graph, TermRole::Subject, TermRole::Predicate, TermRole::Object],
                [TermRole::Object, TermRole::Subject, TermRole::Graph, TermRole::Predicate]
            ))
        )
    }

    pub fn new_anti(s: bool, p: bool, o: bool, g: bool) -> Self {
        // Index conformance expects an [&Option<u32>, 4]
        let zero = Some(0 as u32);
        let none = None;
        
        let term_roles = [
            if s { &none } else { &zero },
            if p { &none } else { &zero },
            if o { &none } else { &zero },
            if g { &none } else { &zero }
        ];

        // Possible blocks
        let mut block_candidates = vec!(
            [TermRole::Object, TermRole::Graph, TermRole::Predicate, TermRole::Subject],
            [TermRole::Graph, TermRole::Predicate, TermRole::Subject, TermRole::Object],
            [TermRole::Predicate, TermRole::Object, TermRole::Graph, TermRole::Subject],
            [TermRole::Subject, TermRole::Predicate, TermRole::Object, TermRole::Graph],
            [TermRole::Graph, TermRole::Subject, TermRole::Predicate, TermRole::Object],
            [TermRole::Object, TermRole::Subject, TermRole::Graph, TermRole::Predicate]
        );

        let mut best_tree = 0;
        let mut best_tree_score = 0;

        for i in 0..block_candidates.len() {
            let block_order = BlockOrder::new(block_candidates[i]);
            let score = block_order.index_conformance(&term_roles);

            if score > best_tree_score {
                best_tree_score = score;
                best_tree = i;
            }
        }

        let init_block = block_candidates[best_tree];
        block_candidates.remove(best_tree);

        Self::new_with_indexes(
            &vec!(init_block),
            Some(&block_candidates)
        )
    }

    /// Returns an iterator on quads represented by their indexes from the dataset
    /// 
    /// This function can potentially build a new tree in the structure if the `can_build_new_tree` parameter is
    /// equal to true.
    pub fn search_all_matching_quads<'a>(&'a self, spog: [Option<u32>; 4], can_build_new_tree: bool) -> FilteredIndexQuads {
        // Find best index
        let term_roles = [&spog[0], &spog[1], &spog[2], &spog[3]];

        let mut best_alt_tree_pos = None;
        let mut best_index_score = self.base_tree.0.index_conformance(&term_roles);
        
        for i in 0..self.optional_trees.len() {
            if can_build_new_tree || self.optional_trees[i].1.get().is_some() {
                let score = self.optional_trees[i].0.index_conformance(&term_roles);
                if score > best_index_score {
                    best_alt_tree_pos = Some(i);
                    best_index_score = score;
                }
            }
        }

        // Split research

        let tree_description = match best_alt_tree_pos {
            Some(x) => {
                let alternative_tree_description = &self.optional_trees[x];

                (
                    &alternative_tree_description.0,
                    alternative_tree_description.1.get_or_init(|| {
                        let content = self.base_tree.0.filter(&self.base_tree.1, [None, None, None, None]);

                        let mut map = BTreeSet::new();
                        alternative_tree_description.0.insert_all_into(&mut map, content);
                        map
                    })
                )
            }
            None => (&self.base_tree.0, &self.base_tree.1)
        };

        tree_description.0.filter(&tree_description.1, spog)
    }

    /// Returns an iterator on quads represented by their indexes from the dataset
    /// 
    /// This function will always build a new tree if a better indexation is possible for this
    /// tree. If you do not want to pay the potential cost of building a new tree, call the
    /// `search_all_matching_quads` function.
    pub fn filter<'a>(&'a self, spog: [Option<u32>; 4]) -> FilteredIndexQuads {
        self.search_all_matching_quads(spog, true)
    }

    /// Inserts in the dataset the quad described by the given array of indexes.
    /// 
    /// Returns true if the quad has been inserted in the dataset (it was not
    /// already in it)
    pub fn insert_by_index(&mut self, spog: [u32; 4]) -> bool {
        if self.base_tree.0.insert_into(&mut self.base_tree.1, &spog) {
            return false;
        }

        for optional_tree_tuple in self.optional_trees.iter_mut() {
            if let Some(instancied_tree) = optional_tree_tuple.1.get_mut() {
                optional_tree_tuple.0.insert_into(instancied_tree, &spog); // assert false
            }
        }

        true
    }

    /// Deletes from the dataset the quad described by the given array of
    /// indexes.
    /// 
    /// Returns true if the quad was in the dataset (and was deleted)
    pub fn delete_by_index(&mut self, spog: [u32; 4]) -> bool {
        if !self.base_tree.0.delete_from(&mut self.base_tree.1, &spog) {
            return false;
        }

        for optional_tree_tuple in self.optional_trees.iter_mut() {
            if let Some(instancied_tree) = optional_tree_tuple.1.get_mut() {
                optional_tree_tuple.0.delete_from(instancied_tree, &spog); // assert true
            }
        }

        true
    }
}
