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
use std::cell::Cell;
use sophia::term::RcTerm;
use sophia::term::Term;
use sophia::term::TermData;
use std::iter::empty;
use once_cell::unsync::OnceCell;
use arr_macro::arr;
use sophia::dataset::MutableDataset;
use sophia::term::RefTerm;
use sophia::dataset::MDResult;

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
    three_indexes: [OnceCell<HashMap<[u32; 3], HashSet<u32>>>; 4],
    two_indexes: [OnceCell<HashMap<[u32; 2], HashSet<[u32; 2]>>>; 6],
    one_indexes: [OnceCell<HashMap<u32, HashSet<[u32; 3]>>>; 4]
}


impl Data {
    pub fn new() -> Data {
        let data = Data {
            three_indexes: arr![OnceCell::new(); 4],
            two_indexes: arr![OnceCell::new(); 6],
            one_indexes: arr![OnceCell::new(); 4]
        };

        data.three_indexes[POS_DEFAULT_BUILT].set(HashMap::new());

        data
    }

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

    pub fn inflate_quads<'a>(&'a self, index_map: &'a TermIndexMapU<u32, RcTermFactory>)
    -> DQuadSource<'a, FullIndexDataset>
    {
        Box::new(
            self.get()
                .map(move |spog| {
                    let s = index_map.get_term(spog[0]).unwrap().clone();
                    let p = index_map.get_term(spog[1]).unwrap().clone();
                    let o = index_map.get_term(spog[2]).unwrap().clone();
                    let g = index_map.get_term(spog[3]).unwrap().clone();
                    Ok(StreamedQuad::by_value([s, p, o, g]))
                })
        )
    }

    fn build_3(&self, position: usize) -> HashMap<[u32; 3], HashSet<u32>> {
        if position >= self.three_indexes.len() {
            panic!("ensure_built_3 : Invalid index {} / {}", position, self.three_indexes.len());
        }

        let mut map_to_fill = HashMap::new();

        let quads = self.get();
        for quad in quads {
            let (key, value) = Data::decompose_3(quad, position);
            map_to_fill.entry(key).or_insert_with(HashSet::new).insert(value);            
        }

        map_to_fill
    }

    fn build_2(&self, position: usize) -> HashMap<[u32; 2], HashSet<[u32; 2]>> {
        if position >= self.two_indexes.len() {
            panic!("ensure_built_2 : Invalid index {} / {}", position, self.two_indexes.len());
        }

        let mut map_to_fill = HashMap::new();

        let quads = self.get();
        for quad in quads {
            let (key, value) = Data::decompose_2(quad, position);
            map_to_fill.entry(key).or_insert_with(HashSet::new).insert(value);            
        }

        map_to_fill
    }

    fn build_1(&self, position: usize) -> HashMap<u32, HashSet<[u32; 3]>> {
        if position >= self.one_indexes.len() {
            panic!("ensure_built_1 : Invalid index {} / {}", position, self.one_indexes.len());
        }

        let mut map_to_fill = HashMap::new();

        let quads = self.get();
        for quad in quads {
            let (key, value) = Data::decompose_1(quad, position);
            map_to_fill.entry(key).or_insert_with(HashSet::new).insert(value);            
        }

        map_to_fill
    }

    pub fn get<'a>(&'a self) -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        let (k1, k2, k3, v1) = Data::indexes(3, POS_DEFAULT_BUILT);

        Box::new(
            self.three_indexes[POS_DEFAULT_BUILT]
                .get()
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

    pub fn get_3<'a>(&'a self, position: usize, key: [u32; 3])
        -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        let map = self.three_indexes[position].get_or_init(|| self.build_3(position)).get(&key);
        
        match map {
            None => Box::new(empty()),
            Some(map) => {
                let (k1, k2, k3, v1) = Data::indexes(3, position);

                Box::new(
                    map.iter()
                        .map(move |value| {
                                let mut quad : [u32; 4] = [0, 0, 0, 0];
                                quad[k1] = key[0];
                                quad[k2] = key[1];
                                quad[k3] = key[2];
                                quad[v1] = *value;
                                quad
                            })
                )
            }
        }
    }

    pub fn get_2<'a>(&'a self, position: usize, key: [u32; 2]) -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        let map = self.two_indexes[position].get_or_init(|| self.build_2(position)).get(&key);
        
        match map {
            None => Box::new(empty()),
            Some(map) => {
                let (k1, k2, v1, v2) = Data::indexes(2, position);

                Box::new(
                    map.iter()
                        .map(move |value| {
                                let mut quad : [u32; 4] = [0, 0, 0, 0];
                                quad[k1] = key[0];
                                quad[k2] = key[1];
                                quad[v1] = value[0];
                                quad[v2] = value[1];
                                quad
                            })
                )
            }
        }
    }

    pub fn get_1<'a>(&'a self, position: usize, key: u32) -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        let map = self.one_indexes[position].get_or_init(|| self.build_1(position)).get(&key);
        
        match map {
            None => Box::new(empty()),
            Some(map) => {
                let (k, v1, v2, v3) = Data::indexes(1, position);

                Box::new(
                    map.iter()
                        .map(move |value| {
                                let mut quad : [u32; 4] = [0, 0, 0, 0];
                                quad[k] = key;
                                quad[v1] = value[0];
                                quad[v2] = value[1];
                                quad[v3] = value[2];
                                quad
                            })
                )
            }
        }
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
    data: Data
}

impl FullIndexDataset {
    pub fn new() -> FullIndexDataset {
        FullIndexDataset {
            term_index: TermIndexMapU::new(),
            data: Data::new()
        }
    }
}

impl Dataset for FullIndexDataset {
    type Quad = ByValue<[RcTerm; 4]>;
    type Error = Infallible;

    fn quads<'a>(&'a self) -> DQuadSource<'a, Self> {
        self.data.inflate_quads(&self.term_index)
    }

    /*
    fn quads_with_s<'s, T>(&'s self, p: &'s Term<T>) -> DQuadSource<'s, Self>
        where T: TermData
    {
        self.data.borrow_mut().ensure_built_1(POS_S);
        self.unsafe_ref_data().inflate_quads(&self.term_index)
    }
    */
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
        let gi = self.term_index.get_index_for_graph_name(g.map(RefTerm::from).as_ref());
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