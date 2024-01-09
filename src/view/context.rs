use wasm_bindgen::prelude::*;

use super::Point;

pub trait Context {
    fn draw_pixel(&self, point: Point, color: &str) -> Result<(), JsValue>;
    fn draw_line(&self, start: Point, end: Point, color: &str) -> Result<(), JsValue>;
    fn draw_rect(
        &self,
        start: Point,
        width: &str,
        height: &str,
        color: &str,
    ) -> Result<(), JsValue>;
    fn draw_circle(&self, center: Point, radius: f64, color: &str) -> Result<(), JsValue>;
    fn draw_text(&self, position: Point, text: &str, color: &str) -> Result<(), JsValue>;
}
