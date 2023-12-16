mod svg;

use wasm_bindgen::prelude::JsValue;

pub type Coordinate = (f32, f32);

pub trait Context {
    fn draw_pixel(&self, coord: Coordinate, color: &str) -> Result<(), JsValue>;
    fn draw_line(&self, start: Coordinate, end: Coordinate, color: &str) -> Result<(), JsValue>;
    fn draw_rect(&self, start: Coordinate, end: Coordinate, color: &str) -> Result<(), JsValue>;
    fn draw_circle(&self, center: Coordinate, radius: f32, color: &str) -> Result<(), JsValue>;
    fn draw_text(&self, position: Coordinate, text: &str, color: &str) -> Result<(), JsValue>;
}
