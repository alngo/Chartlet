use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::chart::frame::Frame;
use crate::chart::history::Data;

pub mod default;

#[wasm_bindgen]
extern "C" {
    pub type Builder;

    #[wasm_bindgen(constructor)]
    fn new() -> Builder;

    #[wasm_bindgen(structural, method)]
    pub fn build_timeline(this: &Builder, frame: Frame, timeline: Vec<u32>);

    #[wasm_bindgen(structural, method)]
    pub fn build_quotation(this: &Builder, frame: Frame);

    #[wasm_bindgen(structural, method)]
    pub fn build_candles(this: &Builder, frame: Frame, data: Vec<Data>);

    #[wasm_bindgen(structural, method)]
    pub fn get_context(this: &Builder) -> HtmlElement;
}
