
extern crate wasm_bindgen;

use std::any;
use std::vec::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use sophia::dataset::Dataset;
use sophia::dataset::inmem::FastDataset;
use sophia::graph::inmem::FastGraph;
use sophia::parser::{nt, nq};
use sophia::triple::stream::*;
use sophia::term::*;
use sophia::quad::{*, stream::*};



#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
      // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_i32(a: i32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}


fn print_type<T>(_: T) {
    log(any::type_name::<T>())
}

//Example of a Struct
#[wasm_bindgen]
pub struct Foo {
    internal: i32,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> Foo {
        Foo { internal: val}
    }

    #[wasm_bindgen(getter)]
    pub fn someattribute(&self) -> i32 {
        self.internal
    }

    #[wasm_bindgen(setter)]
    pub fn set_someattribute(&mut self, val: i32){
        self.internal = val * 100;
    }
    
    pub fn display(&mut self){
        log("Foo interal ->");
        log_i32(self.internal);
    }
}

//JavaScript Functions in Rust
#[wasm_bindgen]
pub fn run_alert(item: &str){
    alert(&format!("This is WASM and {}", item));
}

//First Attempt at Loading Sophia FastGraph
#[wasm_bindgen]
pub fn load_graph(graph: &str){
    let NT_DOC: &str = graph;
    let mut g = FastGraph::new();
    let inserted = nt::parse_str(NT_DOC).in_graph(&mut g);
    let num_inserted: u32 = inserted.unwrap() as u32;
    log("N-Triples Inserted ->");
    log_u32(num_inserted);
}

///////////////////////////////
/// A sample JSTerm to be used in the RDF/JS interface
/// David Pojunas
/// Pierre-Antoine Champin
///////////////////////////////

//Term String
#[wasm_bindgen]
#[derive(Clone)]
pub struct JSTerm (Term<Rc<str>>);


#[wasm_bindgen]
impl JSTerm{
    #[wasm_bindgen(constructor)]
    pub fn new(term: String) -> JSTerm {
        let term: &str = term.as_str();
        let term = Rc::from(term);
        let term = Term::new_iri(term).unwrap();
        return JSTerm(term);
    }

    pub fn n3(&self) -> String {
        self.0.n3()
    }
}

impl From <&'_ RcTerm> for JSTerm{
    fn from(other: &RcTerm) -> JSTerm{
        JSTerm(other.clone())
    } 
}

#[wasm_bindgen]
pub struct JSQuad ([JSTerm; 3], Option<JSTerm>);

#[wasm_bindgen]
impl JSQuad{

    #[wasm_bindgen(getter)]
    pub fn s(&self) -> JSTerm{
        self.0[0].clone()
    }

}

#[wasm_bindgen]
pub struct JSDataset (FastDataset);

use js_sys::Array;

#[wasm_bindgen]
impl JSDataset{
    #[wasm_bindgen(constructor)]
    pub fn new() -> JSDataset {
        JSDataset(FastDataset::new())
    }

    pub fn load(&mut self, nquads: &str) -> usize {
        nq::parse_str(nquads).in_dataset(&mut self.0).unwrap()
    }

    pub fn first_subject(&self) -> JSTerm {
        self.0
            .subjects()
            .unwrap()
            .into_iter()
            .map(|term| JSTerm(term.clone()))
            .next()
            .unwrap()
    }

    pub fn quads(&self) -> Array {
       self.0.quads().into_iter().map(|quad| {
            let quad = quad.unwrap();
            JSQuad([quad.s().into(), quad.p().into(), quad.o().into()], quad.g().map(JSTerm::from))
       }).map(JsValue::from).collect()
    }
}

// ============================================================================
// ==== ADAPTERS

/*
#[wasm_bindgen(js_name = Term)]
pub struct WasmToJsTerm {

}

#[wasm_bindgen(js_class = Term)]
impl WasmToJsTerm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmToJsTerm {
        WasmToJsTerm { }
    }

    #[wasm_bindgen(getter = termType)]
    pub fn term_type(&self) -> String {
        String::from("????")
    }
}
*/


/*
pub trait RustToJsTerm {
    // #[wasm_bindgen(getter = termType)]
    fn term_type(&self) -> String;
}


#[wasm_bindgen(js_name="NamedNode")]
pub struct RustToJsNamedNode {
    iri: IriData<Rc<str>>
}

#[wasm_bindgen(js_class="NamedNode")]
impl RustToJsNamedNode {
    #[wasm_bindgen(getter = termType)]
    pub fn term_type(&self) -> String {
        "NamedNode".into()
    }

    #[wasm_bindgen(getter = value)]
    pub fn value(&self) -> String {
        self.iri.to_string()
    }

    #[wasm_bindgen(setter = value)]
    pub fn set_value(&mut self, s: String) {
        self.iri
    }
}

*/

// ============================================================================
// ====


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Term)]
    pub type JssTerm;

    #[wasm_bindgen(method, getter = termType)]
    pub fn term_type(this: &JssTerm) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &JssTerm) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &JssTerm, value: String);

    #[wasm_bindgen(js_name=equals)]
    pub fn equals(this: &JssTerm, other_term: &JssTerm);

    #[wasm_bindgen(method, getter)]
    pub fn language(this: &JssTerm) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_language(this: &JssTerm, value: String);

    #[wasm_bindgen(method, getter)]
    pub fn datatype(this: &JssTerm) -> JssTerm;
    // Returning a copy of a Jssterm is acceptable by RDFJS standard

    #[wasm_bindgen(method, setter)]
    pub fn set_datatype(this: &JssTerm, named_node: &JssTerm);
}


#[wasm_bindgen(js_name="Term")]
pub struct BJTerm {
    term: RcTerm
}

// These functions are not exported into the Javascript interface
impl BJTerm {   
    pub fn new(term: &RcTerm) -> BJTerm {
        BJTerm { term: term.clone() }
    }
}

#[wasm_bindgen(js_class="Term")]
impl BJTerm {
    #[wasm_bindgen(getter = termType)]
    pub fn term_type(&self) -> String {
        match &self.term {
            Iri(_) => "NamedNode".into(),
            BNode(_) => "BlankNode".into(),
            Literal(_1, _2) => "Literal".into(),
            Variable(_) => "Variable".into()
        }
    }

    #[wasm_bindgen(getter = value)]
    pub fn value(&self) -> String {
        self.term.value()
    }

    #[wasm_bindgen(setter = value)]
    pub fn set_value(&mut self, new_value: String) {
        self.term = match &self.term {
            RcTerm::Iri(_) => RcTerm::new_iri(new_value).unwrap(),
            RcTerm::BNode(_) => RcTerm::new_bnode(new_value).unwrap(),
            RcTerm::Variable(_) => RcTerm::new_variable(new_value).unwrap(),
            RcTerm::Literal(_, Lang(lang)) => RcTerm::new_literal_lang(new_value, lang.clone()).unwrap(),
            RcTerm::Literal(_, Datatype(dt))
                => {
                    RcTerm::new_literal_dt(new_value, RcTerm::new_iri(dt.to_string()).unwrap()).unwrap()
                }
        }
    }

    #[wasm_bindgen(getter = language)]
    pub fn language(&self) -> String {
        match &self.term {
            Literal(_, Lang(language)) => language.to_string(),
            _ => String::from("")
        }
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_language(&mut self, language: String) {
        // In this implementation, if we set the language of a literal, it will be automatically
        // converted to the datatype langString regardless of its previous datatype.
        // Setting the language of any other term has no effect.
        if let Literal(old_value, _) = &self.term {
            let language: Rc<str> = language.as_str().into();
            self.term = RcTerm::new_literal_lang(old_value.to_string(), language).unwrap();
        }
    }

    #[wasm_bindgen(getter)]
    pub fn datatype(&self) -> Option<BJTerm> {
        match &self.term {
            Literal(_1, Lang(_2)) =>
                Option::Some(BJTerm {
                    term: RcTerm::new_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString").unwrap()
                }),
            // TODO : check if iri always has a type (especially for string)
            Literal(_1, Datatype(iri)) =>
                Option::Some(BJTerm {
                    term: RcTerm::new_iri(iri.to_string()).unwrap()
                }),
            _ => Option::None
        }
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_datatype(&mut self, named_node: &JssTerm) {
        if let Literal(_, literal_kind) = &self.term {
            if let Lang(_) = literal_kind {
                if named_node.value() == "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString" {
                    // Do not change datatype to langString of literals that are already langString
                    return
                }
            }
            let new_node_value = self.value();
            let literal_type: RcTerm = RcTerm::new_iri(named_node.value().as_str()).unwrap();
            self.term = RcTerm::new_literal_dt(new_node_value, literal_type).unwrap();
        }
    }

    #[wasm_bindgen(js_name = equals)]
    pub fn equals(&self, other: Option<JssTerm>) -> bool {
        match other {
            None => false,
            Some(x) => {
                // We don't use the implementation of term_type / value to have better performances.
                let other_term_type = x.term_type();
                match &self.term {
                    Iri(_) => other_term_type == "NamedNode" && x.value() == self.term.value(),
                    BNode(_) => other_term_type == "BlankNode" && x.value() == self.term.value(),
                    Variable(_) => other_term_type == "Variable" && x.value() == self.term.value(),
                    Literal(_1, literal_kind) => 
                        other_term_type == "Literal" && x.value() == self.term.value()
                            && BJTerm::equals_to_literal(literal_kind, &x)
                }
            }
        }
    }

    fn equals_to_literal(literal_kind: &LiteralKind<Rc<str>>, other : &JssTerm) -> bool {
        // The standard ensures us that other has the language and datatype attributes
        
        // Documentation questionning :
        // Otherwise, if no datatype is explicitly specified, the datatype has the IRI
        // "http://www.w3.org/2001/XMLSchema#string". -> ????
        match literal_kind {
            Lang(language) => language.to_string() == other.language()
                && "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString" == other.datatype().value(),
            Datatype(iri) => other.language() == "" && other.datatype().value() == iri.to_string()
        }
    }
}


#[wasm_bindgen]
impl JSDataset {
    #[wasm_bindgen(js_name="getABJTerm")]
    pub fn get_a_bj_term(&self, id: usize) -> Option<BJTerm> {
        log_i32(id as i32);
        let mut iter = self.0
            .subjects()
            .unwrap()
            .into_iter()
            .map(|term| BJTerm::new(&term));

        let mut i: usize = 0;

        while i <= id {
            let s = iter.next();

            if let Option::None = s {
                return Option::None;
            } else if i == id {
                return s;
            }

            i += 1;
        };

        Option::None
    }

}














