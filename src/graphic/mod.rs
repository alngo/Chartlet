// pub mod chart {
//     pub use crate::chart::point::Point;
//     pub use crate::chart::scale::Scale;
//     pub use crate::chart::axis::Axis;
//     pub use crate::chart::grid::Grid;
//     pub use crate::chart::legend::Legend;
//     pub use crate::chart::series::Series;
//     pub use crate::chart::chart::Chart;
//
//     pub mod point;
//     pub mod scale;
//     pub mod axis;
//     pub mod grid;
//     pub mod legend;
//     pub mod series;
//     pub mod chart;
// }
//

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

    pub mod circle;
    pub mod rectangle;
    pub mod shape;
    pub mod timeline;

    pub trait Draw {
        fn draw(&self, context: &dyn context::Context);
    }

    pub trait Composable {
        fn insert(&mut self, component: &impl Draw);
        fn remove(&mut self, component: &impl Draw);
    }
}
