
use crate::RcQuad;

use sophia::dataset::Dataset;
use sophia::dataset::MutableDataset;
use sophia::term::BoxTerm;
use sophia::dataset::DQuadSource;
use sophia::quad::streaming_mode::ByValue;
use sophia::quad::streaming_mode::StreamedQuad;
use sophia::term::Term;
use sophia::term::TermData;
use sophia::dataset::MDResult;

#[cfg(test)]
use sophia::dataset::inmem::FastDataset;

#[cfg(test)]
use sophia::test_dataset_impl;

pub enum VecOrDataset<D> where D: Dataset + MutableDataset + Default {
    Vector(Vec<([BoxTerm; 3], Option<BoxTerm>)>),
    Dataset(D)
}

impl<D> Default for VecOrDataset<D> where D: Dataset + MutableDataset + Default {
    fn default() -> Self {
        VecOrDataset::Dataset(D::default())
    }
}

impl<D> VecOrDataset<D> where D: Dataset + MutableDataset + Default {
    pub fn new() -> VecOrDataset<D> {
        VecOrDataset::Dataset(D::default())
    }

    pub fn new_vector() -> VecOrDataset<D> {
        VecOrDataset::Vector(vec!())
    }
}


impl<D> Dataset for VecOrDataset<D> where D: Dataset + MutableDataset + Default {
    type Quad = ByValue<RcQuad>;
    type Error = D::Error;

    fn quads(&self) -> DQuadSource<Self> {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads()
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads()
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_g<'s, TG>(&'s self, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TG: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_g(g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_g(g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_o<'s, TO>(&'s self, o: &'s Term<TO>) -> DQuadSource<'s, Self>
    where TO: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_o(o)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_o(o)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_og<'s, TO, TG>(&'s self, o: &'s Term<TO>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TO: TermData, TG: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_og(o, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_og(o, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_p<'s, TP>(&'s self, p: &'s Term<TP>) -> DQuadSource<'s, Self>
    where TP: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_p(p)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_p(p)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_pg<'s, TP, TG>(&'s self, p: &'s Term<TP>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TP: TermData, TG: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_pg(p, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_pg(p, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_po<'s, TP, TO>(&'s self, p: &'s Term<TP>, o: &'s Term<TO>) -> DQuadSource<'s, Self>
    where TP: TermData, TO: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_po(p, o)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_po(p, o)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_pog<'s, TP, TO, TG>(&'s self, p: &'s Term<TP>, o: &'s Term<TO>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TP: TermData, TO: TermData, TG: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_pog(p, o, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_pog(p, o, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_s<'s, TS>(&'s self, s: &'s Term<TS>) -> DQuadSource<'s, Self>
    where TS: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_s(s)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_s(s)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_sg<'s, TS, TG>(&'s self, s: &'s Term<TS>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TS: TermData, TG: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_sg(s, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_sg(s, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_so<'s, TS, TO>(&'s self, s: &'s Term<TS>, o: &'s Term<TO>) -> DQuadSource<'s, Self>
    where TS: TermData, TO: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_so(s, o)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_so(s, o)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_sog<'s, TS, TO, TG>(&'s self, s: &'s Term<TS>, o: &'s Term<TO>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TS: TermData, TO: TermData, TG: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_sog(s, o, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_sog(s, o, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_sp<'s, TS, TP>(&'s self, s: &'s Term<TS>, p: &'s Term<TP>) -> DQuadSource<'s, Self>
    where TS: TermData, TP: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_sp(s, p)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_sp(s, p)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_spg<'s, TS, TP, TG>(&'s self, s: &'s Term<TS>, p: &'s Term<TP>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TS: TermData, TP: TermData, TG: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_spg(s, p, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_spg(s, p, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_spo<'s, TS, TP, TO>(&'s self, s: &'s Term<TS>, p: &'s Term<TP>, o: &'s Term<TO>) -> DQuadSource<'s, Self>
    where TS: TermData, TP: TermData, TO: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_spo(s, p, o)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_spo(s, p, o)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }

    fn quads_with_spog<'s, TS, TP, TO, TG>(&'s self, s: &'s Term<TS>, p: &'s Term<TP>, o: &'s Term<TO>, g: Option<&'s Term<TG>>) -> DQuadSource<'s, Self>
    where TS: TermData, TP: TermData, TO: TermData, TG: TermData
    {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.quads_with_spog(s, p, o, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads_with_spog(s, p, o, g)
                    .map(|q| q.unwrap())
                    .map(|q| RcQuad::new_from_quad(&q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            }
        }
    }
}

impl<D> MutableDataset for VecOrDataset<D> where D: Dataset + MutableDataset + Default {
    type MutationError = D::MutationError;

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
        match self {
            VecOrDataset::Vector(vect) => Ok(MutableDataset::insert(vect, s, p, o, g).unwrap()),
            VecOrDataset::Dataset(d) => MutableDataset::insert(d, s, p, o, g)
        }
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
        match self {
            VecOrDataset::Vector(vect) => Ok(MutableDataset::remove(vect, s, p, o, g).unwrap()),
            VecOrDataset::Dataset(d) => MutableDataset::remove(d, s, p, o, g)
        }
    }
}


#[cfg(test)]
pub type VecOrFast = VecOrDataset<FastDataset>;

#[cfg(test)]
sophia::test_dataset_impl!(test_vectorfastdatasetfast, VecOrFast, false);

#[cfg(test)]
sophia::test_dataset_impl!(test_vectorfastdatasetvect, VecOrFast, false, VecOrFast::new_vector);
