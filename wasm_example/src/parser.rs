//! Adapter for Sophia parsers (which are adapters of rio parsers) to kind of
//! match N3 parsers' interface

extern crate wasm_bindgen;

// use wasm_bindgen::prelude::*;
// use crate::util::*;


/*
enum RequiredParser {
    nquad, trig
}

#[wasm_bindgen]
pub struct Parser {
    parserChoice: RequiredParser
}

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new(&js_value: JsValue) -> Parser {
        if js_value.is_falsy() {
            Parser { parserChoice }
        } else {

        }
    }

}
*/

/*
    /// Loads the content of a rdf graph formatted following the [TriG syntax](https://www.w3.org/TR/trig/)
    pub fn load(&mut self, content: &str) {
        let r = sophia::parser::trig::parse_str(&content).in_dataset(&mut self.dataset);
        match r {
            Ok(_) => {},
            Err(error) => log(error.to_string().as_str())
        }
    }
*/





