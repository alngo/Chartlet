use crate::view::{context, Point};

use super::Drawable;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }
}

impl Drawable for Line {
    fn draw(&self, context: &dyn context::Context) {
        let _ = context.draw_line(self.start, self.end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_line() {
        let line = Line::new(Point(0.0, 0.0), Point(1.0, 1.0));
        assert_eq!(line.start, Point(0.0, 0.0));
        assert_eq!(line.end, Point(1.0, 1.0));
    }
}
