use arr_macro::arr;
use once_cell::unsync::OnceCell;
use sophia::dataset::DQuadSource;
use sophia::dataset::Dataset;
use sophia::dataset::MDResult;
use sophia::dataset::MutableDataset;
use sophia::graph::inmem::TermIndexMapU;
use sophia::quad::streaming_mode::ByValue;
use sophia::quad::streaming_mode::StreamedQuad;
use sophia::term::factory::RcTermFactory;
use sophia::term::index_map::TermIndexMap;
use sophia::term::RcTerm;
use sophia::term::RefTerm;
use sophia::term::Term;
use sophia::term::TermData;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::convert::Infallible;
use std::iter::empty;
use sophia::dataset::DResult;
use sophia::dataset::DQuad;

use crate::datamodel_quad::SophiaExportQuad;


#[cfg(test)]
use sophia::test_dataset_impl;



const POS_GPS: usize = 0;
const POS_GPO: usize = 1;
const POS_GSO: usize = 2;
const POS_PSO: usize = 3;
const POS_DEFAULT_BUILT: usize = POS_GPS;

const POS_GP: usize = 0;
const POS_GS: usize = 1;
const POS_PS: usize = 2;
const POS_GO: usize = 3;
const POS_PO: usize = 4;
const POS_SO: usize = 5;

const POS_G: usize = 0;
const POS_P: usize = 1;
const POS_S: usize = 2;
const POS_O: usize = 3;

const QUAD_S: usize = 0;
const QUAD_P: usize = 1;
const QUAD_O: usize = 2;
const QUAD_G: usize = 3;

/// A struct that contains the quads with every possible indexing methods using
/// a hashmap.
/// 
/// The indexes are created lazily except the index that maps the Graph,
/// Predicate and Subject index to a hashset of Objects.
/// 
/// The functions of Data expects terms indexes with the order "SPOG".
/// The implementation of the methods uses a different order that uses an
/// "expected numbers of the different terms" orders which is
/// `|G| < |P| < |S |< |O|`
struct Data {
    three_indexes: [OnceCell<HashMap<[u32; 3], HashSet<u32>>>; 4],
    two_indexes: [OnceCell<HashMap<[u32; 2], HashSet<[u32; 2]>>>; 6],
    one_indexes: [OnceCell<HashMap<u32, HashSet<[u32; 3]>>>; 4],
}

impl Data {
    // == Index corresponding :
    // We receive a key with the order SGPO. We want a key with the order
    // specified in the POS constant
    

    /// Receives a SPOG ordered key, returns a GPSO ordered key
    fn rebuild_key_4(key: [u32; 4]) -> [u32; 4] {
        [key[QUAD_G], key[QUAD_P], key[QUAD_S], key[QUAD_O]]
    }

    /// Receives a SPOG ordered key with a missing term, returns its GPSO
    /// order.
    fn rebuild_key_3(key: [u32; 3], position: usize) -> [u32; 3] {
        // Key is SPOG with a missing letter
        // G always last (2), S always first (0)
        match position {
            // SP_G -> P = 1
            POS_GPS => [key[2], key[1], key[0]],
            // _POG -> P = 0, O = 1
            POS_GPO => [key[2], key[0], key[1]],
            // S_OG -> O = 1
            POS_GSO => [key[2], key[0], key[1]],
            // SPO_ -> P = 1, O = 2
            POS_PSO => [key[1], key[0], key[2]],
            _ => panic!()
        }
    }

    /// Receives a SPOG ordered key with two missing terms, returns its
    /// GPSO order
    fn rebuild_key_2(key: [u32; 2], position: usize) -> [u32; 2] {
        let (k1, k2, _, _) = Data::indexes(2, position);
        if k1 < k2 {
            key
        } else {
            // The first indexed term has a greater value in SPOG ordering than
            // the second : we need to flip the keys
            [key[1], key[0]]
        }
    }

    /// Maps the number of indexes and the positions of the index in the hashmap
    /// array to the list of positions of terms in the SPOG order.
    fn indexes(level: u8, position: usize) -> (usize, usize, usize, usize) {
        match (level, position) {
            (3, POS_GPS) => (QUAD_G, QUAD_P, QUAD_S, QUAD_O),
            (3, POS_GPO) => (QUAD_G, QUAD_P, QUAD_O, QUAD_S),
            (3, POS_GSO) => (QUAD_G, QUAD_S, QUAD_O, QUAD_P),
            (3, POS_PSO) => (QUAD_P, QUAD_S, QUAD_O, QUAD_G),
            (2, POS_GP) => (QUAD_G, QUAD_P, QUAD_S, QUAD_O),
            (2, POS_GS) => (QUAD_G, QUAD_S, QUAD_P, QUAD_O),
            (2, POS_PS) => (QUAD_P, QUAD_S, QUAD_G, QUAD_O),
            (2, POS_GO) => (QUAD_G, QUAD_O, QUAD_P, QUAD_S),
            (2, POS_PO) => (QUAD_P, QUAD_O, QUAD_G, QUAD_S),
            (2, POS_SO) => (QUAD_S, QUAD_O, QUAD_G, QUAD_P),
            (1, POS_G) => (QUAD_G, QUAD_P, QUAD_S, QUAD_O),
            (1, POS_P) => (QUAD_P, QUAD_G, QUAD_S, QUAD_O),
            (1, POS_S) => (QUAD_S, QUAD_G, QUAD_P, QUAD_O),
            (1, POS_O) => (QUAD_O, QUAD_G, QUAD_P, QUAD_S),
            (_, _) => panic!(),
        }
    }

    // Data implementation

    /// Builds a new data indexer
    pub fn new() -> Data {
        let data = Data {
            three_indexes: arr![OnceCell::new(); 4],
            two_indexes: arr![OnceCell::new(); 6],
            one_indexes: arr![OnceCell::new(); 4],
        };

        data.three_indexes[POS_DEFAULT_BUILT].set(HashMap::new()).unwrap();

        data
    }

    /// Inserts the passed SPOG quad indexes into the structure
    pub fn insert(&mut self, spog: [u32; 4]) -> bool {
        // Check if already contains the quad
        let (key_default, value_default) = Data::decompose_3(spog, POS_DEFAULT_BUILT);
        let default_index = self.three_indexes[POS_DEFAULT_BUILT].get().unwrap();
        if let Some(mapped_set) = default_index.get(&key_default) {
            if mapped_set.contains(&value_default) {
                return false;
            }
        }

        // Modify every indexes
        for (i, maybe_index) in self.three_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index.get_mut() {
                let (key, value) = Data::decompose_3(spog, i);
                index.entry(key).or_insert_with(HashSet::new).insert(value);
            }
        }

        for (i, maybe_index) in self.two_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index.get_mut() {
                let (key, value) = Data::decompose_2(spog, i);
                index.entry(key).or_insert_with(HashSet::new).insert(value);
            }
        }

        for (i, maybe_index) in self.one_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index.get_mut() {
                let (key, value) = Data::decompose_1(spog, i);
                index.entry(key).or_insert_with(HashSet::new).insert(value);
            }
        }

        true
    }

    /// Removes the passed SPOG quad indexes from the structure
    pub fn remove(&mut self, spog: [u32; 4]) -> bool {
        let (key_default, value_default) = Data::decompose_3(spog, POS_DEFAULT_BUILT);
        let default_index = self.three_indexes[POS_DEFAULT_BUILT].get().unwrap();
        if let Some(mapped_set) = default_index.get(&key_default) {
            if !mapped_set.contains(&value_default) {
                return false;
            }
        }

        for (i, maybe_index) in self.three_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index.get_mut() {
                let (key, value) = Data::decompose_3(spog, i);
                index.get_mut(&key).unwrap().remove(&value);
            }
        }

        for (i, maybe_index) in self.two_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index.get_mut() {
                let (key, value) = Data::decompose_2(spog, i);
                index.get_mut(&key).unwrap().remove(&value);
            }
        }

        for (i, maybe_index) in self.one_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index.get_mut() {
                let (key, value) = Data::decompose_1(spog, i);
                index.get_mut(&key).unwrap().remove(&value);
            }
        }

        true
    }

    /// Builds the *position*th 3 indexes hashmap
    fn build_3(&self, position: usize) -> HashMap<[u32; 3], HashSet<u32>> {
        if position >= self.three_indexes.len() {
            panic!(
                "ensure_built_3 : Invalid index {} / {}",
                position,
                self.three_indexes.len()
            );
        }

        let mut map_to_fill = HashMap::new();

        let quads = self.get();
        for quad in quads {
            let (key, value) = Data::decompose_3(quad, position);
            map_to_fill
                .entry(key)
                .or_insert_with(HashSet::new)
                .insert(value);
        }

        map_to_fill
    }

    /// Builds the *position*th 2 indexes hashmap
    fn build_2(&self, position: usize) -> HashMap<[u32; 2], HashSet<[u32; 2]>> {
        if position >= self.two_indexes.len() {
            panic!(
                "ensure_built_2 : Invalid index {} / {}",
                position,
                self.two_indexes.len()
            );
        }

        let mut map_to_fill = HashMap::new();

        let quads = self.get();
        for quad in quads {
            let (key, value) = Data::decompose_2(quad, position);
            map_to_fill
                .entry(key)
                .or_insert_with(HashSet::new)
                .insert(value);
        }

        map_to_fill
    }

    /// Builds the *position*th 1 index hashmap
    fn build_1(&self, position: usize) -> HashMap<u32, HashSet<[u32; 3]>> {
        if position >= self.one_indexes.len() {
            panic!(
                "ensure_built_1 : Invalid index {} / {}",
                position,
                self.one_indexes.len()
            );
        }

        let mut map_to_fill = HashMap::new();

        let quads = self.get();
        for quad in quads {
            let (key, value) = Data::decompose_1(quad, position);
            map_to_fill
                .entry(key)
                .or_insert_with(HashSet::new)
                .insert(value);
        }

        map_to_fill
    }

    /// Returns every quads in the form of 4 indexes
    pub fn get<'a>(&'a self) -> Box<dyn Iterator<Item = [u32; 4]> + 'a> {
        let (k1, k2, k3, v1) = Data::indexes(3, POS_DEFAULT_BUILT);

        Box::new(
            self.three_indexes[POS_DEFAULT_BUILT]
                .get()
                .unwrap()
                .iter()
                .flat_map(move |(key, values)| {
                    values.iter().map(move |value| {
                        let mut quad: [u32; 4] = [0, 0, 0, 0];
                        quad[k1] = key[0];
                        quad[k2] = key[1];
                        quad[k3] = key[2];
                        quad[v1] = *value;
                        quad
                    })
                }),
        )
    }

    /// Returns every quads that matches the passed SPOG pattern
    pub fn get_4<'a>(
        &'a self,
        key: [u32; 4],
    ) -> Box<dyn Iterator<Item = [u32; 4]> + 'a> {
        let key = Data::rebuild_key_4(key);

        let map = self.three_indexes[POS_DEFAULT_BUILT].get().unwrap().get(&key[0..3]);

        match map {
            None => Box::new(empty()),
            Some(map) => {
                let (k1, k2, k3, v1) = Data::indexes(3, POS_DEFAULT_BUILT);

                Box::new(map.iter()
                            .filter(move |value| **value == key[3])
                            .map(move |value| {
                                let mut quad: [u32; 4] = [0, 0, 0, 0];
                                quad[k1] = key[0];
                                quad[k2] = key[1];
                                quad[k3] = key[2];
                                quad[v1] = *value;
                                quad
                            }))
            }
        }
    }

    /// Returns every quads that matches the passed SPOG pattern with three
    /// known terms
    pub fn get_3<'a>(
        &'a self,
        position: usize,
        key: [u32; 3],
    ) -> Box<dyn Iterator<Item = [u32; 4]> + 'a> {
        let key = Data::rebuild_key_3(key, position);

        let map = self.three_indexes[position]
            .get_or_init(|| self.build_3(position))
            .get(&key);
        match map {
            None => Box::new(empty()),
            Some(map) => {
                let (k1, k2, k3, v1) = Data::indexes(3, position);

                Box::new(map.iter().map(move |value| {
                    let mut quad: [u32; 4] = [0, 0, 0, 0];
                    quad[k1] = key[0];
                    quad[k2] = key[1];
                    quad[k3] = key[2];
                    quad[v1] = *value;
                    quad
                }))
            }
        }
    }

    /// Returns every quads that matches the passed SPOG pattern with two
    /// known terms
    pub fn get_2<'a>(
        &'a self,
        position: usize,
        key: [u32; 2],
    ) -> Box<dyn Iterator<Item = [u32; 4]> + 'a> {
        let key = Data::rebuild_key_2(key, position);

        let map = self.two_indexes[position]
            .get_or_init(|| self.build_2(position))
            .get(&key);
        match map {
            None => Box::new(empty()),
            Some(map) => {
                let (k1, k2, v1, v2) = Data::indexes(2, position);

                Box::new(map.iter().map(move |value| {
                    let mut quad: [u32; 4] = [0, 0, 0, 0];
                    quad[k1] = key[0];
                    quad[k2] = key[1];
                    quad[v1] = value[0];
                    quad[v2] = value[1];
                    quad
                }))
            }
        }
    }

    /// Returns every quads that matches the passed SPOG pattern with one
    /// known term
    pub fn get_1<'a>(
        &'a self,
        position: usize,
        key: u32,
    ) -> Box<dyn Iterator<Item = [u32; 4]> + 'a> {
        let map = self.one_indexes[position]
            .get_or_init(|| self.build_1(position))
            .get(&key);
        match map {
            None => Box::new(empty()),
            Some(map) => {
                let (k, v1, v2, v3) = Data::indexes(1, position);

                Box::new(map.iter().map(move |value| {
                    let mut quad: [u32; 4] = [0, 0, 0, 0];
                    quad[k] = key;
                    quad[v1] = value[0];
                    quad[v2] = value[1];
                    quad[v3] = value[2];
                    quad
                }))
            }
        }
    }

    /// Decompose the SPGO quad into the wanted 3 indexes quad for the
    /// *position*th hashmap into the form (key, value)
    fn decompose_3(quad: [u32; 4], position: usize) -> ([u32; 3], u32) {
        let (k1, k2, k3, v) = Data::indexes(3, position);
        ([quad[k1], quad[k2], quad[k3]], quad[v])
    }

    /// Decompose the SPGO quad into the wanted 2 indexes quad for the
    /// *position*th hashmap into the form (key, value)
    fn decompose_2(quad: [u32; 4], position: usize) -> ([u32; 2], [u32; 2]) {
        let (k1, k2, v1, v2) = Data::indexes(2, position);
        ([quad[k1], quad[k2]], [quad[v1], quad[v2]])
    }

    /// Decompose the SPGO quad into the wanted 1 index quad for the
    /// *position*th hashmap into the form (key, value)
    fn decompose_1(quad: [u32; 4], position: usize) -> (u32, [u32; 3]) {
        let (k, v1, v2, v3) = Data::indexes(1, position);
        (quad[k], [quad[v1], quad[v2], quad[v3]])
    }
}

/// A dataset that can lazily build every possible indexes
pub struct FullIndexDataset {
    term_index: TermIndexMapU<u32, RcTermFactory>,
    data: Data,
}

/// An adapter that transforms an iterator on term indexes into an iterator of
/// Sophia Quads
pub struct InflatedQuadsIterator<'a> {
    base_iterator: Box<dyn Iterator<Item = [u32; 4]> + 'a>,
    term_index: &'a TermIndexMapU<u32, RcTermFactory>
}

impl<'a> InflatedQuadsIterator<'a> {
    /// Builds a Box of InflatedQuadsIterator from an iterator on term indexes
    /// and a `TermIndexMap` to match the `DQuadSource` interface.
    pub fn new_box(
        base_iterator: Box<dyn Iterator<Item = [u32; 4]> + 'a>,
        term_index: &'a TermIndexMapU<u32, RcTermFactory>
    ) -> Box<InflatedQuadsIterator<'a>> {
        Box::new(InflatedQuadsIterator {
            base_iterator: base_iterator,
            term_index: term_index
        })
    }
}

impl<'a> Iterator for InflatedQuadsIterator<'a> {
    type Item = DResult<FullIndexDataset, DQuad<'a, FullIndexDataset>>;

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

impl FullIndexDataset {
    /// Builds an empty `FullIndexDataset`
    pub fn new() -> FullIndexDataset {
        FullIndexDataset {
            term_index: TermIndexMapU::new(),
            data: Data::new(),
        }
    }
}

macro_rules! full_indexed_dataset_quads_with {
    ($function_name: ident, $position: expr, 1) => (
        fn $function_name<'s, T1>(&'s self, t1: &'s Term<T1>) -> DQuadSource<'s, Self>
        where T1: TermData
        {
            let t1 = self.term_index.get_index(&t1.into());
            if t1.is_none() {
                return Box::new(empty());
            } else {
                let quads = self.data.get_1($position, t1.unwrap());
                InflatedQuadsIterator::new_box(quads, &self.term_index)
            }
        }
    );

    ($function_name: ident, $position: expr, 2) => (
        fn $function_name<'s, T1, T2>(&'s self, t1: &'s Term<T1>, t2: &'s Term<T2>) -> DQuadSource<'s, Self>
        where T1: TermData, T2: TermData
        {
            let t1 = self.term_index.get_index(&t1.into());
            let t2 = self.term_index.get_index(&t2.into());
            match (t1, t2) {
                (Some(t1), Some(t2)) => {
                    let quads = self.data.get_2($position, [t1, t2]);
                    InflatedQuadsIterator::new_box(quads, &self.term_index)
                },
                (_, _) => Box::new(empty())
            }
        }
    );

    ($function_name: ident, $position: expr, 3) => (
        fn $function_name<'s, T1, T2, T3>(&'s self, t1: &'s Term<T1>, t2: &'s Term<T2>, t3: &'s Term<T3>) -> DQuadSource<'s, Self>
        where T1: TermData, T2: TermData, T3: TermData
        {
            let t1 = self.term_index.get_index(&t1.into());
            let t2 = self.term_index.get_index(&t2.into());
            let t3 = self.term_index.get_index(&t3.into());
            match (t1, t2, t3) {
                (Some(t1), Some(t2), Some(t3)) => {
                    let quads = self.data.get_3($position, [t1, t2, t3]);
                    InflatedQuadsIterator::new_box(quads, &self.term_index)
                },
                (_, _, _) => Box::new(empty())
            }
        }
    );
    
    ($function_name: ident, $position: expr, 1, true) => (
        fn $function_name<'s, T1>(&'s self, t1: Option<&'s Term<T1>>) -> DQuadSource<'s, Self>
        where T1: TermData
        {
            let t1 = self.term_index.get_index_for_graph_name(t1.map(RefTerm::from).as_ref());
            if t1.is_none() {
                return Box::new(empty());
            } else {
                let quads = self.data.get_1($position, t1.unwrap());
                InflatedQuadsIterator::new_box(quads, &self.term_index)
            }
        }
    );

    ($function_name: ident, $position: expr, 2, true) => (
        fn $function_name<'s, T1, T2>(&'s self, t1: &'s Term<T1>, t2: Option<&'s Term<T2>>) -> DQuadSource<'s, Self>
        where T1: TermData, T2: TermData
        {
            let t1 = self.term_index.get_index(&t1.into());
            let t2 = self.term_index.get_index_for_graph_name(t2.map(RefTerm::from).as_ref());
            match (t1, t2) {
                (Some(t1), Some(t2)) => {
                    let quads = self.data.get_2($position, [t1, t2]);
                    InflatedQuadsIterator::new_box(quads, &self.term_index)
                },
                (_, _) => Box::new(empty())
            }
        }
    );

    ($function_name: ident, $position: expr, 3, true) => (
        fn $function_name<'s, T1, T2, T3>(&'s self, t1: &'s Term<T1>, t2: &'s Term<T2>, t3: Option<&'s Term<T3>>) -> DQuadSource<'s, Self>
        where T1: TermData, T2: TermData, T3: TermData
        {
            let t1 = self.term_index.get_index(&t1.into());
            let t2 = self.term_index.get_index(&t2.into());
            let t3 = self.term_index.get_index_for_graph_name(t3.map(RefTerm::from).as_ref());
            match (t1, t2, t3) {
                (Some(t1), Some(t2), Some(t3)) => {
                    let quads = self.data.get_3($position, [t1, t2, t3]);
                    InflatedQuadsIterator::new_box(quads, &self.term_index)
                },
                (_, _, _) => Box::new(empty())
            }
        }
    );
    
    ($function_name: ident) => (
        fn $function_name<'s, T1, T2, T3, T4>(&'s self, t1: &'s Term<T1>, t2: &'s Term<T2>, t3: &'s Term<T3>, t4: Option<&'s Term<T4>>) -> DQuadSource<'s, Self>
        where T1: TermData, T2: TermData, T3: TermData, T4: TermData
        {
            let t1 = self.term_index.get_index(&t1.into());
            let t2 = self.term_index.get_index(&t2.into());
            let t3 = self.term_index.get_index(&t3.into());
            let t4 = self.term_index.get_index_for_graph_name(t4.map(RefTerm::from).as_ref());
            match (t1, t2, t3, t4) {
                (Some(t1), Some(t2), Some(t3), Some(t4)) => {
                    let quads = self.data.get_4([t1, t2, t3, t4]);
                    InflatedQuadsIterator::new_box(quads, &self.term_index)
                },
                (_, _, _, _) => Box::new(empty())
            }
        }
    );
}


impl Dataset for FullIndexDataset {
    type Quad = ByValue<SophiaExportQuad>;
    type Error = Infallible;

    fn quads<'a>(&'a self) -> DQuadSource<'a, Self> {
        let quads = self.data.get();
        InflatedQuadsIterator::new_box(quads, &self.term_index)
    }

    full_indexed_dataset_quads_with!(quads_with_s, POS_S, 1);
    full_indexed_dataset_quads_with!(quads_with_p, POS_P, 1);
    full_indexed_dataset_quads_with!(quads_with_o, POS_O, 1);
    full_indexed_dataset_quads_with!(quads_with_sp, POS_PS, 2);
    full_indexed_dataset_quads_with!(quads_with_so, POS_SO, 2);
    full_indexed_dataset_quads_with!(quads_with_po, POS_PO, 2);
    full_indexed_dataset_quads_with!(quads_with_spo, POS_PSO, 3);

    full_indexed_dataset_quads_with!(quads_with_g, POS_G, 1, true);
    full_indexed_dataset_quads_with!(quads_with_sg, POS_GS, 2, true);
    full_indexed_dataset_quads_with!(quads_with_pg, POS_GP, 2, true);
    full_indexed_dataset_quads_with!(quads_with_og, POS_GO, 2, true);
    full_indexed_dataset_quads_with!(quads_with_spg, POS_GPS, 3, true);
    full_indexed_dataset_quads_with!(quads_with_sog, POS_GSO, 3, true);
    full_indexed_dataset_quads_with!(quads_with_pog, POS_GPO, 3, true);
    full_indexed_dataset_quads_with!(quads_with_spog);
}

impl MutableDataset for FullIndexDataset {
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
        let modified = self.data.insert([si, pi, oi, gi]);
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
            let modified = self.data.remove([si, pi, oi, gi]);
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
sophia::test_dataset_impl!(test_fulldataset, FullIndexDataset);
