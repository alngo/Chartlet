use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    pub type Builder;

    #[wasm_bindgen(constructor)]
    fn new() -> Builder;

    #[wasm_bindgen(structural, method)]
    pub fn build_timeline(this: &Builder);
}

#[wasm_bindgen]
pub struct DefaultBuilder {}

#[wasm_bindgen]
impl DefaultBuilder {
    pub fn new() -> DefaultBuilder {
        DefaultBuilder {}
    }

    pub fn build_timeline(&self) {
        console::log_1(&JsValue::from_str("Hello from default builder"));
    }
}
