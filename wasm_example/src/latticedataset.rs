//! Nothing could go wrong if a dataset was too much indexed

use std::collections::hash_map::HashMap;
use sophia::dataset::Dataset;
use std::convert::Infallible;
use sophia::quad::streaming_mode::ByTermRefs;
use sophia::dataset::DQuadSource;
use sophia::graph::inmem::TermIndexMapU;
use sophia::term::factory::RcTermFactory;
use sophia::term::index_map::TermIndexMap;
use sophia::quad::streaming_mode::StreamedQuad;
use sophia::term::Term;
use sophia::term::TermData;
use std::collections::HashSet;
use std::iter::empty;
use core::cell::RefCell;

const POS_PGS: usize = 0;
const POS_PGO: usize = 1;
const POS_PSO: usize = 2;
const POS_GSO: usize = 3;

const POS_PG: usize = 0;
const POS_PS: usize = 1;
const POS_GS: usize = 2;
const POS_PO: usize = 3;
const POS_GO: usize = 4;
const POS_SO: usize = 5;

const POS_P: usize = 0;
const POS_G: usize = 1;
const POS_S: usize = 2;
const POS_O: usize = 3;


struct LatticeData {
    three_indexes: [Option<HashMap<[u32; 3], HashSet<u32>>>; 4],
    two_indexes: [Option<HashMap<[u32; 2], HashSet<u32>>>; 6],
    one_indexes: [Option<HashMap<u32, HashSet<u32>>>; 4]
}

pub struct LatticeDataset {
    /// The term indexer
    term_index: TermIndexMapU<u32, RcTermFactory>,

    data: RefCell<LatticeData>
}

impl LatticeData {
    fn ensure_built(&mut self, level: u8, position: usize) {
        match level {
            3 => self.ensure_built_3(position),
            2 => self.ensure_built_2(position),
            1 => self.ensure_built_1(position),
            _ => panic!("Access to a non existing index")
        }
    }

    fn ensure_built_1(&mut self, position: usize) {
        if position >= self.one_indexes.len() {
            panic!("ensure_built_1 : Invalid index {} / {}", position, self.one_indexes.len());
        } else if self.one_indexes[position].is_some() {
            return;
        }

        let (index, parent, mapped_values) = get_parent_citizen(1, position);

        self.ensure_built_2(parent);

        self.one_indexes[position] = Some(HashMap::new());
        let mut map_to_fill = &mut self.one_indexes[position].unwrap();

        let (index_parent, _, mapped_values_parent) = get_parent_citizen(2, parent);

        let i = index_of(index[0], index_parent);
        let v = index_of(mapped_values, index_parent);

        self.two_indexes[parent]
            .unwrap()
            .iter()
            .map(|(key, _)| (key[i], key[v]))
            .map /* foreach */ (|(key, value)| map_to_fill.entry(key).or_insert_with(HashSet::new).insert(value));
    }

    fn ensure_built_2(&mut self, position: usize) {
        if position >= self.two_indexes.len() {
            panic!("ensure_built_2 : Invalid index {} / {}", position, self.two_indexes.len());
        } else if self.two_indexes[position].is_some() {
            return;
        }

        let (index, parent, mapped_values) = get_parent_citizen(2, position);

        self.ensure_built_3(parent);

        self.two_indexes[position] = Some(HashMap::new());
        let mut map_to_fill = &mut self.two_indexes[position].unwrap();

        let (index_parent, _, mapped_values_parent) = get_parent_citizen(3, parent);

        let i1 = index_of(index[0], index_parent);
        let i2 = index_of(index[1], index_parent);
        let v = index_of(mapped_values, index_parent);

        self.three_indexes[parent]
            .unwrap()
            .iter()
            .map(|(key, _)| ([key[i1], key[i2]], key[v]))
            .map /* foreach */ (|(key, value)| map_to_fill.entry(key).or_insert_with(HashSet::new).insert(value));
    }

    fn ensure_built_3(&mut self, position: usize) {
        if position >= self.three_indexes.len() {
            panic!("ensure_built_3 : Invalid index {} / {}", position, self.three_indexes.len());
        } else if self.three_indexes[position].is_some() {
            return;
        }

        let (index, _, mapped_values) = get_parent_citizen(3, position);

        let flatted_iter = self.three_indexes[0]
            .unwrap()
            .iter()
            .flat_map(
                |(pgs, os)| os.iter().map(|o| [pgs[2], pgs[0], *o, pgs[1]])
            );
        
        self.three_indexs[position] = Some(HashMap::new());
        let mut map_to_fill = &mut self.three_indexes[position].unwrap();

        for spog in flatted_iter {
            let key = [spog[index[0]], spog[index[1]], spog[index[2]]];
            map_to_fill.entry(key).or_insert_with(HashSet::new).insert(spog[mapped_values]);            
        }
    }

    fn get_iter_3<'a>(&'a self, position: usize, indexes: [u32; 3]) -> Box<dyn Iterator<Item=[u32;4]> + 'a> {
        let (index, _, mapped_values) = get_parent_citizen(3, position);
        let option_images = self.three_indexes[position].unwrap().get(&indexes);
        match option_images {
            Some(images) => 
                Box::new(images.iter()
                    .map(move |img| {
                        let quad: [u32; 4];
                        quad[index[0]] = indexes[0];
                        quad[index[1]] = indexes[1];
                        quad[index[2]] = indexes[2];
                        quad[mapped_values] = *img;
                        quad
                    })
                ),
            None => Box::new(::std::iter::empty())
        }
    }
}

fn index_of(searched_term: usize, parent_terms: [usize; 3]) -> usize {
    for i in 0..3 {
        if parent_terms[i] == searched_term {
            return i;
        }
    }

    panic!("index_of failed");
}

///
/// 
/// First value : the list of indexed firelds by this citizen_position. A value of GET_PARENT_CITIZEN_INVALID_INDEX
/// means that there is no more term. 
const GET_PARENT_CITIZEN_INVALID_INDEX: usize = 99;
fn get_parent_citizen(current_citizen_level: u8, citizen_position: usize) -> ([usize; 3], usize, usize) {
    const S: usize = 0;
    const P: usize = 1;
    const O: usize = 2;
    const G: usize = 3;
    const I: usize = GET_PARENT_CITIZEN_INVALID_INDEX; // No data

    match (current_citizen_level, citizen_position) {
        (3, POS_PGS) => ([P, G, S], 0, O),
        (3, POS_PGO) => ([P, G, O], 0, S),
        (3, POS_PSO) => ([P, S, O], 0, G),
        (3, POS_GSO) => ([G, S, O], 0, P),
        (2, POS_PG) => ([P, G, I], POS_PGS, S),
        (2, POS_PS) => ([P, S, I], POS_PGS, G),
        (2, POS_GS) => ([G, S, I], POS_PGS, P),
        (2, POS_PO) => ([P, O, I], POS_PGO, G),
        (2, POS_GO) => ([G, O, I], POS_PGO, P),
        (2, POS_SO) => ([S, O, I], POS_PSO, P),
        (1, POS_P) => ([P, I, I], POS_PG, G),
        (1, POS_G) => ([G, I, I], POS_PG, P),
        (1, POS_S) => ([S, I, I], POS_PS, P),
        (1, POS_O) => ([O, I, I], POS_PO, P),
        _ => panic!("get_parent_citizen unknown")
    }
}

impl LatticeDataset {
    fn rebuild_terms<'s, IteratorOnIndex>(&'s self, iter: IteratorOnIndex) -> DQuadSource<'s, Self>
    where IteratorOnIndex : Iterator<Item=[u32; 4]> {
        Box::new(
            iter.map(move |spog| {
                let s = self.term_index.get_term(spog[0]).unwrap();
                let p = self.term_index.get_term(spog[1]).unwrap();
                let o = self.term_index.get_term(spog[2]).unwrap();
                let g = self.term_index.get_term(spog[3]);
                Ok(StreamedQuad::by_term_refs(s, p, o, g))
            })
        )
    }
}

impl Dataset for LatticeDataset {
    type Quad = ByTermRefs<std::rc::Rc<str>>;
    type Error = Infallible;

    fn quads(&self) -> DQuadSource<Self> {
        Box::new(
            self.data.borrow()
                .three_indexes[0]
                .unwrap()
                .iter()
                .flat_map(
                    |(pgs, os)| os.iter().map(|o| [pgs[2], pgs[0], *o, pgs[1]])
                )
                .map(move |spog| {
                    let s = self.term_index.get_term(spog[0]).unwrap();
                    let p = self.term_index.get_term(spog[1]).unwrap();
                    let o = self.term_index.get_term(spog[2]).unwrap();
                    let g = self.term_index.get_term(spog[3]);
                    Ok(StreamedQuad::by_term_refs(s, p, o, g))
                })
            )
    }

    fn quads_with_spo<'s, T, U, V>(
        &'s self, s: &'s Term<T>, p: &'s Term<U>, o: &'s Term<V>) -> DQuadSource<'s, Self> where
        T: TermData,
        U: TermData,
        V: TermData {
            self.data.borrow_mut().ensure_built(3, POS_PSO);
            let indexes = [
                self.term_index.get_index(&p.into()),
                self.term_index.get_index(&s.into()),
                self.term_index.get_index(&o.into())
            ];

            if indexes.contains(&None) {
                return Box::new(empty());
            }

            let indexes = [indexes[0].unwrap(), indexes[1].unwrap(), indexes[2].unwrap()];

            let iter = self.data.borrow().get_iter_3(POS_PSO, indexes);
            self.rebuild_terms(iter)
    }
}

/*
impl MutableDataset for LatticeDataset {
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
        W: TermData {
        let s = self.term_index.make_index(&s.into());
        let p = self.term_index.make_index(&p.into());
        let o = self.term_index.make_index(&o.into());
        let g = self.term_index
                    .make_index_for_graph_name(g.map(RefTerm::from).as_ref());

        let inserted_array = [s, p, o, g];

        let r = self.zero_index.insert(inserted_array);

        if !r {
            return Ok(false);
        }

        // One index
        for (i, eventual_index) in self.one_index.iter_mut().enumerate() {
            if let Some(index) = eventual_index {
                if !index.contains_key(&inserted_array[i]) {
                    index[&inserted_array[i]] = HashSet::new();
                }

                index[&inserted_array[i]].insert(inserted_array);
            }
        }

        // Two index
        for (bit_index, map_element) in self.two_index.iter_mut() {
            let pos = (bit_index % 4, bit_index / 4);

            let index_cache = [&inserted_array[pos.0], &inserted_array[pos.1]];

            if !map_element.contains_key(index_cache) {
                map_element[index_cache] = HashSet::new();
            }

            map_element[index_cache].insert(inserted_array);
        }

        // Three index
        for (i, eventual_index) in self.three_index.iter_mut().enumerate() {
            if let Some(index) = eventual_index {
                let pos = [0, 0, 0];

                let i_pos = 0;
                for explored_values in 0..4 {
                    if explored_values != i {
                        pos[i_pos] = explored_values;
                        i_pos = i_pos + 1;
                    }
                }

                let index_cache = [
                    inserted_array[pos[0]],
                    inserted_array[pos[1]],
                    inserted_array[pos[2]]
                ];

                if !index.contains_key(&index_cache) {
                    index[&index_cache] = HashSet::new();
                }

                index[&index_cache].insert(inserted_array);
            }
        }
        
        Ok(true)
    }
    
    //fn remove(&mut self, _: &sophia::term::Term<T>, _: &sophia::term::Term<U>, _: &sophia::term::Term<V>, _: std::option::Option<&sophia::term::Term<W>>) -> std::result::Result<bool, <Self as sophia::dataset::_traits::MutableDataset>::MutationError> { unimplemented!() }
}
*/

/*

impl MutableDataset for StarIndexedDatAset {

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
        W: TermData {
        let s = self.term_index.make_index(&s.into());
        let p = self.term_index.make_index(&p.into());
        let o = self.term_index.make_index(&o.into());
        let g = self.term_index
                    .make_index_for_graph_name(g.map(RefTerm::from).as_ref());

        let r = self.four_indices.insert([s, p, o, g], ());

        if r.is_none() {
            return Ok(false);
        }

        Ok(true)
    }


}

*/

