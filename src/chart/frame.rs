use wasm_bindgen::prelude::*;

use super::point::Point;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Frame {
    pub(crate) width: u32,
    pub(crate) height: f32,
    pub(crate) offset: Point,
}

#[wasm_bindgen]
impl Frame {
    pub fn new(width: u32, height: f32, offset: Point) -> Frame {
        Frame {
            width: width,
            height: height,
            offset: offset,
        }
    }

    // How to pass this to the builder?
    // Within Js, and in rust
    pub fn to_viewport(&self, point: Point, width: u32, height: u32) -> Point {
        let x_scale = self.width as f32 / width as f32;
        let y_scale = self.height / height as f32;
        let x = (point.x as f32 - self.offset.x as f32) as f32 / x_scale;
        let y = (point.y - self.offset.y) / y_scale;
        Point::new(x, y)
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;

    #[test]
    fn test_frame_to_viewport_origin() {
        let frame = Frame::new(100, 2.0, Point::new(0.0, 0.0));
        let coordinate = Point::new(0.0, 0.0);
        assert_eq!(
            frame.to_viewport(coordinate, 100, 100),
            Point::new(0.0, 0.0)
        );
    }

    #[test]
    fn test_frame_to_viewport_origin_with_offset() {
        let frame = Frame::new(100, 2.0, Point::new(1.0, 1.0));
        let coordinate = Point::new(0.0, 0.0);
        assert_eq!(
            frame.to_viewport(coordinate, 100, 100),
            Point::new(-1.0, -50.0)
        );
    }

    #[test]
    fn test_frame_to_viewport_coordinates() {
        let frame = Frame::new(100, 2.0, Point::new(0.0, 0.0));
        let coordinate = Point::new(1.0, 1.0);
        assert_eq!(
            frame.to_viewport(coordinate, 100, 100),
            Point::new(1.0, 50.0)
        );
        let coordinate = Point::new(1.0, 1.5);
        assert_eq!(
            frame.to_viewport(coordinate, 100, 100),
            Point::new(1.0, 75.0)
        );
        let coordinate = Point::new(2.0, 2.0);
        assert_eq!(
            frame.to_viewport(coordinate, 100, 100),
            Point::new(2.0, 100.0)
        );
    }

    #[test]
    fn test_frame_to_viewport_coordinates_with_offset() {
        let frame = Frame::new(100, 2.0, Point::new(1.0, 1.0));
        let coordinate = Point::new(0.0, 0.0);
        assert_eq!(
            frame.to_viewport(coordinate, 100, 100),
            Point::new(-1.0, -50.0)
        );
        let coordinate = Point::new(1.0, 1.0);
        assert_eq!(
            frame.to_viewport(coordinate, 100, 100),
            Point::new(0.0, 0.0)
        );
        let coordinate = Point::new(1.0, 1.5);
        assert_eq!(
            frame.to_viewport(coordinate, 100, 100),
            Point::new(0.0, 25.0)
        );
        let coordinate = Point::new(1.0, 2.0);
        assert_eq!(
            frame.to_viewport(coordinate, 100, 100),
            Point::new(0.0, 50.0)
        );
    }
}
