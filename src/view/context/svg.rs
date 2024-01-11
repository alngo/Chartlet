use wasm_bindgen::JsCast;
use web_sys::{window, Document, SvgElement};

use crate::view::Point;

use super::Context;

struct Svg {
    document: Document,
    svg: SvgElement,
}

impl Svg {
    pub fn new(width: u32, height: u32) -> Svg {
        let document = window().unwrap().document().unwrap();
        let svg = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
            .unwrap()
            .dyn_into::<SvgElement>()
            .unwrap();
        svg.set_attribute("width", &width.to_string()).unwrap();
        svg.set_attribute("height", &height.to_string()).unwrap();
        svg.set_attribute("viewBox", &format!("0 0 {} {}", width, height))
            .unwrap();
        Svg { document, svg }
    }
}

// pub struct Point(pub f64, pub f64);

impl Context for Svg {
    fn draw_pixel(&self, point: Point) -> Result<(), String> {
        todo!()
    }

    fn draw_line(&self, start: Point, end: Point) -> Result<(), String> {
        todo!()
    }

    fn draw_rect(&self, start: Point, end: Point) -> Result<(), String> {
        todo!()
    }

    fn draw_circle(&self, center: Point, radius: f64) -> Result<(), String> {
        todo!()
    }

    fn draw_text(&self, position: Point, text: &str) -> Result<(), String> {
        todo!()
    }
}
