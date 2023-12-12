use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod canvas_context;
pub mod svg_context;

type Coordinate = (u32, u32);

pub trait Context: Sized {
    fn draw_pixel(&self, coord: Coordinate, color: &str) -> Result<(), JsValue>;
    fn draw_line(&self, start: Coordinate, end: Coordinate, color: &str) -> Result<(), JsValue>;
    fn draw_rect(&self, start: Coordinate, end: Coordinate, color: &str) -> Result<(), JsValue>;
    fn draw_circle(&self, center: Coordinate, radius: u32, color: &str) -> Result<(), JsValue>;
}
