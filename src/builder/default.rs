use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct DefaultBuilder {
}

#[wasm_bindgen]
impl DefaultBuilder {
    pub fn new() -> DefaultBuilder {
        DefaultBuilder {}
    }

    pub fn build_timeline(&self, timeline: Vec<u32>) {
        console::log_1(&JsValue::from_str("Hello from default builder"));


    }
}
