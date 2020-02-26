
#[wasm_bindgen(js_name=Cat)]
pub struct Cat {
}

#[wasm_bindgen(js_class=Cat)]
impl Cat {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Cat { Cat{ } }

    #[wasm_bindgen]
    pub fn speak(&self) -> String {
        String::from("Meow")
    }

    #[wasm_bindgen]
    pub fn meow(&self) -> String {
        String::from("- Ignores you -")
    }
}

#[wasm_bindgen(js_name=Dog)]
pub struct Dog {

}

#[wasm_bindgen(js_class=Dog)]
impl Dog {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Dog {
        Dog { }
    }

    #[wasm_bindgen]
    pub fn speak(&self) -> String {
        String::from("Woof")
    }

    #[wasm_bindgen]
    pub fn play(&self) -> String {
        String::from("Awesome !")
    }
}

#[wasm_bindgen]
pub fn get_a_new_pet(name: String) -> JsValue {
    if name == "Felix" {
        log("Maou");
        JsValue::from(Cat::new())
    } else if name == "Milou" {
        log("wouf");
        JsValue::from(Dog::new())
    } else {
        log("???");
        log(name.as_str());
        JsValue::null()
    }
}
