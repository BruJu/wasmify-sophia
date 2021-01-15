use crate::tree::OnceTreeSet;
use crate::order::{ Subject, Predicate, Object, Graph };
use crate::tree::{ Tree4Iterator, LazyStructure, MaybeTree4 };
use crate::Identifier;

/// A MaybeTree4 implementation whose order is decided at execution time.
///
/// The choice is made by using the appropriate constructor with the position order
/// in the constructor instead of generic parameters. The appropriate OnceTreeSet
/// will be created depending on the given order, and then this structure will act
/// as the desired OnceTreeSet by forwarding the calls to the different methods.
pub enum DynamicOnceTreeSet<I>
where I: Identifier
{
    SPOG(OnceTreeSet<I, Subject, Predicate, Object, Graph>),
    SPGO(OnceTreeSet<I, Subject, Predicate, Graph, Object>),
    SOPG(OnceTreeSet<I, Subject, Object, Predicate, Graph>),
    SOGP(OnceTreeSet<I, Subject, Object, Graph, Predicate>),
    SGPO(OnceTreeSet<I, Subject, Graph, Predicate, Object>),
    SGOP(OnceTreeSet<I, Subject, Graph, Object, Predicate>),
    PSOG(OnceTreeSet<I, Predicate, Subject, Object, Graph>),
    PSGO(OnceTreeSet<I, Predicate, Subject, Graph, Object>),
    POSG(OnceTreeSet<I, Predicate, Object, Subject, Graph>),
    POGS(OnceTreeSet<I, Predicate, Object, Graph, Subject>),
    PGSO(OnceTreeSet<I, Predicate, Graph, Subject, Object>),
    PGOS(OnceTreeSet<I, Predicate, Graph, Object, Subject>),
    OSPG(OnceTreeSet<I, Object, Subject, Predicate, Graph>),
    OSGP(OnceTreeSet<I, Object, Subject, Graph, Predicate>),
    OPSG(OnceTreeSet<I, Object, Predicate, Subject, Graph>),
    OPGS(OnceTreeSet<I, Object, Predicate, Graph, Subject>),
    OGSP(OnceTreeSet<I, Object, Graph, Subject, Predicate>),
    OGPS(OnceTreeSet<I, Object, Graph, Predicate, Subject>),
    GSPO(OnceTreeSet<I, Graph, Subject, Predicate, Object>),
    GSOP(OnceTreeSet<I, Graph, Subject, Object, Predicate>),
    GPSO(OnceTreeSet<I, Graph, Predicate, Subject, Object>),
    GPOS(OnceTreeSet<I, Graph, Predicate, Object, Subject>),
    GOSP(OnceTreeSet<I, Graph, Object, Subject, Predicate>),
    GOPS(OnceTreeSet<I, Graph, Object, Predicate, Subject>),
}


impl<I> DynamicOnceTreeSet<I>
where I: Identifier
{
    /// Builds a new TreeSet whose order is defined at execution time. The tree
    /// is not directly built (the underlying used constructor is the new function
    /// from the OnceTreeSet class)
    /// 
    /// See OnceTreeSet for more details
    pub fn new(order: &[usize; 4]) -> Option<DynamicOnceTreeSet<I>> {
        match order {
            [0, 1, 2, 3] => Some(Self::SPOG( OnceTreeSet::new() )),
            [0, 1, 3, 2] => Some(Self::SPGO( OnceTreeSet::new() )),
            [0, 2, 1, 3] => Some(Self::SOPG( OnceTreeSet::new() )),
            [0, 2, 3, 1] => Some(Self::SOGP( OnceTreeSet::new() )),
            [0, 3, 1, 2] => Some(Self::SGPO( OnceTreeSet::new() )),
            [0, 3, 2, 1] => Some(Self::SGOP( OnceTreeSet::new() )),
            [1, 0, 2, 3] => Some(Self::PSOG( OnceTreeSet::new() )),
            [1, 0, 3, 2] => Some(Self::PSGO( OnceTreeSet::new() )),
            [1, 2, 0, 3] => Some(Self::POSG( OnceTreeSet::new() )),
            [1, 2, 3, 0] => Some(Self::POGS( OnceTreeSet::new() )),
            [1, 3, 0, 2] => Some(Self::PGSO( OnceTreeSet::new() )),
            [1, 3, 2, 0] => Some(Self::PGOS( OnceTreeSet::new() )),
            [2, 0, 1, 3] => Some(Self::OSPG( OnceTreeSet::new() )),
            [2, 0, 3, 1] => Some(Self::OSGP( OnceTreeSet::new() )),
            [2, 1, 0, 3] => Some(Self::OPSG( OnceTreeSet::new() )),
            [2, 1, 3, 0] => Some(Self::OPGS( OnceTreeSet::new() )),
            [2, 3, 0, 1] => Some(Self::OGSP( OnceTreeSet::new() )),
            [2, 3, 1, 0] => Some(Self::OGPS( OnceTreeSet::new() )),
            [3, 0, 1, 2] => Some(Self::GSPO( OnceTreeSet::new() )),
            [3, 0, 2, 1] => Some(Self::GSOP( OnceTreeSet::new() )),
            [3, 1, 0, 2] => Some(Self::GPSO( OnceTreeSet::new() )),
            [3, 1, 2, 0] => Some(Self::GPOS( OnceTreeSet::new() )),
            [3, 2, 0, 1] => Some(Self::GOSP( OnceTreeSet::new() )),
            [3, 2, 1, 0] => Some(Self::GOPS( OnceTreeSet::new() )),
            [_, _, _, _] => None,
        }
    }

    /// Builds a new TreeSet whose order is defined at execution time. The tree
    /// is directly built and ready for usage (according to new_instanciated
    /// specificaiton)
    /// 
    /// See OnceTreeSet for more details
    pub fn new_instanciated(order: &[usize; 4]) -> Option<DynamicOnceTreeSet<I>> {
        match order {
            [0, 1, 2, 3] => Some(Self::SPOG( OnceTreeSet::new_instanciated() )),
            [0, 1, 3, 2] => Some(Self::SPGO( OnceTreeSet::new_instanciated() )),
            [0, 2, 1, 3] => Some(Self::SOPG( OnceTreeSet::new_instanciated() )),
            [0, 2, 3, 1] => Some(Self::SOGP( OnceTreeSet::new_instanciated() )),
            [0, 3, 1, 2] => Some(Self::SGPO( OnceTreeSet::new_instanciated() )),
            [0, 3, 2, 1] => Some(Self::SGOP( OnceTreeSet::new_instanciated() )),
            [1, 0, 2, 3] => Some(Self::PSOG( OnceTreeSet::new_instanciated() )),
            [1, 0, 3, 2] => Some(Self::PSGO( OnceTreeSet::new_instanciated() )),
            [1, 2, 0, 3] => Some(Self::POSG( OnceTreeSet::new_instanciated() )),
            [1, 2, 3, 0] => Some(Self::POGS( OnceTreeSet::new_instanciated() )),
            [1, 3, 0, 2] => Some(Self::PGSO( OnceTreeSet::new_instanciated() )),
            [1, 3, 2, 0] => Some(Self::PGOS( OnceTreeSet::new_instanciated() )),
            [2, 0, 1, 3] => Some(Self::OSPG( OnceTreeSet::new_instanciated() )),
            [2, 0, 3, 1] => Some(Self::OSGP( OnceTreeSet::new_instanciated() )),
            [2, 1, 0, 3] => Some(Self::OPSG( OnceTreeSet::new_instanciated() )),
            [2, 1, 3, 0] => Some(Self::OPGS( OnceTreeSet::new_instanciated() )),
            [2, 3, 0, 1] => Some(Self::OGSP( OnceTreeSet::new_instanciated() )),
            [2, 3, 1, 0] => Some(Self::OGPS( OnceTreeSet::new_instanciated() )),
            [3, 0, 1, 2] => Some(Self::GSPO( OnceTreeSet::new_instanciated() )),
            [3, 0, 2, 1] => Some(Self::GSOP( OnceTreeSet::new_instanciated() )),
            [3, 1, 0, 2] => Some(Self::GPSO( OnceTreeSet::new_instanciated() )),
            [3, 1, 2, 0] => Some(Self::GPOS( OnceTreeSet::new_instanciated() )),
            [3, 2, 0, 1] => Some(Self::GOSP( OnceTreeSet::new_instanciated() )),
            [3, 2, 1, 0] => Some(Self::GOPS( OnceTreeSet::new_instanciated() )),
            [_, _, _, _] => None,
        }
    }

}

impl<I> MaybeTree4<I> for DynamicOnceTreeSet<I>
where I: Identifier
{
    fn exists(&self) -> bool {
        match &self {
            Self::SPOG(tree) => tree.exists(),
            Self::SPGO(tree) => tree.exists(),
            Self::SOPG(tree) => tree.exists(),
            Self::SOGP(tree) => tree.exists(),
            Self::SGPO(tree) => tree.exists(),
            Self::SGOP(tree) => tree.exists(),
            Self::PSOG(tree) => tree.exists(),
            Self::PSGO(tree) => tree.exists(),
            Self::POSG(tree) => tree.exists(),
            Self::POGS(tree) => tree.exists(),
            Self::PGSO(tree) => tree.exists(),
            Self::PGOS(tree) => tree.exists(),
            Self::OSPG(tree) => tree.exists(),
            Self::OSGP(tree) => tree.exists(),
            Self::OPSG(tree) => tree.exists(),
            Self::OPGS(tree) => tree.exists(),
            Self::OGSP(tree) => tree.exists(),
            Self::OGPS(tree) => tree.exists(),
            Self::GSPO(tree) => tree.exists(),
            Self::GSOP(tree) => tree.exists(),
            Self::GPSO(tree) => tree.exists(),
            Self::GPOS(tree) => tree.exists(),
            Self::GOSP(tree) => tree.exists(),
            Self::GOPS(tree) => tree.exists(),
        }
    }

    fn ensure_exists<'a, F>(&self, f: F) where F: FnOnce() -> Tree4Iterator<'a, I> {
        match &self {
            Self::SPOG(tree) => tree.ensure_exists(f),
            Self::SPGO(tree) => tree.ensure_exists(f),
            Self::SOPG(tree) => tree.ensure_exists(f),
            Self::SOGP(tree) => tree.ensure_exists(f),
            Self::SGPO(tree) => tree.ensure_exists(f),
            Self::SGOP(tree) => tree.ensure_exists(f),
            Self::PSOG(tree) => tree.ensure_exists(f),
            Self::PSGO(tree) => tree.ensure_exists(f),
            Self::POSG(tree) => tree.ensure_exists(f),
            Self::POGS(tree) => tree.ensure_exists(f),
            Self::PGSO(tree) => tree.ensure_exists(f),
            Self::PGOS(tree) => tree.ensure_exists(f),
            Self::OSPG(tree) => tree.ensure_exists(f),
            Self::OSGP(tree) => tree.ensure_exists(f),
            Self::OPSG(tree) => tree.ensure_exists(f),
            Self::OPGS(tree) => tree.ensure_exists(f),
            Self::OGSP(tree) => tree.ensure_exists(f),
            Self::OGPS(tree) => tree.ensure_exists(f),
            Self::GSPO(tree) => tree.ensure_exists(f),
            Self::GSOP(tree) => tree.ensure_exists(f),
            Self::GPSO(tree) => tree.ensure_exists(f),
            Self::GPOS(tree) => tree.ensure_exists(f),
            Self::GOSP(tree) => tree.ensure_exists(f),
            Self::GOPS(tree) => tree.ensure_exists(f),
        }
    }

    fn get_quads<'a>(&'a self, pattern: [Option<I>; 4]) -> Tree4Iterator<'a, I> {
        match &self {
            Self::SPOG(tree) => tree.get_quads(pattern),
            Self::SPGO(tree) => tree.get_quads(pattern),
            Self::SOPG(tree) => tree.get_quads(pattern),
            Self::SOGP(tree) => tree.get_quads(pattern),
            Self::SGPO(tree) => tree.get_quads(pattern),
            Self::SGOP(tree) => tree.get_quads(pattern),
            Self::PSOG(tree) => tree.get_quads(pattern),
            Self::PSGO(tree) => tree.get_quads(pattern),
            Self::POSG(tree) => tree.get_quads(pattern),
            Self::POGS(tree) => tree.get_quads(pattern),
            Self::PGSO(tree) => tree.get_quads(pattern),
            Self::PGOS(tree) => tree.get_quads(pattern),
            Self::OSPG(tree) => tree.get_quads(pattern),
            Self::OSGP(tree) => tree.get_quads(pattern),
            Self::OPSG(tree) => tree.get_quads(pattern),
            Self::OPGS(tree) => tree.get_quads(pattern),
            Self::OGSP(tree) => tree.get_quads(pattern),
            Self::OGPS(tree) => tree.get_quads(pattern),
            Self::GSPO(tree) => tree.get_quads(pattern),
            Self::GSOP(tree) => tree.get_quads(pattern),
            Self::GPSO(tree) => tree.get_quads(pattern),
            Self::GPOS(tree) => tree.get_quads(pattern),
            Self::GOSP(tree) => tree.get_quads(pattern),
            Self::GOPS(tree) => tree.get_quads(pattern),
        }
    }

    fn index_conformance(&self, can_build: bool, pattern_layout: &[Option<I>; 4]) -> Option<usize> {
        match &self {
            Self::SPOG(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::SPGO(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::SOPG(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::SOGP(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::SGPO(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::SGOP(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::PSOG(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::PSGO(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::POSG(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::POGS(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::PGSO(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::PGOS(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::OSPG(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::OSGP(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::OPSG(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::OPGS(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::OGSP(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::OGPS(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::GSPO(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::GSOP(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::GPSO(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::GPOS(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::GOSP(tree) => tree.index_conformance(can_build, &pattern_layout),
            Self::GOPS(tree) => tree.index_conformance(can_build, &pattern_layout),
        }
    }

    fn insert(&mut self, id_quad: &[I; 4]) -> Option<bool> {
        match self {
            Self::SPOG(tree) => tree.insert(&id_quad),
            Self::SPGO(tree) => tree.insert(&id_quad),
            Self::SOPG(tree) => tree.insert(&id_quad),
            Self::SOGP(tree) => tree.insert(&id_quad),
            Self::SGPO(tree) => tree.insert(&id_quad),
            Self::SGOP(tree) => tree.insert(&id_quad),
            Self::PSOG(tree) => tree.insert(&id_quad),
            Self::PSGO(tree) => tree.insert(&id_quad),
            Self::POSG(tree) => tree.insert(&id_quad),
            Self::POGS(tree) => tree.insert(&id_quad),
            Self::PGSO(tree) => tree.insert(&id_quad),
            Self::PGOS(tree) => tree.insert(&id_quad),
            Self::OSPG(tree) => tree.insert(&id_quad),
            Self::OSGP(tree) => tree.insert(&id_quad),
            Self::OPSG(tree) => tree.insert(&id_quad),
            Self::OPGS(tree) => tree.insert(&id_quad),
            Self::OGSP(tree) => tree.insert(&id_quad),
            Self::OGPS(tree) => tree.insert(&id_quad),
            Self::GSPO(tree) => tree.insert(&id_quad),
            Self::GSOP(tree) => tree.insert(&id_quad),
            Self::GPSO(tree) => tree.insert(&id_quad),
            Self::GPOS(tree) => tree.insert(&id_quad),
            Self::GOSP(tree) => tree.insert(&id_quad),
            Self::GOPS(tree) => tree.insert(&id_quad),
        }
    }

    fn delete(&mut self, id_quad: &[I; 4]) -> Option<bool> {
        match self {
            Self::SPOG(tree) => tree.delete(&id_quad),
            Self::SPGO(tree) => tree.delete(&id_quad),
            Self::SOPG(tree) => tree.delete(&id_quad),
            Self::SOGP(tree) => tree.delete(&id_quad),
            Self::SGPO(tree) => tree.delete(&id_quad),
            Self::SGOP(tree) => tree.delete(&id_quad),
            Self::PSOG(tree) => tree.delete(&id_quad),
            Self::PSGO(tree) => tree.delete(&id_quad),
            Self::POSG(tree) => tree.delete(&id_quad),
            Self::POGS(tree) => tree.delete(&id_quad),
            Self::PGSO(tree) => tree.delete(&id_quad),
            Self::PGOS(tree) => tree.delete(&id_quad),
            Self::OSPG(tree) => tree.delete(&id_quad),
            Self::OSGP(tree) => tree.delete(&id_quad),
            Self::OPSG(tree) => tree.delete(&id_quad),
            Self::OPGS(tree) => tree.delete(&id_quad),
            Self::OGSP(tree) => tree.delete(&id_quad),
            Self::OGPS(tree) => tree.delete(&id_quad),
            Self::GSPO(tree) => tree.delete(&id_quad),
            Self::GSOP(tree) => tree.delete(&id_quad),
            Self::GPSO(tree) => tree.delete(&id_quad),
            Self::GPOS(tree) => tree.delete(&id_quad),
            Self::GOSP(tree) => tree.delete(&id_quad),
            Self::GOPS(tree) => tree.delete(&id_quad),
        }
    }

    fn size(&self) -> Option<usize> {
        match &self {
            Self::SPOG(tree) => tree.size(),
            Self::SPGO(tree) => tree.size(),
            Self::SOPG(tree) => tree.size(),
            Self::SOGP(tree) => tree.size(),
            Self::SGPO(tree) => tree.size(),
            Self::SGOP(tree) => tree.size(),
            Self::PSOG(tree) => tree.size(),
            Self::PSGO(tree) => tree.size(),
            Self::POSG(tree) => tree.size(),
            Self::POGS(tree) => tree.size(),
            Self::PGSO(tree) => tree.size(),
            Self::PGOS(tree) => tree.size(),
            Self::OSPG(tree) => tree.size(),
            Self::OSGP(tree) => tree.size(),
            Self::OPSG(tree) => tree.size(),
            Self::OPGS(tree) => tree.size(),
            Self::OGSP(tree) => tree.size(),
            Self::OGPS(tree) => tree.size(),
            Self::GSPO(tree) => tree.size(),
            Self::GSOP(tree) => tree.size(),
            Self::GPSO(tree) => tree.size(),
            Self::GPOS(tree) => tree.size(),
            Self::GOSP(tree) => tree.size(),
            Self::GOPS(tree) => tree.size(),
        }
    }

    fn has(&self, id_quad: &[I; 4]) -> Option<bool> {
        match &self {
            Self::SPOG(tree) => tree.has(&id_quad),
            Self::SPGO(tree) => tree.has(&id_quad),
            Self::SOPG(tree) => tree.has(&id_quad),
            Self::SOGP(tree) => tree.has(&id_quad),
            Self::SGPO(tree) => tree.has(&id_quad),
            Self::SGOP(tree) => tree.has(&id_quad),
            Self::PSOG(tree) => tree.has(&id_quad),
            Self::PSGO(tree) => tree.has(&id_quad),
            Self::POSG(tree) => tree.has(&id_quad),
            Self::POGS(tree) => tree.has(&id_quad),
            Self::PGSO(tree) => tree.has(&id_quad),
            Self::PGOS(tree) => tree.has(&id_quad),
            Self::OSPG(tree) => tree.has(&id_quad),
            Self::OSGP(tree) => tree.has(&id_quad),
            Self::OPSG(tree) => tree.has(&id_quad),
            Self::OPGS(tree) => tree.has(&id_quad),
            Self::OGSP(tree) => tree.has(&id_quad),
            Self::OGPS(tree) => tree.has(&id_quad),
            Self::GSPO(tree) => tree.has(&id_quad),
            Self::GSOP(tree) => tree.has(&id_quad),
            Self::GPSO(tree) => tree.has(&id_quad),
            Self::GPOS(tree) => tree.has(&id_quad),
            Self::GOSP(tree) => tree.has(&id_quad),
            Self::GOPS(tree) => tree.has(&id_quad),
        }
    }

}

