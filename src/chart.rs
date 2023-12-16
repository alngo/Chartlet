use crate::history::{History, Timeframe};
use crate::renderer::Renderer;
use wasm_bindgen::prelude::*;

type Point = (u32, f32);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Frame {
    width: u32,
    height: f32,
    offset: (u32, f32),
}

impl Frame {
    pub fn new(width: u32, height: f32, offset: Point) -> Frame {
        Frame {
            width: width,
            height: height,
            offset: offset,
        }
    }

    pub fn to_viewport(&self, point: Point, viewport: (u32, u32)) -> (f32, f32) {
        let x_scale = self.width as f32 / viewport.0 as f32;
        let y_scale = self.height / viewport.1 as f32;
        let x = (point.0 as f32 - self.offset.0 as f32) as f32 / x_scale;
        let y = (point.1 - self.offset.1) / y_scale;
        (x, y)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Chart {
    history: History,
    frame: Frame,
}

#[wasm_bindgen]
impl Chart {
    pub fn new(timeframe: Timeframe, start_date: u32) -> Chart {
        Chart {
            history: History::new(timeframe, start_date),
            frame: Frame {
                width: 0,
                height: 0.0,
                offset: (0, 0.0),
            },
        }
    }

    pub fn add_data(&mut self, open: f32, high: f32, low: f32, close: f32, volume: f32) {
        self.history.insert(open, high, low, close, volume);
    }

    pub fn set_frame(&mut self, width: u32, height: f32, offset_x: u32, offset_y: f32) {
        let offset = (offset_x, offset_y);
        self.frame = Frame::new(width, height, offset);
    }

    pub fn auto_frame(&mut self, from: u32, to: u32) {
        let data = self.history.get_data(from, to);
        let mut min = data[0].2;
        let mut max = data[0].1;
        for (_, high, low, _, _) in data {
            if high > &max {
                max = *high;
            }
            if low < &min {
                min = *low;
            }
        }
        let height = max - min;
        let width = to - from;
        self.frame = Frame::new(width, height, (from, min));
    }

    pub fn render_with(&self, renderer: Renderer) {
        renderer.render_grid(
            &self.frame,
            self.history
                .get_timeline(self.frame.offset.0, self.frame.offset.0 + self.frame.width),
        );
        renderer.render_chart(
            &self.frame,
            self.history
                .get_data(self.frame.offset.0, self.frame.offset.0 + self.frame.width),
        );
        renderer.render_indicators();
        renderer.render_objects();
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;

    #[test]
    fn test_frame_to_viewport_origin() {
        let frame = Frame::new(100, 2.0, (0, 0.0));
        let viewport = (100, 100);
        let coordinate = (0, 0.0);
        assert_eq!(frame.to_viewport(coordinate, viewport), (0.0, 0.0));
    }

    #[test]
    fn test_frame_to_viewport_origin_with_offset() {
        let frame = Frame::new(100, 2.0, (1, 1.0));
        let viewport = (100, 100);
        let coordinate = (0, 0.0);
        assert_eq!(frame.to_viewport(coordinate, viewport), (-1.0, -50.0));
    }

    #[test]
    fn test_frame_to_viewport_coordinates() {
        let frame = Frame::new(100, 2.0, (0, 0.0));
        let viewport = (100, 100);
        let coordinate = (1, 1.0);
        assert_eq!(frame.to_viewport(coordinate, viewport), (1.0, 50.0));
        let coordinate = (1, 1.5);
        assert_eq!(frame.to_viewport(coordinate, viewport), (1.0, 75.0));
        let coordinate = (2, 2.0);
        assert_eq!(frame.to_viewport(coordinate, viewport), (2.0, 100.0));
    }

    #[test]
    fn test_frame_to_viewport_coordinates_with_offset() {
        let frame = Frame::new(100, 2.0, (1, 1.0));
        let viewport = (100, 100);
        let coordinate = (0, 0.0);
        assert_eq!(frame.to_viewport(coordinate, viewport), (-1.0, -50.0));
        let coordinate = (1, 1.0);
        assert_eq!(frame.to_viewport(coordinate, viewport), (0.0, 0.0));
        let coordinate = (1, 1.5);
        assert_eq!(frame.to_viewport(coordinate, viewport), (0.0, 25.0));
        let coordinate = (1, 2.0);
        assert_eq!(frame.to_viewport(coordinate, viewport), (0.0, 50.0));
    }
}

#[cfg(test)]
mod chart_tests {
    use super::*;

    #[test]
    fn test_new() {
        let chart = Chart::new(Timeframe::M5, 0);
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [], timeframe: M5, start_date: 0 }, frame: Frame { width: 0, height: 0.0, offset: (0, 0.0) } }");
    }

    #[test]
    fn test_add_data() {
        let mut chart = Chart::new(Timeframe::M5, 0);
        chart.add_data(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [(1.0, 2.0, 3.0, 4.0, 5.0)], timeframe: M5, start_date: 0 }, frame: Frame { width: 0, height: 0.0, offset: (0, 0.0) } }");
    }

    #[test]
    fn test_auto_frame() {
        let mut chart = Chart::new(Timeframe::M5, 0);
        chart.add_data(1.0, 2.0, 3.0, 4.0, 5.0);
        chart.add_data(2.0, 3.0, 4.0, 5.0, 6.0);
        chart.add_data(3.0, 4.0, 5.0, 6.0, 7.0);
        chart.add_data(4.0, 5.0, 6.0, 7.0, 8.0);
        chart.add_data(5.0, 6.0, 7.0, 8.0, 9.0);
        chart.add_data(6.0, 7.0, 8.0, 9.0, 10.0);
        chart.auto_frame(0, 6);
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [(1.0, 2.0, 3.0, 4.0, 5.0), (2.0, 3.0, 4.0, 5.0, 6.0), (3.0, 4.0, 5.0, 6.0, 7.0), (4.0, 5.0, 6.0, 7.0, 8.0), (5.0, 6.0, 7.0, 8.0, 9.0), (6.0, 7.0, 8.0, 9.0, 10.0)], timeframe: M5, start_date: 0 }, frame: Frame { width: 6, height: 4.0, offset: (0, 3.0) } }");
    }

    #[test]
    fn test_set_frame() {
        let mut chart = Chart::new(Timeframe::M5, 0);
        chart.set_frame(6, 4.0, 0, 3.0);
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [], timeframe: M5, start_date: 0 }, frame: Frame { width: 6, height: 4.0, offset: (0, 3.0) } }");
    }
}
