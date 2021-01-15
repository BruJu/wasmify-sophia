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
        for term_role in Self::to_slice().iter() {
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

/// Returns true if the non None values of the given pattern are equals
/// to the values in quad. In other words, it can be seen as an
/// equality check where none serves as a wildcard.
pub fn pattern_match<T>(quad: &[T; 4], pattern: &[Option<T>; 4]) -> bool
where T: Identifier {
    quad.iter()
        .zip(pattern.iter())
        .all(|(val, opt)| match opt {
            None => true,
            Some(other) => val == other,
        }) 
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

    #[test]
    fn test_pattern_matching() {
        // Match
        assert!(pattern_match(&[556, 120, 325, 636], &[Some(556), Some(120), Some(325), Some(636)]));
        assert!(pattern_match(&[816, 302, 514, 288], &[None, None, None, Some(288)]));
        assert!(pattern_match(&[557, 846, 923, 37], &[None, None, Some(923), Some(37)]));
        assert!(pattern_match(&[771, 941, 722, 703], &[None, None, None, Some(703)]));
        assert!(pattern_match(&[940, 774, 170, 581], &[None, Some(774), Some(170), Some(581)]));
        assert!(pattern_match(&[790, 57, 727, 781], &[None, None, None, Some(781)]));
        assert!(pattern_match(&[432, 14, 851, 802], &[None, None, Some(851), Some(802)]));
        assert!(pattern_match(&[478, 379, 842, 174], &[None, None, None, Some(174)]));
        assert!(pattern_match(&[665, 274, 228, 298], &[Some(665), Some(274), Some(228), Some(298)]));
        assert!(pattern_match(&[907, 975, 968, 416], &[None, None, None, Some(416)]));
        assert!(pattern_match(&[225, 883, 971, 545], &[None, None, Some(971), Some(545)]));
        assert!(pattern_match(&[327, 699, 549, 540], &[None, None, None, Some(540)]));
        assert!(pattern_match(&[998, 717, 753, 607], &[None, Some(717), Some(753), Some(607)]));
        assert!(pattern_match(&[850, 630, 951, 618], &[None, None, None, Some(618)]));
        assert!(pattern_match(&[169, 181, 614, 514], &[None, None, Some(614), Some(514)]));
        assert!(pattern_match(&[924, 420, 423, 370], &[None, None, None, Some(370)]));

        // Do not match
        assert!(!pattern_match(&[4456, 92, 545, 635], &[Some(534), Some(654), Some(564), Some(847)]));
        assert!(!pattern_match(&[534, 92, 545, 635], &[Some(534), Some(654), Some(564), Some(847)]));
        assert!(!pattern_match(&[761, 633, 613, 857], &[None, None, None, Some(459)]));
        assert!(!pattern_match(&[443, 554, 54, 999], &[None, None, Some(54), Some(359)]));
        assert!(!pattern_match(&[752, 324, 152, 477], &[None, None, None, Some(982)]));
        assert!(!pattern_match(&[390, 964, 345, 528], &[None, Some(964), Some(345), Some(863)]));
        assert!(!pattern_match(&[976, 218, 55, 279], &[None, None, None, Some(684)]));
        assert!(!pattern_match(&[520, 831, 47, 285], &[None, None, Some(144), Some(470)]));
        assert!(!pattern_match(&[818, 987, 703, 313], &[None, None, None, Some(463)]));
        assert!(!pattern_match(&[341, 456, 889, 403], &[Some(341), Some(200), Some(940), Some(584)]));
        assert!(!pattern_match(&[560, 140, 359, 673], &[None, None, Some(138), Some(94)]));
        assert!(!pattern_match(&[631, 498, 187, 470], &[None, None, None, Some(431)]));
        assert!(!pattern_match(&[610, 142, 406, 410], &[None, Some(142), Some(983), Some(464)]));
        assert!(!pattern_match(&[725, 105, 474, 274], &[None, None, None, Some(348)]));
        assert!(!pattern_match(&[546, 718, 513, 238], &[None, None, Some(359), Some(238)]));
        assert!(!pattern_match(&[565, 466, 157, 41], &[None, None, None, Some(952)]));
        assert!(!pattern_match(&[598, 76, 513, 307], &[None, None, None, Some(485)]));
        assert!(!pattern_match(&[28, 108, 582, 134], &[None, None, Some(994), Some(134)]));
        assert!(!pattern_match(&[369, 325, 189, 863], &[None, None, None, Some(351)]));
        assert!(!pattern_match(&[852, 23, 430, 292], &[None, Some(23), Some(882), Some(292)]));
        assert!(!pattern_match(&[445, 355, 836, 184], &[None, None, Some(570), Some(278)]));
        assert!(!pattern_match(&[48, 113, 784, 609], &[None, None, None, Some(464)]));
        assert!(!pattern_match(&[149, 402, 66, 687], &[None, None, None, Some(173)]));
        assert!(!pattern_match(&[755, 502, 905, 361], &[None, None, Some(905), Some(61)]));
        assert!(!pattern_match(&[762, 607, 263, 493], &[None, None, None, Some(677)]));
        assert!(!pattern_match(&[655, 815, 951, 172], &[None, None, Some(282), Some(72)]));
    }

    // TODO: test range, index_conformnce, to_slice, valid and invalid orders

}


