use crate::context::Context;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub trait Graphic {
    fn render(&self, context: &impl Context) -> Result<(), JsValue>;
}

pub struct Candlestick {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: String,
}

impl Candlestick {
    pub fn new(x: u32, y: u32, width: u32, height: u32, color: String) -> Candlestick {
        Candlestick {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Graphic for Candlestick {
    fn render(&self, context: &impl Context) -> Result<(), JsValue> {
        context.draw_rect((self.x, self.y), (self.width, self.height), &self.color)
    }
}
