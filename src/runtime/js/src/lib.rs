//! JavaScript runtime bindings for the Gigli programming language
//!
//! This crate provides WebAssembly bindings for running Gigli programs
//! in JavaScript environments such as web browsers and Node.js.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, Event, HtmlElement};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, gigli-runtime-js!");
}

#[wasm_bindgen]
pub fn set_inner_html(id: &str, html: &str) {
    let document = window().unwrap().document().unwrap();
    if let Some(elem) = document.get_element_by_id(id) {
        elem.set_inner_html(html);
    }
}

#[wasm_bindgen]
pub fn add_event_listener(id: &str, event: &str, callback: &js_sys::Function) {
    let document = window().unwrap().document().unwrap();
    if let Some(elem) = document.get_element_by_id(id) {
        let cb = Closure::wrap(Box::new(move |e: Event| {
            callback.call1(&JsValue::NULL, &e.into()).unwrap();
        }) as Box<dyn FnMut(_)>);
        elem.add_event_listener_with_callback(event, cb.as_ref().unchecked_ref()).unwrap();
        cb.forget();
    }
}

#[wasm_bindgen]
pub fn inject_style(css: &str) {
    let document = window().unwrap().document().unwrap();
    let style = document.create_element("style").unwrap();
    style.set_inner_html(css);
    document.head().unwrap().append_child(&style).unwrap();
}

#[wasm_bindgen]
pub fn update_element(id: &str, value: &str) {
    set_inner_html(id, value);
}

#[wasm_bindgen]
pub struct GigliRuntime {
    // TODO: Add runtime state
}

#[wasm_bindgen]
impl GigliRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GigliRuntime {
        GigliRuntime {
            // TODO: Initialize runtime
        }
    }

    #[wasm_bindgen]
    pub fn execute(&self, _bytecode: &[u8]) -> Result<JsValue, JsValue> {
        // TODO: Execute Gigli bytecode
        Ok(JsValue::NULL)
    }
}
