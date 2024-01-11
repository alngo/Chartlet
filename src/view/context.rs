use super::Point;

mod svg;

pub trait Context {
    fn draw_pixel(&self, point: Point);
    fn draw_line(&self, start: Point, end: Point);
    fn draw_rect(&self, start: Point, end: Point);
    fn draw_circle(&self, center: Point, radius: f64);
    fn draw_text(&self, position: Point, text: &str);
}
