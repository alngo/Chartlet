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
            let point = Point::new(i as f32, 0.0);
            let start = frame.to_viewport(point, self.width, self.height);
            let end = Point::new(start.x, self.height as f32);
            let text = NaiveDateTime::from_timestamp(*t as i64, 0)
                .format("%H:%M:%S")
                .to_string();
            self.context.draw_line(start, end, "white").unwrap();
            self.context.draw_text(end, text.as_str(), "white").unwrap();
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
            console::log_1(&JsValue::from_str(text.as_str()));
            self.context.draw_line(start, end, "white").unwrap();
            self.context
                .draw_text(end, text.as_str(), "white")
                .unwrap();
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

            let point = Point::new(i as f32, d.open);
            let start = frame.to_viewport(point, self.width, self.height);
            let point = Point::new(1.0, d.close);
            console::log_1(&JsValue::from_str(format!("{:?}", point).as_str()));
            let end = frame.to_viewport(point, self.width, self.height);
            let color = if d.open > d.close { "red" } else { "green" };
            self.context.draw_rect(start, end, color).unwrap();
        });

    }

    pub fn get_context(&self) -> SvgElement {
        self.context.svg.clone()
    }
}