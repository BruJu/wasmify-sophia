use once_cell::unsync::OnceCell;
use std::collections::BTreeSet;

/// This module defines a forest structure to store quads in several trees.
/// The stored quads are expected in the form of 4 identifiers (`u32` values).
/// The semantic of the identifiers must be stored separately by the user of
/// this module.
///
/// The mains components are :
/// - `Block`: An identifier quad, which order is not SPOG
/// - `BlockOrder`: A structure that enables to convert an identfier quad into
/// a block and conversely
/// - [u32; NB_OF_TERMS]: An array of four identifiers
/// - Identifier: An `u32` (because Web Assembly is good at manipulating
/// these)
/// - `IndexingForest4Id`: A forest designed to index quads in the form of 4
/// identifiers

// Warning for the developper: Many methods use in there implementation the
// fact that there are 4 terms in a quad. So transforming the (Quad) Block
// implementation into a TripleBlock implementation is not just modifying
// NB_OF_TERMS.

/// Number of terms in a quad
pub const NB_OF_TERMS: usize = 4;

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
    data: [T; NB_OF_TERMS],
}

impl <T> Block<T> where T: Clone {
    /// Creates a block with the given values
    pub fn new(values: [T; NB_OF_TERMS]) -> Block<T> {
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

/// A block order enables to convert an identifier quad into a block and get
/// back the identifier quad.
/// 
/// It provides methods to manipulate the elements of a `BTreeSet<Block>`
/// by using functions that takes as input or returns identifier quads.
#[derive(Clone, Copy)]
pub struct BlockOrder {
    term_roles: [TermRole; NB_OF_TERMS],
    // if block is GSPO, `block_order_to_spog_order == [3, 0, 1, 2]`
    block_order_to_spog_order : [usize; NB_OF_TERMS],
    // if block is GSPO, `spog_order_to_block_order == [1, 2, 3, 0]`
    spog_order_to_block_order : [usize; NB_OF_TERMS]
}

impl BlockOrder {
    /// Returns a string that represents the block order
    pub fn name(&self) -> String {
        debug_assert!(NB_OF_TERMS == 4);
        format!(
            "{:?} {:?} {:?} {:?}",
            self.term_roles[0],
            self.term_roles[1],
            self.term_roles[2],
            self.term_roles[3]
        )
    }

    pub fn get_term_roles(&self) -> &[TermRole; NB_OF_TERMS] {
        &self.term_roles
    }

    /// Builds a block builder from an order of SPOG
    pub fn new(term_roles: [TermRole; NB_OF_TERMS]) -> BlockOrder {
        debug_assert!({
            let mut present = [false; NB_OF_TERMS];
            for tr in term_roles.iter() {
                present[*tr as usize] = true;
            }
            present.iter().all(|x| *x)
        });
        let mut block_order_to_spog_order = [0; NB_OF_TERMS];
        let mut spog_order_to_block_order = [0; NB_OF_TERMS];

        for (position, term_role) in term_roles.iter().enumerate() {
            spog_order_to_block_order[*term_role as usize] = position;
            block_order_to_spog_order[position] = *term_role as usize;
        }
        
        BlockOrder { term_roles, block_order_to_spog_order, spog_order_to_block_order }
    }

    /// Builds a block from an identifier quad
    pub fn to_block<T>(&self, identifier_quad: &[T; NB_OF_TERMS]) -> Block<T> where T: Copy {
        debug_assert!(NB_OF_TERMS == 4);
        Block{
            data: [
                identifier_quad[self.block_order_to_spog_order[0]],
                identifier_quad[self.block_order_to_spog_order[1]],
                identifier_quad[self.block_order_to_spog_order[2]],
                identifier_quad[self.block_order_to_spog_order[3]]
            ]
        }
    }

    /// Builds a block from an identifier quad pattern
    pub fn to_filter_block<T>(&self, identifier_quad_pattern: &[Option<T>; NB_OF_TERMS])
        -> Block<Option<T>> where T: Copy + PartialEq {
        Block{
            data: [
                identifier_quad_pattern[self.block_order_to_spog_order[0]],
                identifier_quad_pattern[self.block_order_to_spog_order[1]],
                identifier_quad_pattern[self.block_order_to_spog_order[2]],
                identifier_quad_pattern[self.block_order_to_spog_order[3]]
            ]
        }
    }

    /// Builds an identifier quad from a block
    pub fn to_identifier_quad<T>(&self, block: &Block<T>) -> [T; NB_OF_TERMS] where T: Copy {
        return [
            block.data[self.spog_order_to_block_order[0]],
            block.data[self.spog_order_to_block_order[1]],
            block.data[self.spog_order_to_block_order[2]],
            block.data[self.spog_order_to_block_order[3]],
        ]
    }

    /// Returns the number of term kinds in the array request_terms that can be
    /// used as a prefix
    pub fn index_conformance(&self, request: &[&Option<u32>; NB_OF_TERMS]) -> usize {
        self.term_roles
            .iter()
            .take_while(|tr| request[**tr as usize].is_some())
            .count()
    }

    /// Returns a range on every block that matches the given identifier quad
    /// pattern. The range is restricted as much as possible, but extra quads
    /// that do not match the pattern may be included (best effort).
    /// To let the user filter the extra quads, a filter block is also
    /// returned.
    pub fn range(&self, identifier_quad_pattern: [Option<u32>; NB_OF_TERMS])
        -> (std::ops::RangeInclusive<Block<u32>>, Block<Option<u32>>) {
        // Restrict range as much as possible
        let mut min = [u32::min_value(); NB_OF_TERMS];
        let mut max = [u32::max_value(); NB_OF_TERMS];

        for (i, term_role) in self.term_roles.iter().enumerate() {
            match identifier_quad_pattern[*term_role as usize] {
                None => { break; }
                Some(set_value) => {
                    min[i] = set_value;
                    max[i] = set_value;
                }
            }
        }

	    // Return range + filter block
	    (Block::new(min)..=Block::new(max), self.to_filter_block(&identifier_quad_pattern))
    }

    /// Inserts the given identifier quad in the passed tree, using this block order
    /// 
    /// Returns true if the quad was already present
    pub fn insert_into(&self, tree: &mut BTreeSet<Block<u32>>, identifier_quad: &[u32; NB_OF_TERMS]) -> bool {
        !tree.insert(self.to_block(identifier_quad))
    }

    /// Deletes the given identifier quad from the passed tree, using this block order
    /// 
    /// Returns true if the quad has been deleted
    pub fn delete_from(&self, tree: &mut BTreeSet<Block<u32>>, identifier_quad: &[u32; NB_OF_TERMS]) -> bool {
        tree.remove(&self.to_block(identifier_quad))
    }

    /// Returns true if the passed tree contains the passed quad
    pub fn contains(&self, tree: &BTreeSet<Block<u32>>, identifier_quad: &[u32; NB_OF_TERMS]) -> bool {
        tree.contains(&self.to_block(identifier_quad))
    }

    /// Inserts every identifier quad from iterator in the passed tree
    pub fn insert_all_into<'a>(&self, tree: &mut BTreeSet<Block<u32>>, iterator: IdentifierQuadFilter<'a>) {
        for block in iterator.map(|identifier_quad| self.to_block(&identifier_quad)) {
            tree.insert(block);
        }
    }

    /// Returns an iterator on every identifier quad that matches the given
    /// identifier quad pattern.
    /// 
    /// An identifier quad pattern is an array of four optional identifiers,
    /// None means every quad must be matched on this term, a given value on a
    /// term position that only quads that have the specified identifier have
    /// to be returned.
    /// 
    /// The filter tries to be smart by iterating on the less possible number
    /// of quads in the tree.
    /// 
    /// When several trees are owned, the result of the `index_conformance`
    /// method indicates how many quads will be iterated on : for two block
    /// order, the block order that returns the greater `index_conformance`
    /// will return an iterator that looks over less different quads.
    pub fn filter<'a>(&'a self, tree: &'a BTreeSet<Block<u32>>, identifier_quad_pattern: [Option<u32>; NB_OF_TERMS]) -> IdentifierQuadFilter {
        let (range, filter_block) = self.range(identifier_quad_pattern);
        let tree_range = tree.range(range);

        IdentifierQuadFilter {
            range: tree_range,
            block_order: self,
            filter_block: filter_block
        }
    }
}

/// An iterator on a sub tree
pub struct IdentifierQuadFilter<'a> {
    /// Iterator
    range: std::collections::btree_set::Range<'a, Block<u32>>,
    /// Used block order to retrived SPOG quad
    block_order: &'a BlockOrder,
    /// Term filter for quads that can't be restricted by the range
    filter_block: Block<Option<u32>>
}

impl<'a> Iterator for IdentifierQuadFilter<'a> {
    type Item = [u32; NB_OF_TERMS];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.range.next();

            match next.as_ref() {
                None => { return None; },
                Some(block) => {
                    if block.match_option_block(&self.filter_block) {
                        return Some(self.block_order.to_identifier_quad(block));
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



/// A structure that stores quads (four identifiers) in one to six `BTreeSet`.
///
/// While they contain the same element, the trees sort them in different
/// order, which enables to request for quads complying with a defined pattern
/// without iterating on quads that do not match the pattern.
///
/// The identifiers passed to this structure must be ordered by Subject,
/// Predicate, Object and Graph order. An array of four identifiers respecting
/// this order is called "Identifier Quad".
pub struct IndexingForest4Id {
    /// The tree that is always instancied
    pub base_tree: (BlockOrder, BTreeSet<Block<u32>>),
    /// A list of optional trees that can be instancied ot improve look up
    /// performances at the cost of further insert and deletions
    pub optional_trees: Vec<(BlockOrder, OnceCell<BTreeSet<Block<u32>>>)>,
}

impl Default for IndexingForest4Id {
    fn default() -> Self {
        IndexingForest4Id::new_with_indexes(
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

impl IndexingForest4Id {
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

    /// Returns an iterator on identifier quads from the dataset, respecting
    /// the given pattern.
    /// 
    /// This function can potentially build a new tree in the structure if the
    /// `can_build_new_tree` parameter is equal to true.
    pub fn search_all_matching_quads<'a>(&'a self, identifier_quad_pattern: [Option<u32>; NB_OF_TERMS], can_build_new_tree: bool) -> IdentifierQuadFilter {
        // Find best index
        let term_roles = [
            &identifier_quad_pattern[0],
            &identifier_quad_pattern[1],
            &identifier_quad_pattern[2],
            &identifier_quad_pattern[3]
        ];

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

        tree_description.0.filter(&tree_description.1, identifier_quad_pattern)
    }

    /// Returns an iterator on identifier quads from the dataset, respecting
    /// the given pattern.
    ///
    /// An identifier quad is an array of four identifiers which represents the
    /// subject, the predicate, the object and the graph (in this order).
    /// 
    /// This function will always build a new tree if a better indexation is possible for this
    /// forest. If you do not want to pay the potential cost of building a new tree, use the
    /// `search_all_matching_quads` function.
    pub fn filter<'a>(&'a self, identifier_quad_pattern: [Option<u32>; NB_OF_TERMS]) -> IdentifierQuadFilter {
        self.search_all_matching_quads(identifier_quad_pattern, true)
    }

    /// Inserts in the dataset the quad described by the given array of identifiers.
    /// 
    /// Returns true if the quad has been inserted in the dataset (it was not
    /// already in it)
    pub fn insert(&mut self, identifier_quad: [u32; NB_OF_TERMS]) -> bool {
        if self.base_tree.0.insert_into(&mut self.base_tree.1, &identifier_quad) {
            return false;
        }

        for optional_tree_tuple in self.optional_trees.iter_mut() {
            if let Some(instancied_tree) = optional_tree_tuple.1.get_mut() {
                optional_tree_tuple.0.insert_into(instancied_tree, &identifier_quad); // assert false
            }
        }

        true
    }

    /// Deletes from the dataset the quad described by the given array of
    /// identifiers.
    /// 
    /// Returns true if the quad was in the dataset (and was deleted)
    pub fn delete(&mut self, identifier_quad: [u32; NB_OF_TERMS]) -> bool {
        if !self.base_tree.0.delete_from(&mut self.base_tree.1, &identifier_quad) {
            return false;
        }

        for optional_tree_tuple in self.optional_trees.iter_mut() {
            if let Some(instancied_tree) = optional_tree_tuple.1.get_mut() {
                optional_tree_tuple.0.delete_from(instancied_tree, &identifier_quad); // assert true
            }
        }

        true
    }
}
