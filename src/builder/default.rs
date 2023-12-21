use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::chart::frame::Frame;
use crate::graphic::composite::Point;
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
        console::log_1(&JsValue::from_str("Hello from default builder"));
        let point = Point::new(1.0, 1.0990);
        let vpoint = frame.to_viewport(point, 100, 100);
        console::log_3(
            &JsValue::from_str("vpoint: "),
            &JsValue::from_f64(vpoint.x as f64),
            &JsValue::from_f64(vpoint.y as f64),
        );
    }
}
