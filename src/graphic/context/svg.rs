use super::{Context, Point};
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

// TODO: Remove systematic append?
// Provide the option to "group"? <g>
// How to generalize to all context?
impl Context for SvgRenderingContext {
    fn draw_pixel(&self, coord: Point, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let rect = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
            .dyn_into::<SvgRectElement>()?;
        rect.set_attribute("x", &coord.x.to_string())?;
        rect.set_attribute("y", &coord.y.to_string())?;
        rect.set_attribute("width", "1")?;
        rect.set_attribute("height", "1")?;
        rect.set_attribute("fill", color)?;
        self.svg.append_child(&rect)?;
        Ok(())
    }

    fn draw_line(&self, start: Point, end: Point, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let line = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?
            .dyn_into::<SvgLineElement>()?;
        line.set_attribute("x1", &start.x.to_string())?;
        line.set_attribute("y1", &start.y.to_string())?;
        line.set_attribute("x2", &end.x.to_string())?;
        line.set_attribute("y2", &end.y.to_string())?;
        line.set_attribute("stroke", color)?;
        self.svg.append_child(&line)?;
        Ok(())
    }

    fn draw_rect(&self, start: Point, end: Point, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let rect = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
            .dyn_into::<SvgRectElement>()?;

        rect.set_attribute("x", &start.x.to_string())?;
        rect.set_attribute("y", &start.y.to_string())?;
        rect.set_attribute("width", &end.x.to_string())?;
        rect.set_attribute("height", &end.y.to_string())?;
        rect.set_attribute("stroke", "black")?;
        rect.set_attribute("fill", color)?;
        self.svg.append_child(&rect)?;
        Ok(())
    }

    fn draw_circle(&self, center: Point, radius: f32, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let circle = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?
            .dyn_into::<SvgCircleElement>()?;
        circle.set_attribute("cx", &center.x.to_string())?;
        circle.set_attribute("cy", &center.y.to_string())?;
        circle.set_attribute("r", &radius.to_string())?;
        circle.set_attribute("fill", color)?;
        self.svg.append_child(&circle)?;
        Ok(())
    }

    fn draw_text(&self, position: Point, text: &str, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let text_element = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "text")?
            .dyn_into::<SvgTextElement>()?;
        text_element.set_attribute("x", &position.x.to_string())?;
        text_element.set_attribute("y", &position.y.to_string())?;
        text_element.set_attribute("fill", color)?;
        text_element.set_attribute("font-size", "12")?;
        text_element.set_attribute("font-family", "monospace")?;
        text_element.set_attribute("text-anchor", "end")?;
        text_element.set_inner_html(text);
        self.svg.append_child(&text_element)?;
        Ok(())
    }
}
