use std::marker::PhantomData;
use std::cmp::Ordering;

use once_cell::unsync::OnceCell;
use std::collections::BTreeSet;



pub struct OnceTreeSet<I, A, B, C, D>
where I: Identifier, A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition 
{
    v: OnceCell<BTreeSet<Block<I, A, B, C, D>>>,
}

impl<I, A, B, C, D> QueryableTree4<I> for OnceTreeSet<I, A, B, C, D>
where I: Identifier, A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition 
{
    fn new() -> Self {
        Self { v: OnceCell::new() }
    }

    fn new_instanciated() -> Self {
        Self {
            v: {
                let x = OnceCell::<BTreeSet<Block<I, A, B, C, D>>>::new();
                x.set(BTreeSet::new()).ok();
                x
            }
        }
    }

    fn exists(&self) -> bool {
        self.v.get().is_some()
    }

    pub fn ensure_exists<T>(&mut self, f: F)
    where F: FnOnce() -> impl TreeIterator<'a, I>
    {
        self.v.get_or_init(
            move || {
                let mut tree = BTreeSet::<Block<I, A, B, C, D>>::new();

                for id_quad in f() {
                    tree.insert(Block::<I, A, B, C, D> { 
                        values: id_quad,
                        _boilerplate: PhantomData{}
                    });
                }

                tree
            }
        );
    }

    pub fn get_quads<'a>(&'& self, pattern: [Option<I>; 4]) -> Box<dyn TreeIterator4<'a, I>> {
        Bow::new(
            OnceTreeSetIterator::new(
                self.v.get().unwrap(),
                FixedOrder4::<A, B, C, D>::range(pattern)
            )
        )
    }

    pub fn index_conformance(&self, can_build: bool, pattern_layout: &[Option<I>; 4]) -> Option<usize> {
        if !can_build && self.v.get().is_none() {
            None
        } else {
            Some(FixedOrder4::<A, B, C, D>::index_conformance(pattern_layout))
        }
    }


    pub fn insert(&mut self, id_quad: &[I; 4]) -> Option<bool> {
        if let Some(inst) = self.v.get_mut() {
            Some(inst.insert(Block::<I, A, B, C, D> { 
                values: *id_quad,
                _boilerplate: PhantomData{}
            }))
        } else {
            None
        }
    }

    pub fn size(&self) -> Option<usize> {
        if let Some(inst) = self.v.get() {
            Some(inst.len())
        } else {
            None
        }

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
    where A: BlockPosition, B: BlockPosition, C: BlockPosition, D: BlockPosition 
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


impl<'a, I> Iterator for OnceTreeSetIterator<'a, I>
where I: Identifier {
    type Item = [I; 4];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.range.next();

            match next.as_ref() {
                None => {
                    return None;
                }
                Some(block) => {
                    if pattern_match(&block, &self.filter_block) {
                        return Some(**block)
                    }
                }
            }
        }
    }
}


