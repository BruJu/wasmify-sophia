use once_cell::unsync::OnceCell;
use std::collections::BTreeMap;


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
use sophia::term::RefTerm;
use sophia::term::Term;
use sophia::term::TermData;
use std::iter::empty;
use sophia::dataset::DResult;
use sophia::dataset::DQuad;

use crate::datamodel_quad::SophiaExportQuad;

#[cfg(test)]
use sophia::test_dataset_impl;

pub enum TermKind {
    Subject,
    Predicate,
    Object,
    Graph
}

impl PartialEq for TermKind {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl TermKind {
    pub fn get_spog_position(&self) -> usize {
        match self {
            TermKind::Subject => 0,
            TermKind::Predicate => 1,
            TermKind::Object => 2,
            TermKind::Graph => 3
        }
    }
}

/// A block is a structure that can be stored in a BTreeMap to store quads in
/// a certain order
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Block {
    data: [u32; 4],
}

impl Block {
    pub fn new(values: [u32; 4]) -> Block {
        Block { data: values }
    }
}

pub struct BlockOrder {
    term_kinds: [TermKind; 4],
    to_block_index_to_destination: [usize; 4],
    to_indices_index_to_destination: [usize; 4]
}

impl BlockOrder {
    /// Builds a block builder from an order of SPOG
    pub fn new(term_kinds: [TermKind; 4]) -> BlockOrder {
        let mut to_block_index_to_destination: [usize; 4] = [0, 0, 0, 0];
        let mut to_indices_index_to_destination: [usize; 4] = [0, 0, 0, 0];

        let regular_order: [(usize, TermKind); 4] = [
            (0, TermKind::Subject),
            (1, TermKind::Predicate),
            (2, TermKind::Object),
            (3, TermKind::Graph),
        ];

        for (term_position, term_kind) in regular_order.iter() {
            let position = term_kinds.iter().position(|x| x == term_kind);
            let position = position.unwrap();

            to_block_index_to_destination[*term_position] = position;
            to_indices_index_to_destination[position] = *term_position;
        }
        
        BlockOrder { term_kinds, to_block_index_to_destination, to_indices_index_to_destination }
    }

    /// Builds a block from SPPOG indices
    pub fn to_block(&self, indices: &[u32; 4]) -> Block {
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
    pub fn to_indices(&self, block: &Block) -> [u32; 4] {
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
        for (i, term_kind) in self.term_kinds.iter().enumerate() {
            let spog_position = term_kind.get_spog_position();
            
            if request[spog_position].is_none() {
                return i;
            }
        }

        self.term_kinds.len()
    }

    pub fn range(&self, mut spog: [Option<u32>; 4]) -> (std::ops::RangeInclusive<Block>, Option<u32>, Option<u32>, Option<u32>, Option<u32>) {
        // Restrict range as much as possible
        let mut min = [u32::min_value(), u32::min_value(), u32::min_value(), u32::min_value()];
        let mut max = [u32::max_value(), u32::max_value(), u32::max_value(), u32::max_value()];

        print!("- Enter Range : {:?}\n", spog);
        
        for (i, term_kind) in self.term_kinds.iter().enumerate() {
            let spog_position = term_kind.get_spog_position();
            
            if spog[spog_position].is_none() {
                break;
            }

            // let set_value = spog[spog_position].take().unwrap();

            min[i] = spog[spog_position].unwrap();
            max[i] = spog[spog_position].unwrap();
        }
        print!("min : {:?}\n", min);
        print!("max : {:?}\n", max);

        print!("- Exit Range : {:?}\n", spog);

        // Return range + spog that have to be filtered
        (Block::new(min)..=Block::new(max), spog[0], spog[1], spog[2], spog[3])
    }
}

pub struct TermFilter {
    pub filtered_position: [Option<u32>; 4]
}

impl TermFilter {
    pub fn empty() -> TermFilter {
        TermFilter {
            filtered_position: [ None, None, None, None ]
        }
    }

    pub fn filter(&self, spog: &[u32; 4]) -> bool {
        for i in 0..self.filtered_position.len() {
            if let Some(term) = self.filtered_position[i] {
                if spog[i] != term {
                    return false;
                }
            }
        }

        true
    }
}

pub struct QuadIndexFromSubTreeDataset<'a> {
    range: std::collections::btree_map::Range<'a, Block, ()>,
    block_order: &'a BlockOrder,
    term_filter: TermFilter
}

impl<'a> Iterator for QuadIndexFromSubTreeDataset<'a> {
    type Item = [u32; 4];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.range.next().map(|(block, _) | self.block_order.to_indices(block));

            match next.as_ref() {
                Some(spog) => if self.term_filter.filter(spog) { return next; },
                None => return None
            }
        }
    }
}

pub struct SubBTreeDataset {
    block_order: BlockOrder,
    content: OnceCell<BTreeMap<Block, ()>>
}

impl SubBTreeDataset {
    pub fn new(terms_order: [TermKind; 4], initialized: bool) -> SubBTreeDataset {
        let block_order = BlockOrder::new(terms_order);

        let subtree = SubBTreeDataset {
            block_order: block_order,
            content: OnceCell::new()
        };

        if initialized {
            subtree.content.set(BTreeMap::new());
        }

        subtree
    }

    pub fn index_conformance(&self, requested_terms: &[&Option<u32>; 4]) -> usize {
        self.block_order.index_conformance(requested_terms)
    }

    pub fn insert(&mut self, spog: &[u32; 4]) -> Option<bool> {
        if let Some(content) = self.content.get_mut() {
            let block = self.block_order.to_block(spog);

            let i = content.insert(block, ());

            match i {
                Some(_) => Some(false),
                None => Some(true)
            }
        } else {
            None
        }
    }
    
    pub fn delete(&mut self, spog: &[u32; 4]) -> Option<bool> {
        if let Some(content) = self.content.get_mut() {
            let block = self.block_order.to_block(spog);

            let i = content.remove(&block);

            match i {
                Some(_) => Some(true),
                None => Some(false)
            }
        } else {
            None
        }
    }

    pub fn contains(&self, spog: &[u32; 4]) -> Option<bool> {
        if let Some(content) = self.content.get() {
            let block = self.block_order.to_block(spog);
            Some(content.contains_key(&block))
        } else {
            None
        }
    }

    pub fn is_built(&self) -> bool {
        self.content.get().is_some()
    }

    pub fn build_content<'a>(&self, iterator: QuadIndexFromSubTreeDataset<'a>) -> Result<(), ()> {
        if self.is_built() {
            return Err(());
        }

        let mut map = BTreeMap::new();

        for spog in iterator {
            let block = self.block_order.to_block(&spog);
            map.insert(block, ());
        }

        match self.content.set(map) {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn range(&self, subject: Option<u32>, predicate: Option<u32>, object: Option<u32>, graph: Option<u32>) -> QuadIndexFromSubTreeDataset {
        let (range, subject, predicate, object, graph) = self.block_order.range([subject, predicate, object, graph]);
        print!("Tree content = {:?}", self.content.get().unwrap().to_string());
        let tree_range = self.content.get().unwrap().range(range);

        QuadIndexFromSubTreeDataset {
            range: tree_range,
            block_order: &self.block_order,
            term_filter: TermFilter {
                filtered_position: [subject, predicate, object, graph]
            }
        }
    }
}

pub struct TreedDataset {
    subtrees: Vec<SubBTreeDataset>,
    term_index: TermIndexMapU<u32, RcTermFactory>
}

impl TreedDataset {
    pub fn new() -> TreedDataset {
        let subtrees = vec!(
            SubBTreeDataset::new([TermKind::Object, TermKind::Graph, TermKind::Predicate, TermKind::Subject], true),
            SubBTreeDataset::new([TermKind::Graph, TermKind::Predicate, TermKind::Subject, TermKind::Object], false)
        );

        TreedDataset { subtrees: subtrees, term_index: TermIndexMapU::new() }
    }


    pub fn range<'a>(&'a self, subject: Option<u32>, predicate: Option<u32>, object: Option<u32>, graph: Option<u32>)
    -> QuadIndexFromSubTreeDataset {
        // Find best index
        let term_kinds = [&subject, &predicate, &object, &graph];

        let mut best_index = 0;
        let mut best_index_score = self.subtrees[0].index_conformance(&term_kinds);
        
        for i in 1..self.subtrees.len() {
            let score = self.subtrees[i].index_conformance(&term_kinds);
            if score > best_index_score {
                best_index = i;
                best_index_score = score;
            }
        }

        // Split research
        if !self.subtrees[best_index].is_built() {
            self.subtrees[best_index].build_content(self.subtrees[0].range(None, None, None, None)).unwrap();
        }

        self.subtrees[best_index].range(subject, predicate, object, graph)
    }

    pub fn insert_by_index(&mut self, spog: [u32; 4]) -> bool {
        for i in 0..self.subtrees.len() {
            if let Some(false) = self.subtrees[i].insert(&spog) {
                return false;
            }
        }

        true
    }

    pub fn delete_by_index(&mut self, spog: [u32; 4]) -> bool {
        for i in 0..self.subtrees.len() {
            if let Some(false) = self.subtrees[i].delete(&spog) {
                return false;
            }
        }

        true
    }
}

impl Dataset for TreedDataset {
    type Quad = ByValue<SophiaExportQuad>;
    type Error = Infallible;

    fn quads<'a>(&'a self) -> DQuadSource<'a, Self> {
        let quads = self.range(None, None, None, None);
        InflatedQuadsIterator::new_box(quads, &self.term_index)
    }

    fn quads_with_s<'s, T1>(&'s self, t1: &'s Term<T1>) -> DQuadSource<'s, Self>
    where T1: TermData {
        let t1 = self.term_index.get_index(&t1.into());
        if t1.is_none() {
            return Box::new(empty());
        } else {
            let quads = self.range(t1, None, None, None);
            InflatedQuadsIterator::new_box(quads, &self.term_index)
        }
    }

    fn quads_with_o<'s, T1>(&'s self, t1: &'s Term<T1>) -> DQuadSource<'s, Self>
    where T1: TermData {
        let t1 = self.term_index.get_index(&t1.into());
        if t1.is_none() {
            return Box::new(empty());
        } else {
            let quads = self.range(None, None, t1, None);
            InflatedQuadsIterator::new_box(quads, &self.term_index)
        }
    }

    fn quads_with_g<'s, T1>(&'s self, t1: Option<&'s Term<T1>>) -> DQuadSource<'s, Self>
    where T1: TermData
    {
        let t1 = self.term_index.get_index_for_graph_name(t1.map(RefTerm::from).as_ref());
        if t1.is_none() {
            return Box::new(empty());
        } else {
            let quads = self.range(None, None, None, t1);
            InflatedQuadsIterator::new_box(quads, &self.term_index)
        }
    }

}

/// An adapter that transforms an iterator on term indexes into an iterator of
/// Sophia Quads
pub struct InflatedQuadsIterator<'a> {
    base_iterator: QuadIndexFromSubTreeDataset<'a>,
    term_index: &'a TermIndexMapU<u32, RcTermFactory>
}

impl<'a> InflatedQuadsIterator<'a> {
    /// Builds a Box of InflatedQuadsIterator from an iterator on term indexes
    /// and a `TermIndexMap` to match the `DQuadSource` interface.
    pub fn new_box(
        base_iterator: QuadIndexFromSubTreeDataset<'a>,
        term_index: &'a TermIndexMapU<u32, RcTermFactory>
    ) -> Box<InflatedQuadsIterator<'a>> {
        Box::new(InflatedQuadsIterator {
            base_iterator: base_iterator,
            term_index: term_index
        })
    }
}

impl<'a> Iterator for InflatedQuadsIterator<'a> {
    type Item = DResult<TreedDataset, DQuad<'a, TreedDataset>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.base_iterator.next().map(|spog| {
            let s = self.term_index.get_term(spog[0]).unwrap();
            let p = self.term_index.get_term(spog[1]).unwrap();
            let o = self.term_index.get_term(spog[2]).unwrap();
            let g = self.term_index.get_graph_name(spog[3]).unwrap();
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
