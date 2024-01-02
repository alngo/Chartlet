use super::shape::Shape;

struct Circle {
    shape: Shape,
}

impl Circle {
    fn new(stroke: &str, stroke_width: &str, fill: &str) -> Self {
        Self {
            shape: Shape::new(stroke, stroke_width, fill),
        }
    }
}
