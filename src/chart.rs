pub use crate::context::Context;

#[derive(PartialEq, Debug)]
pub enum ChartKind {
    Line,
    Bar,
    Candle,
}

#[derive(PartialEq, Debug)]
struct Ohlc {
    pub(crate) open: f32,
    pub(crate) high: f32,
    pub(crate) low: f32,
    pub(crate) close: f32,
}

// Offset is a tuple of (x, y) coordinates
type Offset = (f32, f32);
pub type Coordinate = (f32, f32);

#[derive(PartialEq, Debug)]
struct Window {
    pub(crate) width: f32,
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
    window: Window,
    data: Vec<Ohlc>,
    kind: ChartKind,
}

pub trait Graphic {
    fn draw(&self);
}

struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Graphic for Rectangle {
    fn draw(&self) {
        todo!()
    }
}

struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Graphic for Line {
    fn draw(&self) {
        todo!()
    }
}

// Tmp layer to test
pub trait Layer {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn draw(&self, graphics: Vec<Box<dyn Graphic>>);
}

impl Chart {
    pub fn new() -> Self {
        Chart {
            kind: ChartKind::Candle,
            data: Vec::new(),
            window: Window {
                width: 100.0,
                height: 2.0,
                offset: (0.0, 0.0),
            },
        }
    }

    // Chart should have composable Layer
    // Chart track reference to layers
    // Layers are responsible to draw on the chart
    // One mandatory layer is the ohlc layer wich display bars, candles or lines
    // Other layers could be added to display indicators, etc.

    pub fn set_kind(&mut self, kind: ChartKind) {
        self.kind = kind;
    }

    pub fn set_window(&mut self, width: f32, height: f32, offset: Offset) {
        // TODO: check if offset is valid
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

    fn window_to_viewport(&self, coordinate: Coordinate, viewport: (u32, u32)) -> (f32, f32) {
        // calculate the scale factor
        let x_scale = self.window.width / viewport.0 as f32;
        let y_scale = self.window.height / viewport.1 as f32;
        // calculate the new x and y coordinates
        let x = (coordinate.0 - self.window.offset.0) / x_scale;
        let y = (coordinate.1 - self.window.offset.1) / y_scale;
        (x, y)
    }

    fn get_candles(&self) -> &[Ohlc] {
        if self.data.len() < self.window.offset.0 as usize {
            return &[];
        }
        let end = self.window.offset.0 as usize + self.window.width as usize;
        if end > self.data.len() {
            return &self.data[self.window.offset.0 as usize..];
        }
        &self.data[self.window.offset.0 as usize..end]
    }

    // /?\ Should I consider draw_on with the following signature:
    // pub fn draw_on(&self, window: &Window, layer: &impl Layer)
    // or
    // pub fn draw_window_on_layer(&self, window: &Window, layer: &impl Layer)
    // to allow the layer to choose its window?
    // removing the window_to_viewport responsability from the chart
    // window_to_viewport can move to utility? Anti-pattern?
    // Chart can cache "Graphics" instead of "Ohlc" to avoid the conversion
    // and avoid previous calculation
    // How to share the World/Chart?
    // How to draw the grid?
    // How to draw the indicator?
    pub fn draw_on(&self, layer: &impl Layer) {
        let candles: &[Ohlc] = self.get_candles();
        let (width, height) = (layer.width(), layer.height());
        match self.kind {
            ChartKind::Candle => {
                let mut graphics: Vec<Box<dyn Graphic>> = Vec::new();
                for (index, candle) in candles.iter().enumerate() {
                    let x = index as f32;
                    let y = candle.open;
                    let width = 1.0;
                    let height = candle.close - candle.open;
                    let rectangle = Rectangle {
                        x,
                        y,
                        width,
                        height,
                    };
                    let line = Line {
                        x1: x + 0.5,
                        y1: candle.low,
                        x2: x + 0.5,
                        y2: candle.high,
                    };

                    graphics.push(Box::new(rectangle));
                    graphics.push(Box::new(line));
                }
                layer.draw(graphics);
            }
            ChartKind::Line => {
                todo!()
            }
            ChartKind::Bar => {
                todo!()
            }
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
        assert_eq!(chart.window.width, 100.0);
        assert_eq!(chart.window.height, 2.0);
        assert_eq!(chart.window.offset, (0.0, 0.0));
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
        chart.set_window(200.0, 4.0, (10.0, 10.0));
        assert_eq!(chart.window.width, 200.0);
        assert_eq!(chart.window.height, 4.0);
        assert_eq!(chart.window.offset, (10.0, 10.0));
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

    #[test]
    fn test_window_to_viewport_origin() {
        let mut chart = Chart::new();
        chart.set_window(100.0, 2.0, (0.0, 0.0));
        let viewport = (100, 100);
        let coordinate = (0.0, 0.0);
        assert_eq!(chart.window_to_viewport(coordinate, viewport), (0.0, 0.0));
    }

    #[test]
    fn test_window_to_viewport_origin_with_offset() {
        let mut chart = Chart::new();
        chart.set_window(100.0, 2.0, (1.0, 1.0));
        let viewport = (100, 100);
        let coordinate = (0.0, 0.0);
        assert_eq!(
            chart.window_to_viewport(coordinate, viewport),
            (-1.0, -50.0)
        );
    }

    #[test]
    fn test_window_to_viewport_coordinates() {
        let mut chart = Chart::new();
        chart.set_window(100.0, 2.0, (0.0, 0.0));
        let viewport = (100, 100);
        let coordinate = (1.0, 1.0);
        assert_eq!(chart.window_to_viewport(coordinate, viewport), (1.0, 50.0));
        let coordinate = (1.0, 1.5);
        assert_eq!(chart.window_to_viewport(coordinate, viewport), (1.0, 75.0));
        let coordinate = (2.0, 2.0);
        assert_eq!(chart.window_to_viewport(coordinate, viewport), (2.0, 100.0));
    }

    #[test]
    fn test_window_to_viewport_coordinates_with_offset() {
        let mut chart = Chart::new();
        chart.set_window(100.0, 2.0, (1.0, 1.0));
        let viewport = (100, 100);
        let coordinate = (0.0, 0.0);
        assert_eq!(
            chart.window_to_viewport(coordinate, viewport),
            (-1.0, -50.0)
        );
        let coordinate = (1.0, 1.0);
        assert_eq!(chart.window_to_viewport(coordinate, viewport), (0.0, 0.0));
        let coordinate = (1.0, 1.5);
        assert_eq!(chart.window_to_viewport(coordinate, viewport), (0.0, 25.0));
        let coordinate = (1.0, 2.0);
        assert_eq!(chart.window_to_viewport(coordinate, viewport), (0.0, 50.0));
    }

    #[test]
    fn test_get_candles() {}
}
