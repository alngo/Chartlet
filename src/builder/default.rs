use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::chart::frame::Frame;
use crate::chart::history::Data;
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
            let point = Point::new(i as f32 + 0.5, 0.0);
            let start = frame.to_viewport(point, self.width, self.height);
            let end = Point::new(start.x, self.height as f32);
            let text = NaiveDateTime::from_timestamp_opt(59, *t as u32)
                .unwrap()
                .format("%H:%M:%S")
                .to_string();
            self.context.draw_line(start, end, "grey").unwrap();
            self.context
                .draw_text(start, text.as_str(), "white")
                .unwrap();
        });
    }

    pub fn build_quotation(&self, frame: Frame) {
        let mut begin: f32 = frame.offset.y;
        let end = frame.offset.y + frame.height;

        while begin < end {
            let point = Point::new(0.0, begin);
            let start = frame.to_viewport(point, self.width, self.height);
            let end = Point::new(self.width as f32, start.y);
            let text = format!("{:.5}", begin);
            self.context.draw_line(start, end, "grey").unwrap();
            self.context.draw_text(end, text.as_str(), "white").unwrap();
            begin += 0.001;
        }
    }

    pub fn build_candles(&self, frame: Frame, data: Vec<Data>) {
        data.iter().enumerate().for_each(|(i, d)| {
            let point = Point::new(i as f32 + 0.5, d.low);
            let start = frame.to_viewport(point, self.width, self.height);
            let point = Point::new(i as f32 + 0.5, d.high);
            let end = frame.to_viewport(point, self.width, self.height);
            self.context.draw_line(start, end, "white").unwrap();

            if d.open > d.close {
                let point = Point::new(i as f32, d.close);
                let start = frame.to_viewport(point, self.width, self.height);
                let point = Point::new(1.0, d.open);
                let end = frame.to_viewport(point, self.width, self.height);
                let height = end.y - start.y;
                let height = if height < 1.0 { 1.0 } else { height };
                self.context
                    .draw_rect(start, &end.x.to_string(), &height.to_string(), "red")
                    .unwrap();
            } else {
                let point = Point::new(i as f32, d.open);
                let start = frame.to_viewport(point, self.width, self.height);
                let point = Point::new(1.0, d.close);
                let end = frame.to_viewport(point, self.width, self.height);
                let height = end.y - start.y;
                let height = if height < 1.0 { 1.0 } else { height };
                self.context
                    .draw_rect(start, &end.x.to_string(), &height.to_string(), "green")
                    .unwrap();
            }
        });
    }

    pub fn get_context(&self) -> SvgElement {
        self.context.svg.clone()
    }
}
