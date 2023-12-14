pub use context::Context;

#[derive(PartialEq, Debug)]
pub enum ChartKind {
    Line,
    Bar,
    Candle,
}

#[derive(PartialEq, Debug)]
struct Ohlc {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}

// Offset is a tuple of (x, y) coordinates
type Offset = (u32, f32);

#[derive(PartialEq, Debug)]
struct Window {
    pub(crate) width: u32,
    pub(crate) height: f32,
    pub(crate) offset: Offset,
}

impl Ohlc {
    pub fn new(open: f32, high: f32, low: f32, close: f32) -> Self {
        Ohlc {
            open,
            high,
            low,
            close,
        }
    }
}

pub struct Chart {
    kind: ChartKind,
    window: Window,
    pub data: Vec<Ohlc>,
}

impl Chart {
    pub fn new() -> Self {
        Chart {
            kind: ChartKind::Candle,
            data: Vec::new(),
            window: Window {
                width: 100,
                height: 2.0,
                offset: (0, 0.0),
            },
        }
    }

    pub fn set_kind(&mut self, kind: ChartKind) {
        self.kind = kind;
    }

    pub fn set_window(&mut self, width: u32, height: f32, offset: Offset) {
        self.window = Window {
            width,
            height,
            offset,
        };
    }

    pub fn add_ohlc(&mut self, open: f32, high: f32, low: f32, close: f32) {
        self.data.push(Ohlc::new(open, high, low, close));
    }

    pub fn update_ohlc(&mut self, index: usize, open: f32, high: f32, low: f32, close: f32) {
        self.data[index] = Ohlc::new(open, high, low, close);
    }

    pub fn get_ohlc_at(&self, index: usize) -> Option<(f32, f32, f32, f32)> {
        self.data
            .get(index)
            .map(|ohlc| (ohlc.open, ohlc.high, ohlc.low, ohlc.close))
    }

    fn window_to_viewport(&self, point: (f32, f32), viewport: (u32, u32)) -> (u32, u32) {
        // Check calculation
        let x = (point.0 * self.window.width as f32 / viewport.0 as f32) as u32;
        let y = (point.1 * self.window.height / viewport.1 as f32) as u32;
        (x, y)
    }

    pub fn draw_grid(&self, width: u32, height: u32, context: &impl Context) {
        let step: u32 = 10;

        for x in (0..width).step_by(step as usize) {
            context.draw_line(
                (x as f32, 0.0),
                (x as f32, height as f32),
                "grey",
            );
        }
        for y in (0..height).step_by(step as usize) {
            context.draw_line(
                (0.0, y as f32),
                (width as f32, y as f32),
                "grey",
            );
        }
    }
}


#[cfg(test)]
mod chart_tests {
    use super::*;

    #[test]
    fn test_new() {
        let chart = Chart::new();
        assert_eq!(chart.kind, ChartKind::Candle);
        assert_eq!(chart.data.len(), 0);
        assert_eq!(chart.window.width, 100);
        assert_eq!(chart.window.height, 2.0);
        assert_eq!(chart.window.offset, (0, 0.0));
    }

    #[test]
    fn test_set_kind() {
        let mut chart = Chart::new();
        chart.set_kind(ChartKind::Line);
        assert_eq!(chart.kind, ChartKind::Line);
    }

    #[test]
    fn test_set_window() {
        let mut chart = Chart::new();
        chart.set_window(200, 4.0, (10, 10.0));
        assert_eq!(chart.window.width, 200);
        assert_eq!(chart.window.height, 4.0);
        assert_eq!(chart.window.offset, (10, 10.0));
    }

    #[test]
    fn test_add_ohlc() {
        let mut chart = Chart::new();
        chart.add_ohlc(10.0, 20.0, 5.0, 15.0);
        assert_eq!(chart.data.len(), 1);
    }

    #[test]
    fn test_get_ohlc_at() {
        let mut chart = Chart::new();
        chart.add_ohlc(10.0, 20.0, 5.0, 15.0);
        assert_eq!(chart.get_ohlc_at(0), Some((10.0, 20.0, 5.0, 15.0)));
    }

    #[test]
    fn test_update_ohlc() {
        let mut chart = Chart::new();
        chart.add_ohlc(10.0, 20.0, 5.0, 15.0);
        chart.add_ohlc(15.0, 25.0, 10.0, 20.0);
        chart.update_ohlc(0, 20.0, 30.0, 15.0, 25.0);
        assert_eq!(chart.get_ohlc_at(0), Some((20.0, 30.0, 15.0, 25.0)));
    }
}
