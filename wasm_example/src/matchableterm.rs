//! A [`MatchableTerm`] serves as an interface between Sophia matchers and the
//! kind of matches a typical user wants to use in a query.
//! 
//! Any user can build a `MatchableTerm` from a `RcTerm` or use
//! `MatchableTerm::AnyTerm` to match everything.
//! 
//! For graphes, an `Option<RcTerm>` is expected insteas of a `RcTerm` to match
//! Sophia's representation of graphes.

use sophia::term::matcher::GraphNameMatcher;
use sophia::term::matcher::TermMatcher;
use sophia::term::RcTerm;
use sophia::term::Term;
use sophia::term::TermData;

// TODO : this class is obsolete as it was integrated in Sophia's interface

/// An enum that can be used to match with one RcTerm, DefaultGraph or any term
/// 
/// `MatchableTerm<RcTerm>` can be used as `TermMatcher`
/// `MatchableTerm<Option<RcTerm>>` can be used as `GraphNameMatcher`
/// 
/// Inspired by [sophia_rs/src/query.rs]
pub enum MatchableTerm<T> {
    ToTerm(T),
    AnyTerm,
}

impl<T> From<Option<T>> for MatchableTerm<T> {
    fn from(src: Option<T>) -> MatchableTerm<T> {
        match src {
            Some(t) => MatchableTerm::ToTerm(t),
            None => MatchableTerm::AnyTerm,
        }
    }
}

impl TermMatcher for MatchableTerm<RcTerm> {
    type TermData = std::rc::Rc<str>;
    fn constant(&self) -> Option<&Term<Self::TermData>> {
        match self {
            MatchableTerm::ToTerm(t) => Some(t),
            MatchableTerm::AnyTerm => None,
        }
    }
    fn matches<T>(&self, t: &Term<T>) -> bool
    where
        T: TermData,
    {
        match self {
            MatchableTerm::ToTerm(tself) => tself == t,
            MatchableTerm::AnyTerm => true,
        }
    }
}

impl GraphNameMatcher for MatchableTerm<Option<RcTerm>> {
    type TermData = std::rc::Rc<str>;

    fn constant(&self) -> Option<Option<&Term<Self::TermData>>> {
        match self {
            MatchableTerm::ToTerm(t) => Some(t.as_ref()),
            MatchableTerm::AnyTerm => None,
        }
    }

    fn matches<T>(&self, t: Option<&Term<T>>) -> bool
    where
        T: TermData,
    {
        match self {
            MatchableTerm::ToTerm(Some(term)) => match t {
                Some(arg_t) => arg_t == term,
                None => false
            },
            MatchableTerm::ToTerm(None) => t.is_none(),
            MatchableTerm::AnyTerm => true,
        }
    }
}
