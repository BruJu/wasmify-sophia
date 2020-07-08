
use crate::datamodel_quad::SophiaExportQuad;

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
    type Quad = ByValue<SophiaExportQuad>;
    type Error = D::Error;

    fn quads(&self) -> DQuadSource<Self> {
        match &self {
            VecOrDataset::Vector(vect) => {
                let qs = vect.into_iter()
                    .map(|q| SophiaExportQuad::new_from_quad(q))
                    .map(|q| Ok(StreamedQuad::by_value(q)));

                Box::new(qs)
            },
            VecOrDataset::Dataset(d) => {
                let qs = d.quads()
                    .map(|q| q.unwrap())
                    .map(|q| SophiaExportQuad::new_from_quad(&q))
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
