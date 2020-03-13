use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use sophia::graph::inmem::TermIndexMapU;
use sophia::term::factory::RcTermFactory;
use std::cell::RefCell;
use std::convert::Infallible;
use sophia::quad::streaming_mode::ByValue;
use sophia::dataset::Dataset;
use sophia::dataset::DQuadSource;
use sophia::term::index_map::TermIndexMap;
use sophia::quad::streaming_mode::StreamedQuad;
use std::cell::Ref;
use sophia::quad::Quad;
use sophia::term::RcTerm;

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

struct Data {
    three_indexes: [Option<HashMap<[u32; 3], HashSet<u32>>>; 4],
    two_indexes: [Option<HashMap<[u32; 2], HashSet<[u32; 2]>>>; 6],
    one_indexes: [Option<HashMap<u32, HashSet<[u32; 3]>>>; 4]
}


impl Data {
    pub fn insert(&mut self, spog: [u32; 4]) -> bool {
        // Check if already contains the quad
        let (key_default, value_default) = Data::decompose_3(spog, POS_DEFAULT_BUILT);
        let default_index = self.three_indexes[POS_DEFAULT_BUILT].as_ref().unwrap();
        if let Some(mapped_set) = default_index.get(&key_default) {
            if mapped_set.contains(&value_default) {
                return false;
            }
        }

        // Modify every indexes
        for (i, maybe_index) in self.three_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index {
                let (key, value) = Data::decompose_3(spog, i);
                index.entry(key).or_insert_with(HashSet::new).insert(value);
            }
        }

        for (i, maybe_index) in self.two_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index {
                let (key, value) = Data::decompose_2(spog, i);
                index.entry(key).or_insert_with(HashSet::new).insert(value);
            }
        }

        for (i, maybe_index) in self.one_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index {
                let (key, value) = Data::decompose_1(spog, i);
                index.entry(key).or_insert_with(HashSet::new).insert(value);
            }
        }

        true
    }

    pub fn remove(&mut self, spog: [u32; 4]) -> bool {
        let (key_default, value_default) = Data::decompose_3(spog, POS_DEFAULT_BUILT);
        let default_index = self.three_indexes[POS_DEFAULT_BUILT].as_ref().unwrap();
        if let Some(mapped_set) = default_index.get(&key_default) {
            if !mapped_set.contains(&value_default) {
                return false;
            }
        }

        for (i, maybe_index) in self.three_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index {
                let (key, value) = Data::decompose_3(spog, i);
                index.get_mut(&key).unwrap().remove(&value);
            }
        }

        for (i, maybe_index) in self.two_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index {
                let (key, value) = Data::decompose_2(spog, i);
                index.get_mut(&key).unwrap().remove(&value);
            }
        }

        for (i, maybe_index) in self.one_indexes.iter_mut().enumerate() {
            if let Some(index) = maybe_index {
                let (key, value) = Data::decompose_1(spog, i);
                index.get_mut(&key).unwrap().remove(&value);
            }
        }

        true
    }

    pub fn inflate_quads<'a>(&'a self, index_map: &'a TermIndexMapU<u32, RcTermFactory>)
    -> DQuadSource<'a, FullIndexDataset>
    {
        Box::new(
            self.get_3(POS_DEFAULT_BUILT)
            .map(move |spog| {
                let s = index_map.get_term(spog[0]).unwrap().clone();
                let p = index_map.get_term(spog[1]).unwrap().clone();
                let o = index_map.get_term(spog[2]).unwrap().clone();
                let g = index_map.get_term(spog[3]).unwrap().clone();
                Ok(StreamedQuad::by_value([s, p, o, g]))
            })
        )
    }


    /*
    pub fn get_quads<'a>(&'a mut self, level: u8, position: usize) -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        match level {
            3 => {
                self.ensure_built_3(position);
                self.get_3(position)
            },
            2 => {
                self.ensure_built_2(position);
                self.get_2(position)
            },
            1 => {
                self.ensure_built_1(position);
                self.get_1(position)
            },
            _ => panic!("Access to a non existing index")
        }

    }
    */

    pub fn ensure_built(&mut self, level: u8, position: usize) {
        match level {
            3 => self.ensure_built_3(position),
            2 => self.ensure_built_2(position),
            1 => self.ensure_built_1(position),
            _ => panic!("Access to a non existing index")
        }
    }

    fn ensure_built_3(&mut self, position: usize) {
        if position >= self.three_indexes.len() {
            panic!("ensure_built_3 : Invalid index {} / {}", position, self.three_indexes.len());
        } else if self.three_indexes[position].is_some() {
            return;
        }

        let mut map_to_fill = HashMap::new();

        let quads = self.get(3, POS_DEFAULT_BUILT);
        for quad in quads {
            let (key, value) = Data::decompose_3(quad, position);
            map_to_fill.entry(key).or_insert_with(HashSet::new).insert(value);            
        }

        self.three_indexes[position] = Some(map_to_fill);
    }

    fn ensure_built_2(&mut self, position: usize) {
        if position >= self.two_indexes.len() {
            panic!("ensure_built_2 : Invalid index {} / {}", position, self.two_indexes.len());
        } else if self.two_indexes[position].is_some() {
            return;
        }

        let mut map_to_fill = HashMap::new();

        let quads = self.get(3, POS_DEFAULT_BUILT);
        for quad in quads {
            let (key, value) = Data::decompose_2(quad, position);
            map_to_fill.entry(key).or_insert_with(HashSet::new).insert(value);            
        }

        self.two_indexes[position] = Some(map_to_fill);
    }

    fn ensure_built_1(&mut self, position: usize) {
        if position >= self.one_indexes.len() {
            panic!("ensure_built_1 : Invalid index {} / {}", position, self.one_indexes.len());
        } else if self.one_indexes[position].is_some() {
            return;
        }

        let mut map_to_fill = HashMap::new();

        let quads = self.get(3, POS_DEFAULT_BUILT);
        for quad in quads {
            let (key, value) = Data::decompose_1(quad, position);
            map_to_fill.entry(key).or_insert_with(HashSet::new).insert(value);            
        }

        self.one_indexes[position] = Some(map_to_fill);
    }

    pub fn get<'a>(&'a self, level: u8, position: usize) -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        match level {
            3 => self.get_3(position),
            2 => self.get_2(position),
            1 => self.get_1(position),
            _ => panic!("Get : Access to a non existing index")
        }
    }

    fn get_3<'a>(&'a self, position: usize) -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        let (k1, k2, k3, v1) = Data::indexes(3, position);

        Box::new(
            self.three_indexes[position]
                .as_ref()
                .unwrap()
                .iter()
                .flat_map(move |(key, values)| {
                    values.iter().map(move |value| {
                        let mut quad : [u32; 4] = [0, 0, 0, 0];
                        quad[k1] = key[0];
                        quad[k2] = key[1];
                        quad[k3] = key[2];
                        quad[v1] = *value;
                        quad
                    })
                })
        )
    }

    fn get_2<'a>(&'a self, position: usize) -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        let (k1, k2, v1, v2) = Data::indexes(2, position);

        Box::new(
            self.two_indexes[position]
                .as_ref()
                .unwrap()
                .iter()
                .flat_map(move |(key, values)| {
                    values.iter().map(move |value| {
                        let mut quad : [u32; 4] = [0, 0, 0, 0];
                        quad[k1] = key[0];
                        quad[k2] = key[1];
                        quad[v1] = value[0];
                        quad[v2] = value[1];
                        quad
                    })
                })
        )
    }

    fn get_1<'a>(&'a self, position: usize) -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        let (k, v1, v2, v3) = Data::indexes(1, position);

        Box::new(
            self.one_indexes[position]
                .as_ref()
                .unwrap()
                .iter()
                .flat_map(move |(key, values)| {
                    values.iter().map(move |value| {
                        let mut quad : [u32; 4] = [0, 0, 0, 0];
                        quad[k] = *key;
                        quad[v1] = value[0];
                        quad[v2] = value[1];
                        quad[v3] = value[2];
                        quad
                    })
                })
        )
    }

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
            (_, _) => panic!()
        }
    }

    fn decompose_3(quad: [u32; 4], position: usize) -> ([u32; 3], u32){
        let (k1, k2, k3, v) = Data::indexes(3, position);
        ([quad[k1], quad[k2], quad[k3]], quad[v])
    }

    fn decompose_2(quad: [u32; 4], position: usize) -> ([u32; 2], [u32; 2]){
        let (k1, k2, v1, v2) = Data::indexes(2, position);
        ([quad[k1], quad[k2]], [quad[v1], quad[v2]])
    }

    fn decompose_1(quad: [u32; 4], position: usize) -> (u32, [u32; 3]){
        let (k, v1, v2, v3) = Data::indexes(1, position);
        (quad[k], [quad[v1], quad[v2], quad[v3]])
    }
}


pub struct FullIndexDataset {
    term_index: TermIndexMapU<u32, RcTermFactory>,
    data: RefCell<Data>
}

impl Dataset for FullIndexDataset {
    type Quad = ByValue<[RcTerm; 4]>;
    type Error = Infallible;

    fn quads<'a>(&'a self) -> DQuadSource<'a, Self> {
        // self.data.borrow_mut.ensure_built_3(3, POS_DEFAULT_BUILT);

        let borrowed: Ref<'a, Data> = self.data.borrow();
        let iter = borrowed.inflate_quads(&self.term_index);
        iter
    }
}
