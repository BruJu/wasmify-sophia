
use sophia::term::RcTerm;
use sophia::quad::Quad;
use std::rc::Rc;

/// A RcQuad owns its data in the form of four RcTerms.
pub struct RcQuad {
    /// Subject of the quad
    pub _subject: RcTerm,
    /// Predicate of the quad
    pub _predicate: RcTerm,
    /// Object of the quad
    pub _object: RcTerm,
    /// Graph of the quad. The default graph is represented as None
    pub _graph: Option<RcTerm>
}

impl Quad for RcQuad {
    type TermData = Rc<str>;

    fn s(&self) -> &RcTerm { &self._subject }
    fn p(&self) -> &RcTerm { &self._predicate }
    fn o(&self) -> &RcTerm { &self._object }
    fn g(&self) -> Option<&RcTerm> { self._graph.as_ref() }
}

impl RcQuad {
    /// Creates a new quad by cloning the passed RcTerms
    pub fn new(s: &RcTerm, p: &RcTerm, o: &RcTerm, g: Option<&RcTerm>) -> RcQuad {
        RcQuad {
            _subject: s.clone(),
            _predicate: p.clone(),
            _object: o.clone(),
            _graph: g.cloned()
        }
    }

    /// Creates a new quad from a Sophia Quad
    pub fn new_from_quad<Q>(quad: &Q) -> RcQuad
        where Q: Quad {
            RcQuad {
            _subject: quad.s().into(),
            _predicate: quad.p().into(),
            _object: quad.o().into(),
            _graph: quad.g().clone().map(|t| t.into())
        }
    }
}
