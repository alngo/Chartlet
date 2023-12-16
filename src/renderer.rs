use crate::chart::Frame;
use wasm_bindgen::prelude::*;
use web_sys::*;

enum RenderingContext {}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Renderer {}

impl Renderer {
    pub fn default() -> Renderer {
        Renderer {}
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
