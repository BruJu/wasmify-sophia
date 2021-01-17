use super::*;

pub struct SPOG {}
impl Tree4Profile for SPOG {
    type First = Subject;
    type Second = Predicate;
    type Third = Object;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct SPGO {}
impl Tree4Profile for SPGO {
    type First = Subject;
    type Second = Predicate;
    type Third = Graph;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct SOPG {}
impl Tree4Profile for SOPG {
    type First = Subject;
    type Second = Object;
    type Third = Predicate;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct SOGP {}
impl Tree4Profile for SOGP {
    type First = Subject;
    type Second = Object;
    type Third = Graph;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct SGPO {}
impl Tree4Profile for SGPO {
    type First = Subject;
    type Second = Graph;
    type Third = Predicate;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct SGOP {}
impl Tree4Profile for SGOP {
    type First = Subject;
    type Second = Graph;
    type Third = Object;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct PSOG {}
impl Tree4Profile for PSOG {
    type First = Predicate;
    type Second = Subject;
    type Third = Object;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct PSGO {}
impl Tree4Profile for PSGO {
    type First = Predicate;
    type Second = Subject;
    type Third = Graph;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct POSG {}
impl Tree4Profile for POSG {
    type First = Predicate;
    type Second = Object;
    type Third = Subject;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct POGS {}
impl Tree4Profile for POGS {
    type First = Predicate;
    type Second = Object;
    type Third = Graph;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct PGSO {}
impl Tree4Profile for PGSO {
    type First = Predicate;
    type Second = Graph;
    type Third = Subject;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct PGOS {}
impl Tree4Profile for PGOS {
    type First = Predicate;
    type Second = Graph;
    type Third = Object;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct OSPG {}
impl Tree4Profile for OSPG {
    type First = Object;
    type Second = Subject;
    type Third = Predicate;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct OSGP {}
impl Tree4Profile for OSGP {
    type First = Object;
    type Second = Subject;
    type Third = Graph;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct OPSG {}
impl Tree4Profile for OPSG {
    type First = Object;
    type Second = Predicate;
    type Third = Subject;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct OPGS {}
impl Tree4Profile for OPGS {
    type First = Object;
    type Second = Predicate;
    type Third = Graph;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct OGSP {}
impl Tree4Profile for OGSP {
    type First = Object;
    type Second = Graph;
    type Third = Subject;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct OGPS {}
impl Tree4Profile for OGPS {
    type First = Object;
    type Second = Graph;
    type Third = Predicate;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct GSPO {}
impl Tree4Profile for GSPO {
    type First = Graph;
    type Second = Subject;
    type Third = Predicate;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct GSOP {}
impl Tree4Profile for GSOP {
    type First = Graph;
    type Second = Subject;
    type Third = Object;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct GPSO {}
impl Tree4Profile for GPSO {
    type First = Graph;
    type Second = Predicate;
    type Third = Subject;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct GPOS {}
impl Tree4Profile for GPOS {
    type First = Graph;
    type Second = Predicate;
    type Third = Object;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct GOSP {}
impl Tree4Profile for GOSP {
    type First = Graph;
    type Second = Object;
    type Third = Subject;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct GOPS {}
impl Tree4Profile for GOPS {
    type First = Graph;
    type Second = Object;
    type Third = Predicate;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = false;
}
pub struct SPOGAlways {}
impl Tree4Profile for SPOGAlways {
    type First = Subject;
    type Second = Predicate;
    type Third = Object;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct SPGOAlways {}
impl Tree4Profile for SPGOAlways {
    type First = Subject;
    type Second = Predicate;
    type Third = Graph;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct SOPGAlways {}
impl Tree4Profile for SOPGAlways {
    type First = Subject;
    type Second = Object;
    type Third = Predicate;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct SOGPAlways {}
impl Tree4Profile for SOGPAlways {
    type First = Subject;
    type Second = Object;
    type Third = Graph;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct SGPOAlways {}
impl Tree4Profile for SGPOAlways {
    type First = Subject;
    type Second = Graph;
    type Third = Predicate;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct SGOPAlways {}
impl Tree4Profile for SGOPAlways {
    type First = Subject;
    type Second = Graph;
    type Third = Object;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct PSOGAlways {}
impl Tree4Profile for PSOGAlways {
    type First = Predicate;
    type Second = Subject;
    type Third = Object;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct PSGOAlways {}
impl Tree4Profile for PSGOAlways {
    type First = Predicate;
    type Second = Subject;
    type Third = Graph;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct POSGAlways {}
impl Tree4Profile for POSGAlways {
    type First = Predicate;
    type Second = Object;
    type Third = Subject;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct POGSAlways {}
impl Tree4Profile for POGSAlways {
    type First = Predicate;
    type Second = Object;
    type Third = Graph;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct PGSOAlways {}
impl Tree4Profile for PGSOAlways {
    type First = Predicate;
    type Second = Graph;
    type Third = Subject;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct PGOSAlways {}
impl Tree4Profile for PGOSAlways {
    type First = Predicate;
    type Second = Graph;
    type Third = Object;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct OSPGAlways {}
impl Tree4Profile for OSPGAlways {
    type First = Object;
    type Second = Subject;
    type Third = Predicate;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct OSGPAlways {}
impl Tree4Profile for OSGPAlways {
    type First = Object;
    type Second = Subject;
    type Third = Graph;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct OPSGAlways {}
impl Tree4Profile for OPSGAlways {
    type First = Object;
    type Second = Predicate;
    type Third = Subject;
    type Fourth = Graph;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct OPGSAlways {}
impl Tree4Profile for OPGSAlways {
    type First = Object;
    type Second = Predicate;
    type Third = Graph;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct OGSPAlways {}
impl Tree4Profile for OGSPAlways {
    type First = Object;
    type Second = Graph;
    type Third = Subject;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct OGPSAlways {}
impl Tree4Profile for OGPSAlways {
    type First = Object;
    type Second = Graph;
    type Third = Predicate;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct GSPOAlways {}
impl Tree4Profile for GSPOAlways {
    type First = Graph;
    type Second = Subject;
    type Third = Predicate;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct GSOPAlways {}
impl Tree4Profile for GSOPAlways {
    type First = Graph;
    type Second = Subject;
    type Third = Object;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct GPSOAlways {}
impl Tree4Profile for GPSOAlways {
    type First = Graph;
    type Second = Predicate;
    type Third = Subject;
    type Fourth = Object;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct GPOSAlways {}
impl Tree4Profile for GPOSAlways {
    type First = Graph;
    type Second = Predicate;
    type Third = Object;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct GOSPAlways {}
impl Tree4Profile for GOSPAlways {
    type First = Graph;
    type Second = Object;
    type Third = Subject;
    type Fourth = Predicate;
    const ALWAYS_INSTANCIATED: bool = true;
}
pub struct GOPSAlways {}
impl Tree4Profile for GOPSAlways {
    type First = Graph;
    type Second = Object;
    type Third = Predicate;
    type Fourth = Subject;
    const ALWAYS_INSTANCIATED: bool = true;
}
