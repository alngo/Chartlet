use wasm_bindgen::prelude::*;

use super::Point;

mod svg;

pub trait Context {
    fn draw_pixel(&self, point: Point) -> Result<(), String>;
    fn draw_line(&self, start: Point, end: Point) -> Result<(), String>;
    fn draw_rect(&self, start: Point, end: Point) -> Result<(), String>;
    fn draw_circle(&self, center: Point, radius: f64) -> Result<(), String>;
    fn draw_text(&self, position: Point, text: &str) -> Result<(), String>;
}
