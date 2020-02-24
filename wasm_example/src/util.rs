//! This module contains functions to debug using logs in the console
#![allow(dead_code)]
extern crate wasm_bindgen;

use std::any;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_i32(a: i32);
}

pub fn print_type<T>(_: T) {
    log(any::type_name::<T>())
}

/// If I could only keep only function from the whole project, I would keep
/// this one.
pub fn chicken() {
    log("🐔");
}
