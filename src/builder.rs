use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::chart::frame::Frame;

pub mod default;

#[wasm_bindgen]
extern "C" {
    pub type Builder;

    #[wasm_bindgen(constructor)]
    fn new() -> Builder;

    #[wasm_bindgen(structural, method)]
    pub fn build_timeline(this: &Builder, frame: Frame, timeline: Vec<u32>);
}
