use crate::chart::Frame;
use crate::context::Context;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Renderer {
    context: Box<dyn Context>
}

impl Renderer {
    pub fn default() -> Renderer {
        Renderer {
            context: SvgRenderingContext::new().unwrap(),
        }
    }

    pub fn render_grid(&self, frame: &Frame, timeline: Vec<u32>) {
        todo!()
    }

    pub fn render_chart(&self, frame: &Frame, data: &[(f32, f32, f32, f32, f32)]) {
        todo!()
    }

    pub fn render_indicators(&self) {
        todo!()
    }

    pub fn render_objects(&self) {
        todo!()
    }
}
