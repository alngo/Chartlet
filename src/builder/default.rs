use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::chart::frame::Frame;
use crate::graphic::composite::Point;
use crate::graphic::context::Context;
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

    pub fn build_timeline(&self, frame: Frame, timeline: Vec<u32>) {
        timeline.iter().for_each(|t| {
            let point = Point::new(*t as f32, 1.0);
            let start = frame.to_viewport(point, 100, 100);
            let end = Point::new(0.0, 100.0);
            self.context.draw_line(start, end, "white").unwrap();
        });
    }

    pub fn get_context(&self) -> SvgElement {
        self.context.svg.clone()
    }
}
