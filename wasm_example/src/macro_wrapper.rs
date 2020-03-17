extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use crate::util::*;





macro_rules! animal {
    ($class_name:ident, $js_name:expr) => (
        #[wasm_bindgen(js_name = $js_name)]
        pub struct $class_name {
        }
        
        #[wasm_bindgen(js_class = $js_name)]
        impl $class_name {
            /// A very insteresting constructor
            #[wasm_bindgen(constructor)]
            pub fn new() -> $class_name {
                $class_name {}
            }
        
            /// Heyyyy
            pub fn cry(&self) {
                log("Hey");
            }
        }
    );
}


animal!(Animal, "Pet");

//animal!(Lama);
