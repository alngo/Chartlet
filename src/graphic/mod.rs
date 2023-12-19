pub mod context {
    use crate::chart::point::Point;
    use wasm_bindgen::prelude::JsValue;

    pub mod svg;

    pub trait Context {
        fn draw_pixel(&self, point: Point, color: &str) -> Result<(), JsValue>;
        fn draw_line(&self, start: Point, end: Point, color: &str) -> Result<(), JsValue>;
        fn draw_rect(&self, start: Point, end: Point, color: &str) -> Result<(), JsValue>;
        fn draw_circle(&self, center: Point, radius: f32, color: &str) -> Result<(), JsValue>;
        fn draw_text(&self, position: Point, text: &str, color: &str) -> Result<(), JsValue>;
    }
}

pub mod composite {

}
