use std::convert::Infallible;
use sophia::dataset::MDResult;
use sophia::dataset::Dataset;
use sophia::dataset::MutableDataset;
use sophia::dataset::DQuadSource;
use sophia::term::Term;
use sophia::term::TermData;
use sophia::quad::streaming_mode::StreamedQuad;
use sophia::term::factory::RcTermFactory;
use sophia::graph::inmem::TermIndexMapU;
use sophia::dataset::DResult;
use sophia::dataset::DQuad;
use sophia::term::RcTerm;
use sophia_term::index_map::TermIndexMap;
use sophia::quad::streaming_mode::ByValue;
use sophia::term::RefTerm;
use std::rc::Rc;
use std::iter::empty;


/// A SophiaExportQuad owns its data in the form of four RcTerms.
pub struct SophiaExportQuad {
    /// Subject of the quad
    pub _subject: RcTerm,
    /// Predicate of the quad
    pub _predicate: RcTerm,
    /// Object of the quad
    pub _object: RcTerm,
    /// Graph of the quad. The default graph is represented as None
    pub _graph: Option<RcTerm>
}

// A SophiaExportQuad is a trivial to implement as a quad
impl sophia::quad::Quad for SophiaExportQuad {
    type TermData = Rc<str>;

    fn s(&self) -> &RcTerm { &self._subject }
    fn p(&self) -> &RcTerm { &self._predicate }
    fn o(&self) -> &RcTerm { &self._object }
    fn g(&self) -> Option<&RcTerm> { self._graph.as_ref() }
}

impl SophiaExportQuad {
    /// Creates a new quad by cloning the passed RcTerms
    pub fn new(s: &RcTerm, p: &RcTerm, o: &RcTerm, g: Option<&RcTerm>) -> SophiaExportQuad {
        SophiaExportQuad {
            _subject: s.clone(),
            _predicate: p.clone(),
            _object: o.clone(),
            _graph: g.cloned()
        }
    }
}

pub struct SortableDataset {
    s: Vec<[u32; 4]>,
    current_sort: Option<[bool; 4]>,
    term_index: TermIndexMapU<u32, RcTermFactory>
}

impl SortableDataset {
    pub fn new() -> SortableDataset {
        SortableDataset {
            s: Vec::new(),
            current_sort: None,
            term_index: TermIndexMapU::new()
        }
    }

    pub fn sort_for(&mut self, sort: &[bool; 4]) {
        self.s.sort_unstable_by(|a, b| {
            for i in 0..4 {
                if a[i] > b[i] {
                    return std::cmp::Ordering::Greater;
                } else if a[i] < b[i] {
                    return std::cmp::Ordering::Less;
                }
            }

            std::cmp::Ordering::Equal
        });

        self.current_sort = Some(*sort);
    }

    pub fn is_optimal(&self, indexes: &[Option<u32>; 4]) -> bool {
        if self.current_sort.is_none() {
            return false;
        }

        let b = [
            indexes[0].is_some(),
            indexes[1].is_some(),
            indexes[2].is_some(),
            indexes[3].is_some()
        ];

        self.current_sort.unwrap() == b
    }

    pub fn quads_with_opt_spog<'s>(&'s self, indexes: [Option<u32>; 4]) -> DQuadSource<'s, Self> {
        let indexref = &indexes;
        
        let quads =
            if self.is_optimal(&indexes) {
                let i_res = self.s.binary_search_by(
                    |quad_indexes| {
                        for i in 0..4 {
                            if indexes[i].is_some() {
                                let seeked = indexref[i].unwrap();
                                if seeked < quad_indexes[i] {
                                    return std::cmp::Ordering::Greater;
                                } else if seeked > quad_indexes[i] {
                                    return std::cmp::Ordering::Less;
                                }
                            }
                        }

                        std::cmp::Ordering::Equal
                    }
                );

                let b = match i_res {
                    Ok(v) => v,
                    Err(v) => v
                };


                SortableDatasetIter {
                    vect: &self.s,
                    current_pos: b,
                    is_bound: true,
                    request: indexes
                }
            } else {
                SortableDatasetIter {
                    vect: &self.s,
                    current_pos: 0,
                    is_bound: false,
                    request: indexes
                }
            };
        
        InflatedQuadsIterator::new_box(quads, &self.term_index)
    }
}

impl Dataset for SortableDataset {
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


impl MutableDataset for SortableDataset {
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
        let si = self.term_index.make_index(&s.into());
        let pi = self.term_index.make_index(&p.into());
        let oi = self.term_index.make_index(&o.into());
        let gi = self
            .term_index
            .make_index_for_graph_name(g.map(RefTerm::from).as_ref());

        self.s.push([si, pi, oi, gi]);

        self.current_sort = None;

        Ok(true)
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
        W: TermData {

            panic!("not yet implemented");
    }

}


/// An iterator on a sub tree
pub struct SortableDatasetIter<'a> {
    vect: &'a Vec<[u32; 4]>,
    current_pos: usize,
    is_bound: bool,
    request: [Option<u32>; 4]
}

impl<'a> Iterator for SortableDatasetIter<'a> {
    type Item = [u32; 4];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_pos >= self.vect.len() {
                return None;
            }

            let n = self.vect[self.current_pos];
            self.current_pos = self.current_pos + 1;

            if (self.request[0].is_none() || self.request[0].unwrap() == n[0])
            && (self.request[1].is_none() || self.request[1].unwrap() == n[1])
            && (self.request[2].is_none() || self.request[2].unwrap() == n[2])
            && (self.request[3].is_none() || self.request[3].unwrap() == n[3])
            {
                return Some(n);
            } else {
                if self.is_bound {
                    return None;
                }
            }

        }
    }
}


pub struct InflatedQuadsIterator<'a> {
    base_iterator: SortableDatasetIter<'a>,
    term_index: &'a TermIndexMapU<u32, RcTermFactory>,
    last_tuple: Option<[(u32, &'a RcTerm); 3]>,
    last_graph: Option<(u32, &'a RcTerm)>
}

impl<'a> InflatedQuadsIterator<'a> {
    /// Builds a Box of InflatedQuadsIterator from an iterator on term indexes
    /// and a `TermIndexMap` to match the `DQuadSource` interface.
    pub fn new_box(
        base_iterator: SortableDatasetIter<'a>,
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
    type Item = DResult<SortableDataset, DQuad<'a, SortableDataset>>;

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

