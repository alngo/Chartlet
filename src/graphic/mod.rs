use std::collections::HashMap;

use crate::chart::Frame;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod contexts;

use contexts::{Context, SvgRenderingContext};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Renderer {
    width: u32,
    height: u32,
    layers: HashMap<String, SvgElement>,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new(width: u32, height: u32) -> Renderer {
        Renderer {
            width,
            height,
            layers: HashMap::new(),
        }
    }

    pub fn default() -> Renderer {
        Renderer {
            width: 0,
            height: 0,
            layers: HashMap::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn render_timeline(&mut self, frame: &Frame, timeline: Vec<u32>) -> Result<(), JsValue> {
        let context = SvgRenderingContext::new(self.width, self.height)?;
        for (i, time) in timeline.iter().enumerate() {
            let (x, y) = frame.to_viewport((i as u32, 0.0), (self.width, self.height));
            context.draw_line((x, 0.0), (x, self.height as f32), "black")?;
        }
        self.layers
            .insert("timeline".to_string(), context.svg.clone());
        Ok(())
    }

    pub fn get_timeline(&self) -> SvgElement {
        self.layers.get("timeline").unwrap().clone()
    }

    // pub fn render_chart(&self, frame: &Frame, data: &[(f32, f32, f32, f32, f32)]) {
    //     todo!()
    // }

    // pub fn render_quotation(&self, frame: &Frame, data: &[(f32, f32, f32, f32, f32)]) {
    // consider a vec<f32>
    //     todo!()
    // }

    // pub fn render_indicators(&self) {
    //     todo!()
    // }

    // pub fn render_objects(&self) {
    //     todo!()
    // }
}

#[cfg(test)]
mod renderer_tests {
    use super::*;

    #[test]
    fn test_resize() {
        let mut renderer = Renderer::default();
        renderer.resize(100, 100);
        assert_eq!(renderer.width, 100);
        assert_eq!(renderer.height, 100);
    }
}
