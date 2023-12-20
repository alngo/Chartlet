use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}
