use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::chart::frame::Frame;
use crate::graphic::composite::Point;
use crate::graphic::context::svg::SvgRenderingContext;
use crate::graphic::context::Context;

use chrono::prelude::*;

#[wasm_bindgen]
pub struct DefaultBuilder {
    width: u32,
    height: u32,
    context: SvgRenderingContext,
}

#[wasm_bindgen]
impl DefaultBuilder {
    pub fn new(width: u32, height: u32) -> DefaultBuilder {
        let context = SvgRenderingContext::new(width, height).unwrap();
        DefaultBuilder {
            width,
            height,
            context,
        }
    }

    pub fn build_timeline(&self, frame: Frame, timeline: Vec<u32>) {
        timeline.iter().enumerate().for_each(|(i, t)| {
            let point = Point::new(i as f32, 0.0);
            let start = frame.to_viewport(point, self.width, self.height);
            let end = Point::new(start.x, self.height as f32);
            self.context.draw_line(start, end, "white").unwrap();
            let text = NaiveDateTime::from_timestamp(*t as i64, 0)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            self.context.draw_text(end, text.as_str(), "white").unwrap();
        });
    }

    pub fn get_context(&self) -> SvgElement {
        self.context.svg.clone()
    }
}
