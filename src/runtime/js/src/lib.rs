//! JavaScript runtime bindings for the Gigli programming language
//!
//! This crate provides WebAssembly bindings for running Gigli programs
//! in JavaScript environments such as web browsers and Node.js.

use wasm_bindgen::prelude::*;

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
    pub fn execute(&self, bytecode: &[u8]) -> Result<JsValue, JsValue> {
        // TODO: Execute Gigli bytecode
        Ok(JsValue::NULL)
    }
}
