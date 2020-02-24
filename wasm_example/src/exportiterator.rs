//! An iterator on the elements contained by an exported container.
//!
//! The iterator we provide is an iterator on the elements that are contained
//! when we create the iterator : new and deleted elements in the origian
//! container do not change the state of the iterator.
//!
//! This feature is implemented as earlier versions of NodeJs do not support
//! `js_sys::Array::values()`


extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


/// An exportable iterator on that keeps an array of every elements that needs
/// to be iterated on
#[wasm_bindgen]
pub struct RustExportIterator {
    array: js_sys::Array
}

impl RustExportIterator {
    /// Build an iterator from a `js_sys::Array`
    pub fn new(array: js_sys::Array) -> RustExportIterator {
        // We reverse in place so we can think our iterator as a list of quads we have not iterated on yet
        array.reverse();
        RustExportIterator { array }
    }
}

#[wasm_bindgen]
impl RustExportIterator {
    /// Returns an `RustExportIteratorNext` that contains to the next element.
    /// This corresponds to the next function in Javascript iterators.
    pub fn next(&mut self) -> RustExportIteratorNext {
        if self.array.length() != 0 {
            RustExportIteratorNext{ current_element: Some(self.array.pop()) }
        } else {
            RustExportIteratorNext{ current_element: None }
        }
    }
}

/// An object that contains an element returned by `RustExportIterator::next`
///
/// It follows the Javascript specification, having a `done` attribute that tells if the iterator is empty and a
/// `value` attribute that contains the eventual value. They are modelized with an optional `JsValue`.
#[wasm_bindgen]
pub struct RustExportIteratorNext {
    #[wasm_bindgen(skip)]
    /// The JsValue contained by this object
    pub current_element: Option<JsValue>
}

#[wasm_bindgen]
impl RustExportIteratorNext {
    /// Return true if the iterator is empty
    #[wasm_bindgen(getter)]
    pub fn done(&self) -> bool {
        self.current_element.is_none()
    }

    /// Return the possessed `JsValue`
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        match self.current_element.as_ref() {
            None => JsValue::undefined(),
            Some(real_value) => real_value.clone()
        }
    }
}
