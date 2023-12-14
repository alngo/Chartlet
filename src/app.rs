use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::chart::Chart;
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

    pub fn set_width(&mut self, width: u32) {
        self.configuration.set_width(width);
    }

    pub fn set_height(&mut self, height: u32) {
        self.configuration.set_height(height);
    }

    pub fn set_shift(&mut self, shift: u32) {
        self.configuration.set_shift(shift);
    }

    pub fn add_ohlc(&mut self, open: f32, high: f32, low: f32, close: f32) {
        self.chart.add_ohlc(open, high, low, close);
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
    fn test_app_set_shift() {
        let mut app = App::new(100, 100);
        app.set_shift(10);
        assert_eq!(app.configuration.shift, 10);
    }
}
