

use std::convert::Infallible;
use sophia::dataset::MDResult;
use sophia::dataset::Dataset;
use sophia::dataset::MutableDataset;
use sophia::dataset::DQuadSource;
use sophia::term::Term;
use sophia::term::TermData;
use sophia::quad::streaming_mode::ByRef;
use sophia::quad::streaming_mode::StreamedQuad;
use crate::datamodel_quad::SophiaExportQuad;


pub struct ArryDataset {
    s: Vec<SophiaExportQuad>
}

impl ArryDataset {
    pub fn new() -> ArryDataset {
        ArryDataset { s: Vec::new() }
    }
}

impl Dataset for ArryDataset {
    type Quad = ByRef<SophiaExportQuad>;
    type Error = Infallible;

    fn quads(&self) -> DQuadSource<Self> {
        let mut res = Vec::new();

        for stored_quad in &self.s {
            res.push(Ok(StreamedQuad::by_ref(stored_quad)));
        }

        Box::from(res.into_iter())
    }
}


impl MutableDataset for ArryDataset {
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
        
        self.s.push(SophiaExportQuad {
            _subject: s.into(),
            _predicate: p.into(),
            _object: o.into(),
            _graph: match g {
                None => None,
                Some(gprime) => Some(gprime.into())
            }
        });

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



        Ok (true)
    }

}

