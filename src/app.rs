use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::chart::{Chart, Ohlc};
use crate::configuration::Configuration;

#[wasm_bindgen]
pub struct App {
    configuration: Configuration,
    chart: Chart,
}

#[wasm_bindgen]
impl App {
    pub fn new(width: u32, height: u32) -> Self {
        App {
            configuration: Configuration::new(width, height),
            chart: Chart::new(),
        }
    }

    pub fn set_shift(&mut self, shift: u32) {
        self.configuration.set_shift(shift);
    }

    pub fn set_scale(&mut self, scale: u32) {
        self.configuration.set_scale(scale);
    }

    pub fn set_width(&mut self, width: u32) {
        self.configuration.set_width(width);
    }

    pub fn set_height(&mut self, height: u32) {
        self.configuration.set_height(height);
    }

    pub fn add_ohcl(&mut self, open: f32, high: f32, low: f32, close: f32) {
        let ohlc = Ohlc::new(open, high, low, close);
        self.chart.add(ohlc);
    }
}

#[cfg(test)]
mod app_tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new(100, 100);
        assert_eq!(app.configuration.width, 100);
        assert_eq!(app.configuration.height, 100);
    }

    #[test]
    fn test_app_set_shift() {
        let mut app = App::new(100, 100);
        app.set_shift(10);
        assert_eq!(app.configuration.shift, 10);
    }

    #[test]
    fn test_app_set_scale() {
        let mut app = App::new(100, 100);
        app.set_scale(10);
        assert_eq!(app.configuration.scale, 10);
    }

    #[test]
    fn test_app_set_width() {
        let mut app = App::new(100, 100);
        app.set_width(10);
        assert_eq!(app.configuration.width, 10);
    }

    #[test]
    fn test_app_set_height() {
        let mut app = App::new(100, 100);
        app.set_height(10);
        assert_eq!(app.configuration.height, 10);
    }

    #[test]
    fn test_app_add() {
        let mut app = App::new(100, 100);
        app.add_ohcl(10.0, 20.0, 5.0, 15.0);
        assert_eq!(app.chart.data.len(), 1);
        assert_eq!(app.chart.data[0].open, 10.0);
        assert_eq!(app.chart.data[0].high, 20.0);
        assert_eq!(app.chart.data[0].low, 5.0);
        assert_eq!(app.chart.data[0].close, 15.0);
    }
}
