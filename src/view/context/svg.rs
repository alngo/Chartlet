use crate::view::Point;

use super::Context;

struct Svg;

// pub struct Point(pub f64, pub f64);

impl Context for Svg {
    fn draw_pixel(&self, point: Point) {
        todo!()
    }

    fn draw_line(&self, start: Point, end: Point) {
        todo!()
    }

    fn draw_rect(&self, start: Point, end: Point) {
        todo!()
    }

    fn draw_circle(&self, center: Point, radius: f64) {
        todo!()
    }

    fn draw_text(&self, position: Point, text: &str) {
        todo!()
    }
}
