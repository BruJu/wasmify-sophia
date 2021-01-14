use std::marker::PhantomData;
use std::cmp::Ordering;

use once_cell::unsync::OnceCell;
use std::collections::BTreeSet;

// !!!!!!!!!!!!!!!
// This file will be deleted soon as its last features
// are copied / adapted in tree_predefined.rs


pub struct OnceTreeSet<I, A, B, C, D>
where I: Identifier, A: Position, B: Position, C: Position, D: Position 
{
    v: OnceCell<BTreeSet<Block<I, A, B, C, D>>>,
}

impl<I, A, B, C, D> OnceTreeSet<I, A, B, C, D>
where I: Identifier, A: Position, B: Position, C: Position, D: Position 
{
    pub fn new() -> Self {
        Self { v: OnceCell::new() }
    }

    pub fn new_instanciated() -> Self {
        Self {
            v: {
                let x = OnceCell::<BTreeSet<Block<I, A, B, C, D>>>::new();
                x.set(BTreeSet::new()).ok();
                x
            }
        }
    }

    pub fn exists(&self) -> bool {
        self.v.get().is_some()
    }

    pub fn get_quads<'a>(&'a self, pattern: [Option<I>; 4]) -> OnceTreeSetIterator<'a, I> {
        OnceTreeSetIterator::new(
            self.v.get().unwrap(),
            FixedOrder4::<A, B, C, D>::range(pattern)
        )
    }

    pub fn index_conformance(&self, can_build: bool, pattern_layout: &[Option<I>; 4]) -> Option<usize> {
        if !can_build && self.v.get().is_none() {
            None
        } else {
            Some(FixedOrder4::<A, B, C, D>::index_conformance(*pattern_layout))
        }
    }

    pub fn initialize<'a>(&self, iter: OnceTreeSetIterator<'a, I>) {

    }
}


pub struct OnceTreeSetIterator<'a, I>
where I: Identifier
{
    range: std::collections::btree_set::Range<'a, [I; 4]>,
    filter_block: [Option<I>; 4]
}


impl<'a, I> OnceTreeSetIterator<'a, I>
where I: Identifier
{
    

    pub fn new<A, B, C, D>(
        tree: &'a BTreeSet<Block<I, A, B, C, D>>,
        things: (std::ops::RangeInclusive<Block<I, A, B, C, D>>, [Option<I>; 4])
    )
    -> Self
    where A: Position, B: Position, C: Position, D: Position 
    {
        let range = tree.range(things.0);

        let range_rover = unsafe {
            std::mem::transmute::<
                std::collections::btree_set::Range<'a, Block<I, A, B, C, D>>,
                std::collections::btree_set::Range<'a, [I; 4]>
            >(range)
        };

        Self {
            range: range_rover,
            filter_block: things.1
        }
    }

}





//pub struct Order4 {
//    order: [usize; 4]
//}
//
//impl Order4 {
//    fn compare<T>(&self, lhs: &[T; 4], rhs: &[T; 4]) -> std::cmp::Ordering
//    where T: Ord + Copy
//    {   
//        (&lhs[self.order[0]]).cmp(&rhs[self.order[0]])
//        .then_with(|| (&lhs[self.order[1]]).cmp(&rhs[self.order[1]]))
//        .then_with(|| (&lhs[self.order[2]]).cmp(&rhs[self.order[2]]))
//        .then_with(|| (&lhs[self.order[3]]).cmp(&rhs[self.order[3]]))
//    }
//}





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


