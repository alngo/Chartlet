use super::{Context, Coordinate};
use wasm_bindgen::prelude::{JsCast, JsValue};
use web_sys::{SvgCircleElement, SvgElement, SvgLineElement, SvgRectElement, SvgTextElement};

pub struct SvgRenderingContext {
    pub svg: SvgElement,
}

impl SvgRenderingContext {
    pub fn new(width: u32, height: u32) -> Result<SvgRenderingContext, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let svg = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?
            .dyn_into::<SvgElement>()?;
        svg.set_attribute("width", &width.to_string())?;
        svg.set_attribute("height", &height.to_string())?;
        svg.set_attribute("viewBox", &format!("0 0 {} {}", width, height))?;
        Ok(SvgRenderingContext { svg })
    }
}

impl Context for SvgRenderingContext {
    fn draw_pixel(&self, coord: Coordinate, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let rect = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
            .dyn_into::<SvgRectElement>()?;
        rect.set_attribute("x", &coord.0.to_string())?;
        rect.set_attribute("y", &coord.1.to_string())?;
        rect.set_attribute("width", "1")?;
        rect.set_attribute("height", "1")?;
        rect.set_attribute("fill", color)?;
        self.svg.append_child(&rect)?;
        Ok(())
    }

    fn draw_line(&self, start: Coordinate, end: Coordinate, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let line = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?
            .dyn_into::<SvgLineElement>()?;
        line.set_attribute("x1", &start.0.to_string())?;
        line.set_attribute("y1", &start.1.to_string())?;
        line.set_attribute("x2", &end.0.to_string())?;
        line.set_attribute("y2", &end.1.to_string())?;
        line.set_attribute("stroke", color)?;
        self.svg.append_child(&line)?;
        Ok(())
    }

    fn draw_rect(&self, start: Coordinate, end: Coordinate, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let rect = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
            .dyn_into::<SvgRectElement>()?;
        rect.set_attribute("x", &start.0.to_string())?;
        rect.set_attribute("y", &start.1.to_string())?;
        rect.set_attribute("width", &end.0.to_string())?;
        rect.set_attribute("height", &end.1.to_string())?;
        rect.set_attribute("stroke", "black")?;
        rect.set_attribute("fill", color)?;
        self.svg.append_child(&rect)?;
        Ok(())
    }

    fn draw_circle(&self, center: Coordinate, radius: f32, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let circle = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?
            .dyn_into::<SvgCircleElement>()?;
        circle.set_attribute("cx", &center.0.to_string())?;
        circle.set_attribute("cy", &center.1.to_string())?;
        circle.set_attribute("r", &radius.to_string())?;
        circle.set_attribute("fill", color)?;
        self.svg.append_child(&circle)?;
        Ok(())
    }

    fn draw_text(&self, position: Coordinate, text: &str, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let text_element = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "text")?
            .dyn_into::<SvgTextElement>()?;
        text_element.set_attribute("x", &position.0.to_string())?;
        text_element.set_attribute("y", &position.1.to_string())?;
        text_element.set_attribute("fill", color)?;
        text_element.set_attribute("font-size", "12")?;
        text_element.set_attribute("font-family", "monospace")?;
        text_element.set_attribute("text-anchor", "middle")?;
        text_element.set_inner_html(text);
        self.svg.append_child(&text_element)?;
        Ok(())
    }
}
