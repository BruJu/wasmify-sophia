use once_cell::unsync::OnceCell;
use std::collections::BTreeSet;

use sophia::dataset::MutableDataset;
use std::convert::Infallible;
use sophia::dataset::DQuadSource;
use sophia::dataset::Dataset;
use sophia::dataset::MDResult;
use sophia::graph::inmem::TermIndexMapU;
use sophia::quad::streaming_mode::ByValue;
use sophia::quad::streaming_mode::StreamedQuad;
use sophia::term::factory::RcTermFactory;
use sophia::term::index_map::TermIndexMap;
use sophia::term::RcTerm;
use sophia::term::RefTerm;
use sophia::term::Term;
use sophia::term::TermData;
use std::iter::empty;
use sophia::dataset::DResult;
use sophia::dataset::DQuad;

use crate::datamodel_quad::SophiaExportQuad;

#[cfg(test)]
use sophia::test_dataset_impl;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TermRole {
    Subject = 0,
    Predicate = 1,
    Object = 2,
    Graph = 3,
}

/// A block is a structure that can be stored in a BTreeSet to store quads in
/// a certain order
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
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
pub struct TreedDataset {
    /// The tree that is always instancied
    base_tree: (BlockOrder, BTreeSet<Block<u32>>),
    /// A list of optional trees that can be instancied ot improve look up
    /// performances at the cost of further insert and deletions
    optional_trees: Vec<(BlockOrder, OnceCell<BTreeSet<Block<u32>>>)>,
    /// A term index map that matches RcTerms with u32 indexes
    term_index: TermIndexMapU<u32, RcTermFactory>
}

impl Default for TreedDataset {
    fn default() -> TreedDataset {
        TreedDataset {
            base_tree: (
                BlockOrder::new([TermRole::Object, TermRole::Graph, TermRole::Predicate, TermRole::Subject]),
                BTreeSet::new()
            ),
            optional_trees: vec!(
                (BlockOrder::new([TermRole::Graph, TermRole::Subject, TermRole::Predicate, TermRole::Object]), OnceCell::new())
            ),
            term_index: TermIndexMapU::new()
        }
    }
}

impl TreedDataset {
    pub fn new_with_indexes(default_initialized: &Vec<[TermRole; 4]>, optional_indexes: Option<&Vec<[TermRole; 4]>>) -> TreedDataset {
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

        TreedDataset {
            base_tree: base_tree,
            optional_trees: optional_trees,
            term_index: TermIndexMapU::new()
        }
    }

    pub fn new() -> TreedDataset {
        TreedDataset::new_with_indexes(
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

    pub fn new_anti(s: bool, p: bool, o: bool, g: bool) -> TreedDataset {
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

        TreedDataset::new_with_indexes(
            &vec!(init_block),
            Some(&block_candidates)
        )
    }

    /// Returns an iterator on quads represented by their indexes from the 
    pub fn filter<'a>(&'a self, spog: [Option<u32>; 4]) -> FilteredIndexQuads {
        // Find best index
        let term_roles = [&spog[0], &spog[1], &spog[2], &spog[3]];

        let mut best_alt_tree_pos = None;
        let mut best_index_score = self.base_tree.0.index_conformance(&term_roles);
        
        for i in 0..self.optional_trees.len() {
            let score = self.optional_trees[i].0.index_conformance(&term_roles);
            if score > best_index_score {
                best_alt_tree_pos = Some(i);
                best_index_score = score;
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

impl TreedDataset {
    /// Returns an iterator on Sophia Quads that matches the given pattern of indexes.
    /// 
    /// indexes is in the format on four term indexes, in the order Subject,
    /// Prdicate, Object, Graph. None means every term must be matched, a given
    /// value that only the given term must be matched.
    fn quads_with_opt_spog<'s>(&'s self, indexes: [Option<u32>; 4]) -> DQuadSource<'s, Self> {
        let quads = self.filter(indexes);
        InflatedQuadsIterator::new_box(quads, &self.term_index)
    }
}

impl Dataset for TreedDataset {
    type Quad = ByValue<SophiaExportQuad>;
    type Error = Infallible;

    fn quads<'a>(&'a self) -> DQuadSource<'a, Self> {
        self.quads_with_opt_spog([None, None, None, None])
    }

    // One term
    fn quads_with_s<'s, TS>(&'s self, s: &'s Term<TS>) -> DQuadSource<'s, Self>
    where TS: TermData {
        let s = self.term_index.get_index(&s.into());
        if s.is_none() {
            return Box::new(empty());
        } else {
            self.quads_with_opt_spog([s, None, None, None])
        }
    }

    fn quads_with_p<'s, TP>(&'s self, p: &'s Term<TP>) -> DQuadSource<'s, Self>
    where TP: TermData {
        let p = self.term_index.get_index(&p.into());
        if p.is_none() {
            return Box::new(empty());
        } else {
            self.quads_with_opt_spog([None, p, None, None])
        }
    }

    fn quads_with_o<'s, TO>(&'s self, o: &'s Term<TO>) -> DQuadSource<'s, Self>
    where TO: TermData {
        let o = self.term_index.get_index(&o.into());
        if o.is_none() {
            return Box::new(empty());
        } else {
            self.quads_with_opt_spog([None, None, o, None])
        }
    }

    fn quads_with_g<'s, TG>(&'s self, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TG: TermData
    {
        let g = self.term_index.get_index_for_graph_name(g.map(RefTerm::from).as_ref());
        if g.is_none() {
            return Box::new(empty());
        } else {
            self.quads_with_opt_spog([None, None, None, g])
        }
    }

    // Two terms
    fn quads_with_sp<'s, TS, TP>(&'s self, s: &'s Term<TS>, p: &'s Term<TP>) -> DQuadSource<'s, Self>
    where TS: TermData, TP: TermData {
        let s = self.term_index.get_index(&s.into());
        if s.is_none() {
            return Box::new(empty());
        }

        let p = self.term_index.get_index(&p.into());
        if p.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([s, p, None, None])
    }

    fn quads_with_so<'s, TS, TO>(&'s self, s: &'s Term<TS>, o: &'s Term<TO>) -> DQuadSource<'s, Self>
    where TS: TermData, TO: TermData {
        let s = self.term_index.get_index(&s.into());
        if s.is_none() {
            return Box::new(empty());
        }

        let o = self.term_index.get_index(&o.into());
        if o.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([s, None, o, None])
    }

    fn quads_with_sg<'s, TS, TG>(&'s self, s: &'s Term<TS>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TS: TermData, TG: TermData {
        let s = self.term_index.get_index(&s.into());
        if s.is_none() {
            return Box::new(empty());
        }

        let g = self.term_index.get_index_for_graph_name(g.map(RefTerm::from).as_ref());
        if g.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([s, None, None, g])
    }

    fn quads_with_po<'s, TP, TO>(&'s self, p: &'s Term<TP>, o: &'s Term<TO>) -> DQuadSource<'s, Self>
    where TP: TermData, TO: TermData {
        let p = self.term_index.get_index(&p.into());
        if p.is_none() {
            return Box::new(empty());
        }

        let o = self.term_index.get_index(&o.into());
        if o.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([None, p, o, None])
    }

    fn quads_with_pg<'s, TP, TG>(&'s self, p: &'s Term<TP>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TP: TermData, TG: TermData {
        let p = self.term_index.get_index(&p.into());
        if p.is_none() {
            return Box::new(empty());
        }

        let g = self.term_index.get_index_for_graph_name(g.map(RefTerm::from).as_ref());
        if g.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([None, p, None, g])
    }

    fn quads_with_og<'s, TO, TG>(&'s self, o: &'s Term<TO>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TO: TermData, TG: TermData {
        let o = self.term_index.get_index(&o.into());
        if o.is_none() {
            return Box::new(empty());
        }

        let g = self.term_index.get_index_for_graph_name(g.map(RefTerm::from).as_ref());
        if g.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([None, None, o, g])
    }

    // Three terms
    fn quads_with_spo<'s, TS, TP, TO>(&'s self, s: &'s Term<TS>, p: &'s Term<TP>, o: &'s Term<TO>) -> DQuadSource<'s, Self>
    where TS: TermData, TP: TermData, TO: TermData {
        let s = self.term_index.get_index(&s.into());
        if s.is_none() {
            return Box::new(empty());
        }

        let p = self.term_index.get_index(&p.into());
        if p.is_none() {
            return Box::new(empty());
        }

        let o = self.term_index.get_index(&o.into());
        if o.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([s, p, o, None])
    }

    fn quads_with_spg<'s, TS, TP, TG>(&'s self, s: &'s Term<TS>, p: &'s Term<TP>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TS: TermData, TP: TermData, TG: TermData {
        let s = self.term_index.get_index(&s.into());
        if s.is_none() {
            return Box::new(empty());
        }

        let p = self.term_index.get_index(&p.into());
        if p.is_none() {
            return Box::new(empty());
        }

        let g = self.term_index.get_index_for_graph_name(g.map(RefTerm::from).as_ref());
        if g.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([s, p, None, g])
    }

    fn quads_with_sog<'s, TS, TO, TG>(&'s self, s: &'s Term<TS>, o: &'s Term<TO>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TS: TermData, TO: TermData, TG: TermData {
        let s = self.term_index.get_index(&s.into());
        if s.is_none() {
            return Box::new(empty());
        }

        let o = self.term_index.get_index(&o.into());
        if o.is_none() {
            return Box::new(empty());
        }

        let g = self.term_index.get_index_for_graph_name(g.map(RefTerm::from).as_ref());
        if g.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([s, None, o, g])
    }

    fn quads_with_pog<'s, TP, TO, TG>(&'s self, p: &'s Term<TP>, o: &'s Term<TO>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TP: TermData, TO: TermData, TG: TermData {
        let p = self.term_index.get_index(&p.into());
        if p.is_none() {
            return Box::new(empty());
        }
        
        let o = self.term_index.get_index(&o.into());
        if o.is_none() {
            return Box::new(empty());
        }

        let g = self.term_index.get_index_for_graph_name(g.map(RefTerm::from).as_ref());
        if g.is_none() {
            return Box::new(empty());
        }
        
        self.quads_with_opt_spog([None, p, o, g])
    }
    
    // Four terms

    fn quads_with_spog<'s, T1, T2, T3, T4>(&'s self, t1: &'s Term<T1>, t2: &'s Term<T2>, t3: &'s Term<T3>, t4: Option<&'s Term<T4>>) -> DQuadSource<'s, Self>
    where T1: TermData, T2: TermData, T3: TermData, T4: TermData
    {
        let t1 = self.term_index.get_index(&t1.into());
        let t2 = self.term_index.get_index(&t2.into());
        let t3 = self.term_index.get_index(&t3.into());
        let t4 = self.term_index.get_index_for_graph_name(t4.map(RefTerm::from).as_ref());
        match (t1, t2, t3, t4) {
            (Some(_), Some(_), Some(_), Some(_)) => {
                self.quads_with_opt_spog([t1, t2, t3, t4])
            },
            (_, _, _, _) => Box::new(empty())
        }
    }
}

/// An adapter that transforms an iterator on four term indexes into an iterator
/// of Sophia Quads
pub struct InflatedQuadsIterator<'a> {
    base_iterator: FilteredIndexQuads<'a>,
    term_index: &'a TermIndexMapU<u32, RcTermFactory>,
    last_tuple: Option<[(u32, &'a RcTerm); 3]>,
    last_graph: Option<(u32, &'a RcTerm)>
}

impl<'a> InflatedQuadsIterator<'a> {
    /// Builds a Box of InflatedQuadsIterator from an iterator on term indexes
    /// and a `TermIndexMap` to match the `DQuadSource` interface.
    pub fn new_box(
        base_iterator: FilteredIndexQuads<'a>,
        term_index: &'a TermIndexMapU<u32, RcTermFactory>
    ) -> Box<InflatedQuadsIterator<'a>> {
        Box::new(InflatedQuadsIterator {
            base_iterator: base_iterator,
            term_index: term_index,
            last_tuple: None,
            last_graph: None
        })
    }
}

impl<'a> Iterator for InflatedQuadsIterator<'a> {
    type Item = DResult<TreedDataset, DQuad<'a, TreedDataset>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.base_iterator.next().map(|spog| {
            let s = match self.last_tuple {
                Some([(a, x), _, _]) if a == spog[0] => x,
                _ => self.term_index.get_term(spog[0]).unwrap()
            };
            let p = match self.last_tuple {
                Some([_, (a, x), _]) if a == spog[1] => x,
                _ => self.term_index.get_term(spog[1]).unwrap()
            };
            let o = match self.last_tuple {
                Some([_, _, (a, x)]) if a == spog[2] => x,
                _ => self.term_index.get_term(spog[2]).unwrap()
            };

            self.last_tuple = Some([(spog[0], s), (spog[1], p), (spog[2], o)]);

            let g = match (spog[3], self.last_graph) {
                (x, _) if x == TermIndexMapU::<u32, RcTermFactory>::NULL_INDEX => None,
                (x, Some((y, value))) if x == y => Some(value),
                (_, _) => {
                    let g = self.term_index.get_graph_name(spog[3]).unwrap();
                    self.last_graph = Some((spog[3], g.unwrap()));
                    g
                }
            };

            Ok(StreamedQuad::by_value(SophiaExportQuad::new(&s, &p, &o, g)))
        })
    }
}

impl MutableDataset for TreedDataset {
    type MutationError = Infallible;

    fn insert<T, U, V, W>(
        &mut self,
        s: &Term<T>,
        p: &Term<U>,
        o: &Term<V>,
        g: Option<&Term<W>>,
    ) -> MDResult<Self, bool>
    where
        T: TermData,
        U: TermData,
        V: TermData,
        W: TermData,
    {
        let si = self.term_index.make_index(&s.into());
        let pi = self.term_index.make_index(&p.into());
        let oi = self.term_index.make_index(&o.into());
        let gi = self
            .term_index
            .make_index_for_graph_name(g.map(RefTerm::from).as_ref());
        let modified = self.insert_by_index([si, pi, oi, gi]);
        if modified {
            //Some([si, pi, oi, gi])
        } else {
            self.term_index.dec_ref(si);
            self.term_index.dec_ref(pi);
            self.term_index.dec_ref(oi);
            self.term_index.dec_ref(gi);
            //None
        };

        Ok(modified)
    }

    fn remove<T, U, V, W>(
        &mut self,
        s: &Term<T>,
        p: &Term<U>,
        o: &Term<V>,
        g: Option<&Term<W>>,
    ) -> MDResult<Self, bool>
    where
        T: TermData,
        U: TermData,
        V: TermData,
        W: TermData,
    {
        let si = self.term_index.get_index(&s.into());
        let pi = self.term_index.get_index(&p.into());
        let oi = self.term_index.get_index(&o.into());
        let gi = self
            .term_index
            .get_index_for_graph_name(g.map(RefTerm::from).as_ref());
        if let (Some(si), Some(pi), Some(oi), Some(gi)) = (si, pi, oi, gi) {
            let modified = self.delete_by_index([si, pi, oi, gi]);
            if modified {
                self.term_index.dec_ref(si);
                self.term_index.dec_ref(pi);
                self.term_index.dec_ref(oi);
                self.term_index.dec_ref(gi);
                return Ok(true);
            }
        }
 
        Ok(false)
    }
}

#[cfg(test)]
sophia::test_dataset_impl!(test_fulldataset, TreedDataset);
