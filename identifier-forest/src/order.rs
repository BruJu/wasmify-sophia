


/// A position in a list of identifiers. Used to design which order to use
pub trait BlockPosition {
    const VALUE: usize;
    const NAME: &'static str;
}

/// The subject position in a SPO RDF triple / SPOG RDF quad
pub struct Subject {}
impl BlockPosition for Subject {
    const VALUE: usize = 0;
    const NAME: &'static str = "Subject";
}

/// The predicate position in a SPO RDF triple / SPOG RDF quad
pub struct Predicate {}
impl BlockPosition for Predicate {
    const VALUE: usize = 1;
    const NAME: &'static str = "Predicate";
}

/// The object position in a SPO RDF triple / SPOG RDF quad
pub struct Object {}
impl BlockPosition for Object {
    const VALUE: usize = 2;
    const NAME: &'static str = "Object";
}

/// The graph position in a SPOG RDF quad
pub struct Graph {}
impl BlockPosition for Graph {
    const VALUE: usize = 3;
    const NAME: &'static str = "Graph";
}


// ============================================================================
// ============================================================================

pub trait Identifier : Ord + Copy {
    const MIN: Self;
    const MAX: Self;
}

impl Identifier for u32 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
}


// ============================================================================
// ============================================================================


/// An utility struct that provides method to manipulate quads sorted in the
/// ABCD order
pub struct FixedOrder4<A, B, C, D>
where A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition {
    _a: PhantomData<*const A>,
    _b: PhantomData<*const B>,
    _c: PhantomData<*const C>,
    _d: PhantomData<*const D>,
}


impl<A, B, C, D> FixedOrder4<A, B, C, D>
where A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition {
    const NB_OF_TERMS: usize = 4;

    pub fn name() -> String {
        debug_assert!(Self::NB_OF_TERMS == 4);
        format!("{:?} {:?} {:?} {:?}", A::VALUE, B::VALUE, C::VALUE, D::VALUE)
    }

    pub fn to_slice() -> [usize; 4] {
        [A::VALUE, B::VALUE, C::VALUE, D::VALUE]
    }
    
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
    
    /// Return a range on every block that matches the given identifier quad
    /// pattern (assuming the lexicographical order).
    /// The range is restricted as much as possible, but extra quads
    /// that do not match the pattern may be included (best effort).
    /// To let the user filter the extra quads, a filter block is also
    /// returned.
    pub fn range<T>(
        identifier_quad_pattern: [Option<T>; 4],
    ) -> (std::ops::RangeInclusive<Block<T, A, B, C, D>>, [Option<T>; 4])     
    where T: Identifier
    {
        // Restrict range as much as possible
        let mut min = Block::<T, A, B, C, D>{ values : [T::MIN; 4], _boilerplate: PhantomData{} };
        let mut max = Block::<T, A, B, C, D>{ values : [T::MAX; 4], _boilerplate: PhantomData{} };

        let term_roles = Self::to_slice();

        for term_role in term_roles.iter() {
            match identifier_quad_pattern[*term_role] {
                None => {
                    break;
                }
                Some(set_value) => {
                    min.values[*term_role] = set_value;
                    max.values[*term_role] = set_value;
                }
            }
        }

        // Return range + filter block
        (
            min..=max,
            identifier_quad_pattern,
        )
    }
}




// ============================================================================
// ============================================================================


/// Wrapper for an array of 4 Ts, ordered by A then B then C the nD
pub struct Block<T, A, B, C, D>
where T: Identifier, A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition 
{
    values: [T; 4],
    _boilerplate: PhantomData<*const FixedOrder4<A, B, C, D>>,
}

impl<T, A, B, C, D> PartialOrd for Block<T, A, B, C, D>
where T: Identifier, A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, A, B, C, D> Ord for Block<T, A, B, C, D>
where T: Identifier, A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition 
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        FixedOrder4::<A, B, C, D>::compare(&self.values, &other.values)
    }
}

impl<T, A, B, C, D> Eq for Block<T, A, B, C, D>
where T: Identifier, A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition 
{}

impl<T, A, B, C, D> PartialEq for Block<T, A, B, C, D>
where T: Identifier, A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition 
{
    fn eq(&self, other: &Self) -> bool {
        &self.values == &other.values
    }
}

// ============================================================================
// ============================================================================

/// Returns true if the non None values of the given filter_block are equals
/// to the values of this block
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

        //let o = FirstOrder::to_dynamic();
//
        //assert!(o.compare(&m1234, &m1234) == std::cmp::Ordering::Equal);
        //assert!(o.compare(&m1234, &m1324) == std::cmp::Ordering::Less);
        //assert!(o.compare(&m1324, &m1234) == std::cmp::Ordering::Greater);


        type SecondOrder = FixedOrder4<Subject, Predicate, Graph, Object>;

        assert!(SecondOrder::compare(&m1234, &m1234) == std::cmp::Ordering::Equal);
        assert!(SecondOrder::compare(&m1234, &m1324) == std::cmp::Ordering::Greater);
        assert!(SecondOrder::compare(&m1324, &m1234) == std::cmp::Ordering::Less);
    }


}


