use super::shape::Shape;

struct Rectangle {
    shape: Shape,
}

impl Rectangle {
    fn new(stroke: &str, stroke_width: &str, fill: &str) -> Self {
        Self {
            shape: Shape::new(stroke, stroke_width, fill),
        }
    }
}
