extern crate wasm_bindgen;

// TODO : find how to use folder instead of using _

// Data Model
pub mod datamodel_term;
pub mod datamodel_quad;
pub mod datamodel_factory;

// Dataset Core
pub mod matchableterm;
pub mod exportiterator;
pub mod dataset_core;

// Debug / Log
pub mod util;
use crate::util::*;

// Required by factory
use crate::dataset_core::*;


// The lab

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Dog {
    name: String
}

#[wasm_bindgen]
impl Dog {
    #[wasm_bindgen(constructor)]
    pub fn new(s: String) -> Dog {
        Dog { name: s }
    }
    
    pub fn woof(&self) {
        let full_woof = format!("🐶 {}: Woof !", self.name);
        log(full_woof.as_str());
    }
}
