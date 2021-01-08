//! This crate defines a forest structure to store an [RDF] [dataset] as a set of
//! b-trees.
//!
//! An [RDF dataset] is often seen as a set of *quads*, each composed of a
//! subject S, a predicate P, an object O, and a graph name G.
//! While in [RDF], these components are [RDF term]s, the quads handled by this
//! crate are composed of 4 identifiers (`u32` values).
//! The semantics of the identifiers (i.e. their corresponding [RDF term]s)
//! must be stored separately by the user of this crate.
//!
//! All b-trees in the forest contain the same quads, but in different orders,
//! for the purpose of efficiently replying to different queries.
//!
//! The main types of this crate are :
//! - *Identifier*: A `u32` (because Web Assembly is good at manipulating
//! these)
//! - `[u32; NB_OF_TERMS=4]`: Quads are represented by arrays of four
//! identifiers, where the elements represent S, P, O and G respectively.
//! - [`IndexingForest4`]: A forest designed to index quads of identifiers. It
//! can be used to store arrays of 4 `u32`s and query them from any pattern
//! (for example [*, 7, *, 3] will retrieve every previously
//! stored quads whose predicate is 7 and whose graph name is 3).
//! - [`Block`]: An qand of identifiers whete the SPOG components are stored in
//! a different order. A [`BlockOrder`] is required to reorder them.
//! - [`BlockOrder`]: A structure that enables to convert between [`Block`]s
//! and "canonical" (SPOG) quad of identfiers.
//!
//! [RDF]: https://www.w3.org/TR/rdf11-primer/
//! [dataset]: https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset
//! [RDF Term]: https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-term
//#![deny(missing_docs)]

use once_cell::unsync::OnceCell;
use std::collections::BTreeSet;

// Warning for the developper: Many function use in their implementation the
// fact that there are 4 terms in a quad. So transforming the (Quad) Block
// implementation into a TripleBlock implementation is not just modifying
// NB_OF_TERMS.

/// Number of terms in a quad / Number of identifiers for each value of the tree
pub const NB_OF_TERMS: usize = 4;

/// Term roles for the components of a quad in an [RDF] dataset
///
/// [RDF]: https://www.w3.org/TR/rdf11-primer/
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TermRole {
    Subject = 0,
    Predicate = 1,
    Object = 2,
    Graph = 3,
}

/// A block is a structure that can be stored in a [`BTreeSet`] to store quads in
/// a certain order.
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub struct Block<T> {
    data: [T; NB_OF_TERMS],
}

impl<T> Block<T>
where
    T: Clone,
{
    /// Creates a block with the given values (given in the canonical SPOG order)
    pub fn new(values: [T; NB_OF_TERMS]) -> Block<T> {
        Block { data: values }
    }
}

impl<T> Block<T>
where
    T: Clone + PartialEq,
{
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

/// A block order enables to convert between [`Block`] in arbitrary order and
/// identifier arrays in the SPOG order.
///
/// It provides methods to manipulate the elements of a `BTreeSet<Block>`
/// by using functions that take as input or returns identifier quads.
#[derive(Clone, Copy)]
pub struct BlockOrder {
    term_roles: [TermRole; NB_OF_TERMS],
    // e.g. if block is GSPO, `block_order_to_spog_order == [3, 0, 1, 2]`
    block_order_to_spog_order: [usize; NB_OF_TERMS],
    // e.g. if block is GSPO, `spog_order_to_block_order == [1, 2, 3, 0]`
    spog_order_to_block_order: [usize; NB_OF_TERMS],
}

impl BlockOrder {
    /// Return a string that represents the block order
    pub fn name(&self) -> String {
        debug_assert!(NB_OF_TERMS == 4);
        format!(
            "{:?} {:?} {:?} {:?}",
            self.term_roles[0], self.term_roles[1], self.term_roles[2], self.term_roles[3]
        )
    }

    /// Retun an array of [`TermRole`]s in the order corresponding to this [`BlockOrder`].
    pub fn get_term_roles(&self) -> &[TermRole; NB_OF_TERMS] {
        &self.term_roles
    }

    /// Build a [`BlockOrder`] from an order of SPOG
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

        BlockOrder {
            term_roles,
            block_order_to_spog_order,
            spog_order_to_block_order,
        }
    }

    /// Build a block from an SPOG identifier quad
    pub fn to_block<T>(&self, identifier_quad: &[T; NB_OF_TERMS]) -> Block<T>
    where
        T: Copy,
    {
        debug_assert!(NB_OF_TERMS == 4);
        Block {
            data: [
                identifier_quad[self.block_order_to_spog_order[0]],
                identifier_quad[self.block_order_to_spog_order[1]],
                identifier_quad[self.block_order_to_spog_order[2]],
                identifier_quad[self.block_order_to_spog_order[3]],
            ],
        }
    }

    /// Build a block from an identifier quad pattern
    pub fn to_filter_block<T>(
        &self,
        identifier_quad_pattern: &[Option<T>; NB_OF_TERMS],
    ) -> Block<Option<T>>
    where
        T: Copy + PartialEq,
    {
        Block {
            data: [
                identifier_quad_pattern[self.block_order_to_spog_order[0]],
                identifier_quad_pattern[self.block_order_to_spog_order[1]],
                identifier_quad_pattern[self.block_order_to_spog_order[2]],
                identifier_quad_pattern[self.block_order_to_spog_order[3]],
            ],
        }
    }

    /// Build an SPOG identifier quad from a block
    pub fn to_identifier_quad<T>(&self, block: &Block<T>) -> [T; NB_OF_TERMS]
    where
        T: Copy,
    {
        return [
            block.data[self.spog_order_to_block_order[0]],
            block.data[self.spog_order_to_block_order[1]],
            block.data[self.spog_order_to_block_order[2]],
            block.data[self.spog_order_to_block_order[3]],
        ];
    }

    /// Return the number of term roles that can be used as a prefix, with this
    /// block order, to filter the quads matching the given pattern.
    ///
    /// This gives an indication of how efficient the
    /// [`filter`](BlockOrder::filter) method will be.
    /// The higher, the better suited this block order is to answer this pattern.
    pub fn index_conformance(&self, pattern: &[&Option<u32>; NB_OF_TERMS]) -> usize {
        self.term_roles
            .iter()
            .take_while(|tr| pattern[**tr as usize].is_some())
            .count()
    }

    /// Return a range on every block that matches the given identifier quad
    /// pattern (assuming the lexicographical order).
    /// The range is restricted as much as possible, but extra quads
    /// that do not match the pattern may be included (best effort).
    /// To let the user filter the extra quads, a filter block is also
    /// returned.
    pub fn range(
        &self,
        identifier_quad_pattern: [Option<u32>; NB_OF_TERMS],
    ) -> (std::ops::RangeInclusive<Block<u32>>, Block<Option<u32>>) {
        // Restrict range as much as possible
        let mut min = [u32::min_value(); NB_OF_TERMS];
        let mut max = [u32::max_value(); NB_OF_TERMS];

        for (i, term_role) in self.term_roles.iter().enumerate() {
            match identifier_quad_pattern[*term_role as usize] {
                None => {
                    break;
                }
                Some(set_value) => {
                    min[i] = set_value;
                    max[i] = set_value;
                }
            }
        }

        // Return range + filter block
        (
            Block::new(min)..=Block::new(max),
            self.to_filter_block(&identifier_quad_pattern),
        )
    }

    /// Insert the given identifier quad in the passed tree, using this block order
    ///
    /// Return true if the quad was already present
    pub fn insert_into(
        &self,
        tree: &mut BTreeSet<Block<u32>>,
        identifier_quad: &[u32; NB_OF_TERMS],
    ) -> bool {
        !tree.insert(self.to_block(identifier_quad))
    }

    /// Delete the given identifier quad from the passed tree, using this block order
    ///
    /// Return true if the quad has been deleted
    pub fn delete_from(
        &self,
        tree: &mut BTreeSet<Block<u32>>,
        identifier_quad: &[u32; NB_OF_TERMS],
    ) -> bool {
        tree.remove(&self.to_block(identifier_quad))
    }

    /// Return true if the passed tree contains the passed quad
    pub fn contains(
        &self,
        tree: &BTreeSet<Block<u32>>,
        identifier_quad: &[u32; NB_OF_TERMS],
    ) -> bool {
        tree.contains(&self.to_block(identifier_quad))
    }

    /// Insert every identifier quad from iterator in the passed tree
    pub fn insert_all_into<'a>(
        &self,
        tree: &mut BTreeSet<Block<u32>>,
        iterator: IndexingForest4Filter<'a>,
    ) {
        for block in iterator.map(|identifier_quad| self.to_block(&identifier_quad)) {
            tree.insert(block);
        }
    }

    /// Return an iterator on every identifier quad that matches the given
    /// SPOG identifier quad pattern.
    ///
    /// An identifier quad pattern is an array of four optional identifiers,
    /// corresponding to S, P, O and G component respectively.
    /// `None` means that the returned quads may have any value for this
    /// component, while `Some(value)` means that the returned quads must
    /// have exactly `value` for this component.
    ///
    /// The filter tries to be smart by iterating on the less possible number
    /// of quads in the tree.
    ///
    /// See also [`BlockOrder::index_conformance`]
    pub fn filter<'a>(
        &'a self,
        tree: &'a BTreeSet<Block<u32>>,
        identifier_quad_pattern: [Option<u32>; NB_OF_TERMS],
    ) -> IndexingForest4Filter {
        let (range, filter_block) = self.range(identifier_quad_pattern);
        let tree_range = tree.range(range);

        IndexingForest4Filter {
            range: tree_range,
            block_order: self,
            filter_block: filter_block,
        }
    }

    /// Return a `BTreeSet<Block>` that contains every identifier quad in the
    /// `source` that does not match the `identifier_quad_pattern`. The block
    /// order of both the source tree and the returned tree is the one of
    /// this object.
    pub fn filter_to_tree(
        &self,
        source: &BTreeSet<Block<u32>>,
        identifier_quad_pattern: &[Option<u32>; 4],
    ) -> BTreeSet<Block<u32>> {
        let filter_block = self.to_filter_block(identifier_quad_pattern);

        source
            .into_iter()
            .filter(|block| !block.match_option_block(&filter_block))
            .map(|b| b.clone())
            .collect()
    }
}

/// An iterator on a sub tree
pub struct IndexingForest4Filter<'a> {
    /// Underlying iterator
    range: std::collections::btree_set::Range<'a, Block<u32>>,
    /// Used block order to convert retrieved blocks to SPOG quad
    block_order: &'a BlockOrder,
    /// Term filter for unrelevant quads that couldn't be restricted by the range
    filter_block: Block<Option<u32>>,
}

impl<'a> Iterator for IndexingForest4Filter<'a> {
    type Item = [u32; NB_OF_TERMS];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.range.next();

            match next.as_ref() {
                None => {
                    return None;
                }
                Some(block) => {
                    if block.match_option_block(&self.filter_block) {
                        return Some(self.block_order.to_identifier_quad(block));
                    }
                }
            }
        }
    }
}

/// A structure that stores quads (four identifiers) in one to six [`BTreeSet`]s.
///
/// It consists in a main tree that is always created, and the other trees that
/// are lazily created.
///
/// While they contain the same quads, the trees sort them in different
/// order, which enables to retrieve quads matching a given pattern
/// without iterating on quads that do not match the pattern.
///
/// The identifiers passed to this structure must be ordered by Subject,
/// Predicate, Object and Graph order. An array of four identifiers respecting
/// this order is called "Identifier Quad".
pub struct IndexingForest4 {
    /// The tree that is always instancied
    base_tree: (BlockOrder, BTreeSet<Block<u32>>),
    /// A list of optional trees that can be instancied to improve look up
    /// performances at the cost of further insert and deletions
    optional_trees: Vec<(BlockOrder, OnceCell<BTreeSet<Block<u32>>>)>,
}

impl Default for IndexingForest4 {
    fn default() -> Self {
        IndexingForest4::new_with_indexes(
            &vec![[
                TermRole::Object,
                TermRole::Graph,
                TermRole::Predicate,
                TermRole::Subject,
            ]],
            Some(&vec![
                [
                    TermRole::Subject,
                    TermRole::Predicate,
                    TermRole::Object,
                    TermRole::Graph,
                ],
                [
                    TermRole::Graph,
                    TermRole::Predicate,
                    TermRole::Subject,
                    TermRole::Object,
                ],
                [
                    TermRole::Predicate,
                    TermRole::Object,
                    TermRole::Graph,
                    TermRole::Subject,
                ],
                [
                    TermRole::Graph,
                    TermRole::Subject,
                    TermRole::Predicate,
                    TermRole::Object,
                ],
                [
                    TermRole::Object,
                    TermRole::Subject,
                    TermRole::Graph,
                    TermRole::Predicate,
                ],
            ]),
        )
    }
}

impl IndexingForest4 {
    /// Build an `IndexingForest4` with a tree for each `default_initialize`
    /// order built from initialization and lazy trees for each
    /// `optional_indexes` order.
    pub fn new_with_indexes(
        default_initialized: &Vec<[TermRole; 4]>,
        optional_indexes: Option<&Vec<[TermRole; 4]>>,
    ) -> Self {
        assert!(!default_initialized.is_empty());

        // Base tree
        let base_tree = (BlockOrder::new(default_initialized[0]), BTreeSet::new());

        // Redundant trees
        let mut optional_trees = Vec::new();

        // Default initialized
        for i in 1..default_initialized.len() {
            let cell = OnceCell::new();
            let set_result = cell.set(BTreeSet::new());
            assert!(set_result.is_ok());

            let new_tree = (BlockOrder::new(default_initialized[i]), cell);

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
            optional_trees: optional_trees,
        }
    }

    /// Build an `IndexingForest4` with maximum indexing capacity (5 lazy indexes).
    pub fn new() -> Self {
        Self::new_with_indexes(
            &vec![[
                TermRole::Object,
                TermRole::Graph,
                TermRole::Predicate,
                TermRole::Subject,
            ]],
            Some(&vec![
                [
                    TermRole::Graph,
                    TermRole::Predicate,
                    TermRole::Subject,
                    TermRole::Object,
                ],
                [
                    TermRole::Predicate,
                    TermRole::Object,
                    TermRole::Graph,
                    TermRole::Subject,
                ],
                [
                    TermRole::Subject,
                    TermRole::Predicate,
                    TermRole::Object,
                    TermRole::Graph,
                ],
                [
                    TermRole::Graph,
                    TermRole::Subject,
                    TermRole::Predicate,
                    TermRole::Object,
                ],
                [
                    TermRole::Object,
                    TermRole::Subject,
                    TermRole::Graph,
                    TermRole::Predicate,
                ],
            ]),
        )
    }

    /// Instanciate a new `IndexingForest4` that can build up to 6 trees, and
    /// an initial tree that has the lowest possible score for the given
    /// matching quad pattern.
    #[deprecated(note = "Use either `new` or `new_with_indexes`")]
    pub fn new_anti(s: bool, p: bool, o: bool, g: bool) -> Self {
        // Index conformance expects an [&Option<u32>, 4]
        let zero = Some(0 as u32);
        let none = None;

        let term_roles = [
            if s { &none } else { &zero },
            if p { &none } else { &zero },
            if o { &none } else { &zero },
            if g { &none } else { &zero },
        ];

        // Possible blocks
        let mut block_candidates = vec![
            [
                TermRole::Object,
                TermRole::Graph,
                TermRole::Predicate,
                TermRole::Subject,
            ],
            [
                TermRole::Graph,
                TermRole::Predicate,
                TermRole::Subject,
                TermRole::Object,
            ],
            [
                TermRole::Predicate,
                TermRole::Object,
                TermRole::Graph,
                TermRole::Subject,
            ],
            [
                TermRole::Subject,
                TermRole::Predicate,
                TermRole::Object,
                TermRole::Graph,
            ],
            [
                TermRole::Graph,
                TermRole::Subject,
                TermRole::Predicate,
                TermRole::Object,
            ],
            [
                TermRole::Object,
                TermRole::Subject,
                TermRole::Graph,
                TermRole::Predicate,
            ],
        ];

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

        Self::new_with_indexes(&vec![init_block], Some(&block_candidates))
    }

    /// Return an iterator on identifier quads from the dataset, matching
    /// the given pattern.
    ///
    /// This function can potentially build a new tree in the structure if the
    /// `can_build_new_tree` parameter is equal to true.
    pub fn search_all_matching_quads<'a>(
        &'a self,
        identifier_quad_pattern: [Option<u32>; NB_OF_TERMS],
        can_build_new_tree: bool,
    ) -> IndexingForest4Filter {
        // Find best index
        let term_roles = [
            &identifier_quad_pattern[0],
            &identifier_quad_pattern[1],
            &identifier_quad_pattern[2],
            &identifier_quad_pattern[3],
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
                        let content = self
                            .base_tree
                            .0
                            .filter(&self.base_tree.1, [None, None, None, None]);

                        let mut map = BTreeSet::new();
                        alternative_tree_description
                            .0
                            .insert_all_into(&mut map, content);
                        map
                    }),
                )
            }
            None => (&self.base_tree.0, &self.base_tree.1),
        };

        tree_description
            .0
            .filter(&tree_description.1, identifier_quad_pattern)
    }

    /// Return an iterator on identifier quads from the dataset, matching
    /// the given pattern.
    ///
    /// This function will always build a new tree if a better indexation is possible for this
    /// forest. If you do not want to pay the potential cost of building a new tree, use the
    /// [`search_all_matching_quads`](IndexingForest4::search_all_matching_quads) method.
    pub fn filter<'a>(
        &'a self,
        identifier_quad_pattern: [Option<u32>; NB_OF_TERMS],
    ) -> IndexingForest4Filter {
        self.search_all_matching_quads(identifier_quad_pattern, true)
    }

    /// Insert in the dataset the quad described by the given array of identifiers.
    ///
    /// Returns true if the quad has been inserted in the dataset (it was not
    /// already in it)
    pub fn insert(&mut self, identifier_quad: [u32; NB_OF_TERMS]) -> bool {
        if self
            .base_tree
            .0
            .insert_into(&mut self.base_tree.1, &identifier_quad)
        {
            return false;
        }

        for optional_tree_tuple in self.optional_trees.iter_mut() {
            if let Some(instancied_tree) = optional_tree_tuple.1.get_mut() {
                optional_tree_tuple
                    .0
                    .insert_into(instancied_tree, &identifier_quad); // assert false
            }
        }

        true
    }

    /// Delete from the dataset the quad described by the given array of
    /// identifiers.
    ///
    /// Returns true if the quad was in the dataset (and was deleted)
    pub fn delete(&mut self, identifier_quad: [u32; NB_OF_TERMS]) -> bool {
        if !self
            .base_tree
            .0
            .delete_from(&mut self.base_tree.1, &identifier_quad)
        {
            return false;
        }

        for optional_tree_tuple in self.optional_trees.iter_mut() {
            if let Some(instancied_tree) = optional_tree_tuple.1.get_mut() {
                optional_tree_tuple
                    .0
                    .delete_from(instancied_tree, &identifier_quad); // assert true
            }
        }

        true
    }

    /// Return the number of currently instancied trees
    pub fn get_number_of_living_trees(&self) -> usize {
        1 + self
            .optional_trees
            .iter()
            .filter(|pair| pair.1.get().is_some())
            .count()
    }

    /// Ensure the optimal index tree for this forest is built for the given
    /// query pattern.
    pub fn ensure_has_index_for(&mut self, s: bool, p: bool, o: bool, g: bool) {
        let spog: [Option<u32>; 4] = [
            if s { Some(0) } else { None },
            if p { Some(0) } else { None },
            if o { Some(0) } else { None },
            if g { Some(0) } else { None },
        ];

        let mut iter = self.search_all_matching_quads(spog, true);
        iter.next(); // Ensure the tree is not lazily built
    }
}
