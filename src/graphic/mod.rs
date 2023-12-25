pub mod context {
    pub use crate::chart::point::Point;
    pub use wasm_bindgen::prelude::JsValue;

    pub mod svg;

    pub trait Context {
        fn draw_pixel(&self, point: Point, color: &str) -> Result<(), JsValue>;
        fn draw_line(&self, start: Point, end: Point, color: &str) -> Result<(), JsValue>;
        fn draw_rect(
            &self,
            start: Point,
            width: &str,
            height: &str,
            color: &str,
        ) -> Result<(), JsValue>;
        fn draw_circle(&self, center: Point, radius: f32, color: &str) -> Result<(), JsValue>;
        fn draw_text(&self, position: Point, text: &str, color: &str) -> Result<(), JsValue>;
    }
}

pub mod composite {
    pub use crate::chart::point::Point;
    pub use crate::graphic::context;

    pub mod timeline;

    pub trait Draw {
        fn draw(&self, context: &dyn context::Context);
    }

    pub trait Composable {
        fn insert(&mut self, component: &impl Draw);
        fn remove(&mut self, component: &impl Draw);
    }
    // Draft Draw a timeline composed by a vertical line and an optional text
    // <g> for grouping
    //  <line>...
    //  <text>...
    // </g>
}
