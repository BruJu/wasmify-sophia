//! This file defines how an array of four identifiers can be ordered

use crate::order::Position;
use crate::Identifier;

use std::cmp::Ordering;
use std::marker::PhantomData;


/// An utility struct that provides static method to manipulate quads sorted in
/// the ABCD order
pub struct FixedOrder4<A, B, C, D>
where A: Position, B: Position, C: Position, D: Position {
    _a: PhantomData<*const (A, B, C, D)>,
}

impl<A, B, C, D> FixedOrder4<A, B, C, D>
where A: Position, B: Position, C: Position, D: Position {
    // Number of generic types.
    // TODO: use it
    // const NB_OF_TERMS: usize = 4;

    /// Returns the list of indexes described by the generic parameters
    pub fn to_slice() -> [usize; 4] {
        [A::VALUE, B::VALUE, C::VALUE, D::VALUE]
    }
    
    /// Compares two arrays of four orderable values by their `A::VALUE`-th
    /// term, then their `B::VALUE`-th term, then their `C::VALUE`-th term,
    /// then their `D::VALUE`-th term.
    pub fn compare<T>(lhs: &[T; 4], rhs: &[T; 4]) -> std::cmp::Ordering 
    where T: Ord
    {
        (&lhs[A::VALUE]).cmp(&rhs[A::VALUE])
            .then_with(|| (&lhs[B::VALUE]).cmp(&rhs[B::VALUE]))
            .then_with(|| (&lhs[C::VALUE]).cmp(&rhs[C::VALUE]))
            .then_with(|| (&lhs[D::VALUE]).cmp(&rhs[D::VALUE]))
    }

    /// Return the number of term roles that can be used as a prefix, with this
    /// block order, to filter the quads matching the given pattern.
    ///
    /// This gives an indication of how efficient the
    /// [`filter`](BlockOrder::filter) method will be.
    /// The higher, the better suited this block order is to answer this pattern.
    pub fn index_conformance<T>(pattern: &[Option<T>; 4]) -> usize {
        Self::to_slice()
            .iter()
            .take_while(|tr| pattern[**tr as usize].is_some())
            .count()
    }
    
    /// Assuming that we have a structure ordered by the order A, B, C, D,
    /// and that we want to extract every array that matches the given
    /// pattern (None is a wildcard, else we want the specified value),
    /// returns the smallest possible range to retrieve every arrays from the
    /// structure (best effort) and a new pattern which contains values only
    /// on non wildcard indexes that are not already filtered by the range.
    pub fn range<T>(
        pattern: [Option<T>; 4],
    ) -> (std::ops::RangeInclusive<Block<T, A, B, C, D>>, [Option<T>; 4])     
    where T: Identifier
    {
        // Initial range (checks every values)
        let mut min = Block::<T, A, B, C, D>::new([T::MIN; 4]);
        let mut max = Block::<T, A, B, C, D>::new([T::MAX; 4]);

        // Restrict it as much as possible
        for term_role in Self::to_slice() {
            match pattern[*term_role] {
                None => {
                    break;
                }
                Some(set_value) => {
                    min.values[*term_role] = set_value;
                    max.values[*term_role] = set_value;
                }
            }
        }

        // Return the range and the filter block
        (
            min..=max,
            pattern,
        )
    }
}

// ============================================================================
// ============================================================================

/// Wrapper for an array of 4 values of the same type whose purpose is to
/// provide a compile time defined order on these arrays.
///
/// If the generic type `I` is an [`Identifier`], and the `A`, `B`, `C` and `D`
/// generics types are Identifiers, then ordered by A then B then C the nD
///
/// **TODO:** Compile time check if A != B != C != D and 0 < [A, B, C, D] < 4
pub struct Block<I, A, B, C, D>
where I: Identifier, A: Position, B: Position, C: Position, D: Position 
{
    /// The wrapped array of four values.
    pub values: [I; 4],
    _boilerplate: PhantomData<*const FixedOrder4<A, B, C, D>>,
}

impl<I, A, B, C, D> Block<I, A, B, C, D>
where I: Identifier, A: Position, B: Position, C: Position, D: Position 
{
    /// Wraps an array of four values
    pub fn new(elements: [I; 4]) -> Self {
        Self {
            values: elements,
            _boilerplate: PhantomData{}
        }
    }
}

/// The order on [`Block`]s is a total order.
impl<T, A, B, C, D> PartialOrd for Block<T, A, B, C, D>
where T: Identifier, A: Position, B: Position, C: Position, D: Position 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares two arrays of four [`Identifier`]s using the A::VALUE-th term.
/// If it is equals, compares the B::VALUE-th term, ... until the
/// D::VALUE-th term.
impl<T, A, B, C, D> Ord for Block<T, A, B, C, D>
where T: Identifier, A: Position, B: Position, C: Position, D: Position 
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        FixedOrder4::<A, B, C, D>::compare(&self.values, &other.values)
    }
}

impl<T, A, B, C, D> Eq for Block<T, A, B, C, D>
where T: Identifier, A: Position, B: Position, C: Position, D: Position 
{}

impl<T, A, B, C, D> PartialEq for Block<T, A, B, C, D>
where T: Identifier, A: Position, B: Position, C: Position, D: Position 
{
    fn eq(&self, other: &Self) -> bool {
        // Order of comparison is not important here
        &self.values == &other.values
    }
}

// ============================================================================
// ============================================================================

/// Returns true if the non None values of the given filter_block are equals
/// to the values of this block. In other words, it can be seen as an
/// equality check where none serves as a wildcard.
pub fn pattern_match<T>(block: &[T; 4], pattern: &[Option<T>; 4]) -> bool
where T: Identifier {
    for i in 0..block.len() {
        if let Some(filter_data) = pattern[i].as_ref() {
            if block[i] != *filter_data {
                return false;
            }
        }
    }

    true
}


// ============================================================================


#[cfg(test)]
mod test {
    use crate::order::{Subject, Predicate, Object, Graph};
    use super::*;

    #[test]
    fn order4() {

        assert!(
            std::mem::size_of::<Block<u32, Subject, Predicate, Object, Graph>>() ==
            std::mem::size_of::<[u32; 4]>()
        );

        type FirstOrder = FixedOrder4<Subject, Predicate, Object, Graph>;

        let m1234 = [1, 2, 3, 4];
        let m1324 = [1, 2, 4, 3];

        assert!(FirstOrder::compare(&m1234, &m1234) == std::cmp::Ordering::Equal);
        assert!(FirstOrder::compare(&m1234, &m1324) == std::cmp::Ordering::Less);
        assert!(FirstOrder::compare(&m1324, &m1234) == std::cmp::Ordering::Greater);

        type SecondOrder = FixedOrder4<Subject, Predicate, Graph, Object>;

        assert!(SecondOrder::compare(&m1234, &m1234) == std::cmp::Ordering::Equal);
        assert!(SecondOrder::compare(&m1234, &m1324) == std::cmp::Ordering::Greater);
        assert!(SecondOrder::compare(&m1324, &m1234) == std::cmp::Ordering::Less);
    }

    // TODO: test range, index_conformnce, to_slice, valid and invalid orders

}


