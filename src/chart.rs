use wasm_bindgen::prelude::*;
use web_sys::*;
use crate::history::{History, Timeframe};

type Point = (f32, f32);

#[wasm_bindgen]
#[derive(Clone, Debug)]
struct Frame {
    width: u32,
    height: f32,
    offset: Point,
}

impl Frame {
    pub fn new(width: u32, height: f32, offset: Point) -> Frame {
        Frame {
            width: width,
            height: height,
            offset: offset,
        }
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
                offset: (0.0, 0.0),
            },
        }
    }

    pub fn add_data(&mut self, open: f32, high: f32, low: f32, close: f32, volume: f32) {
        self.history.insert(open, high, low, close, volume);
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
        self.frame = Frame::new(width, height, (from as f32, min));
    }
}

#[cfg(test)]
mod chart_tests {
    use super::*;

    #[test]
    fn test_new() {
        let chart = Chart::new(Timeframe::M5, 0);
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [], timeframe: M5, start_date: 0 }, frame: Frame { width: 0, height: 0.0, offset: (0.0, 0.0) } }");
    }

    #[test]
    fn test_add_data() {
        let mut chart = Chart::new(Timeframe::M5, 0);
        chart.add_data(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [(1.0, 2.0, 3.0, 4.0, 5.0)], timeframe: M5, start_date: 0 }, frame: Frame { width: 0, height: 0.0, offset: (0.0, 0.0) } }");
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
        assert_eq!(format!("{:?}", chart), "Chart { history: History { data: [(1.0, 2.0, 3.0, 4.0, 5.0), (2.0, 3.0, 4.0, 5.0, 6.0), (3.0, 4.0, 5.0, 6.0, 7.0), (4.0, 5.0, 6.0, 7.0, 8.0), (5.0, 6.0, 7.0, 8.0, 9.0), (6.0, 7.0, 8.0, 9.0, 10.0)], timeframe: M5, start_date: 0 }, frame: Frame { width: 6, height: 4.0, offset: (0.0, 3.0) } }");
    }
}

