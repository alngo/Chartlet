use super::context;
use super::Draw;
use super::Point;

pub struct Timeline {
    x: f32,
    height: f32,
    label: String,
}

impl Timeline {
    fn new(x: f32, height: f32) -> Self {
        Self {
            x,
            height,
            label: String::from(""),
        }
    }

    fn with_label(x: f32, height: f32, label: String) -> Self {
        Self { x, height, label }
    }
}

impl Draw for Timeline {
    fn draw(&self, context: &dyn context::Context) {
        match self.label.len() > 0 {
            true => {
                let _ = context.draw_line(
                    Point::new(self.x, 10.0),
                    Point::new(self.x, self.height),
                    "white",
                );
                let _ = context.draw_text(Point::new(self.x, 0.0), self.label.as_str(), "white");
            }
            _ => {
                let _ = context.draw_line(
                    Point::new(self.x, 0.0),
                    Point::new(self.x, self.height),
                    "white",
                );
            }
        }
    }
}
