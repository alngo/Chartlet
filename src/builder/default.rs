use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::chart::frame::Framer;
use crate::graphic::composite::timeline::Timeline;
use crate::graphic::context::svg::SvgRenderingContext;

#[wasm_bindgen]
pub struct DefaultBuilder {
    context: SvgRenderingContext,
}

#[wasm_bindgen]
impl DefaultBuilder {
    pub fn new() -> DefaultBuilder {
        let context = SvgRenderingContext::new(100, 100).unwrap();
        DefaultBuilder { context }
    }

    pub fn build_timeline(&self, frame: &Framer, timeline: Vec<u32>) {
        console::log_1(&JsValue::from_str("Hello from default builder"));
    }
}
