pub struct Shape {
    pub stroke: Box<str>,
    pub stroke_width: Box<str>,
    pub fill: Box<str>,
}

impl Shape {
    pub fn new(stroke: &str, stroke_width: &str, fill: &str) -> Self {
        Self {
            stroke: stroke.into(),
            stroke_width: stroke_width.into(),
            fill: fill.into(),
        }
    }
}
