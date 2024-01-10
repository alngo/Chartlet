use super::Point;

struct Viewport(usize, usize);
struct Frame(usize, f64, usize, f64);

pub struct Converter {
    viewport: Viewport,
    frame: Frame,
}

impl Converter {
    pub fn new(
        width: usize,
        height: usize,
        min_x: usize,
        min_y: f64,
        max_x: usize,
        max_y: f64,
    ) -> Converter {
        Converter {
            viewport: Viewport(width, height),
            frame: Frame(min_x, min_y, max_x, max_y),
        }
    }

    pub fn frame_to_viewport(&self, point: Point) -> Point {
        let frame_width = self.frame.2 - self.frame.0;
        let frame_height = self.frame.3 - self.frame.1;
        let x_scale = frame_width as f64 / self.viewport.0 as f64;
        let y_scale = frame_height / self.viewport.1 as f64;
        let x = (point.0 as f64 - self.frame.0 as f64) as f64 / x_scale;
        let y = (point.1 - self.frame.1) / y_scale;
        Point(x, y)
    }

    pub fn viewport_to_frame(&self, point: Point) -> Point {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_to_viewport_coordinates() {
        let converter = Converter::new(100, 100, 0, 0.0, 100, 2.0);
        assert_eq!(
            converter.frame_to_viewport(Point(1.0, 1.0)),
            Point(1.0, 50.0)
        );
        assert_eq!(
            converter.frame_to_viewport(Point(1.0, 1.5)),
            Point(1.0, 75.0)
        );
        assert_eq!(
            converter.frame_to_viewport(Point(2.0, 2.0)),
            Point(2.0, 100.0)
        );
    }
}
