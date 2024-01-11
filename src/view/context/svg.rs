use crate::view::Point;

use super::Context;

struct Svg;

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
