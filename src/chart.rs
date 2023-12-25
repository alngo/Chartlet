use wasm_bindgen::prelude::*;

// use crate::graphic::Renderer;
use crate::builder::Builder;

pub mod frame;
pub mod history;
pub mod point;

use frame::Frame;
use history::{History, Timeframe};
use point::Point;

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
                offset: Point::new(0.0, 0.0),
            },
        }
    }

    pub fn add_data(&mut self, open: f32, high: f32, low: f32, close: f32, volume: f32) {
        self.history.insert(open, high, low, close, volume);
    }

    pub fn set_frame(&mut self, width: u32, height: f32, offset_x: f32, offset_y: f32) {
        let offset = Point::new(offset_x, offset_y);
        self.frame = Frame::new(width, height, offset);
    }

    pub fn auto_frame(&mut self, from: u32, to: u32) {
        let datas = self.history.get_data(from, to);
        let mut min = datas[0].low;
        let mut max = datas[0].high;
        datas.into_iter().for_each(|data| {
            if data.high > max {
                max = data.high;
            }
            if data.low < min {
                min = data.low;
            }
        });
        let height = max - min;
        let width = to - from;
        self.frame = Frame::new(width, height, Point::new(from as f32, min));
    }

    pub fn build_with(&self, builder: &Builder) {
        let timeline = self.history.get_timeline(
            self.frame.offset.x as u32,
            self.frame.offset.x as u32 + self.frame.width,
        );
        builder.build_timeline(self.frame.clone(), timeline);
        builder.build_quotation(self.frame.clone());
        builder.build_candles(
            self.frame.clone(),
            self.history.get_data(
                self.frame.offset.x as u32,
                self.frame.offset.x as u32 + self.frame.width,
            ),
        );
    }
}

#[cfg(test)]
mod chart_tests {
    use super::*;

    #[test]
    fn test_new() {
        let chart = Chart::new(Timeframe::M5, 0);
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [], timeframe: M5, start_date: 0 }, frame: Frame { width: 0, height: 0.0, offset: Point { x: 0.0, y: 0.0 } } }");
    }

    #[test]
    fn test_add_data() {
        let mut chart = Chart::new(Timeframe::M5, 0);
        chart.add_data(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [Data { open: 1.0, high: 2.0, low: 3.0, close: 4.0, volume: 5.0 }], timeframe: M5, start_date: 0 }, frame: Frame { width: 0, height: 0.0, offset: Point { x: 0.0, y: 0.0 } } }");
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
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [Data { open: 1.0, high: 2.0, low: 3.0, close: 4.0, volume: 5.0 }, Data { open: 2.0, high: 3.0, low: 4.0, close: 5.0, volume: 6.0 }, Data { open: 3.0, high: 4.0, low: 5.0, close: 6.0, volume: 7.0 }, Data { open: 4.0, high: 5.0, low: 6.0, close: 7.0, volume: 8.0 }, Data { open: 5.0, high: 6.0, low: 7.0, close: 8.0, volume: 9.0 }, Data { open: 6.0, high: 7.0, low: 8.0, close: 9.0, volume: 10.0 }], timeframe: M5, start_date: 0 }, frame: Frame { width: 6, height: 4.0, offset: Point { x: 0.0, y: 3.0 } } }");
    }

    #[test]
    fn test_set_frame() {
        let mut chart = Chart::new(Timeframe::M5, 0);
        chart.set_frame(6, 4.0, 0.0, 3.0);
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [], timeframe: M5, start_date: 0 }, frame: Frame { width: 6, height: 4.0, offset: Point { x: 0.0, y: 3.0 } } }");
    }
}
