use crate::context::{Context, Coordinate};
use wasm_bindgen::prelude::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct CanvasRenderingContext {
    pub canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl CanvasRenderingContext {
    pub fn new(width: u32, height: u32) -> Result<CanvasRenderingContext, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        canvas.set_width(width);
        canvas.set_height(height);
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        Ok(CanvasRenderingContext { canvas, context })
    }
}

impl Context for CanvasRenderingContext {
    fn draw_pixel(&self, coord: Coordinate, color: &str) -> Result<(), JsValue> {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context
            .fill_rect(coord.0 as f64, coord.1 as f64, 1.0, 1.0);
        Ok(())
    }

    fn draw_line(&self, start: Coordinate, end: Coordinate, color: &str) -> Result<(), JsValue> {
        self.context.set_stroke_style(&JsValue::from_str(color));
        self.context.begin_path();
        self.context.move_to(start.0 as f64, start.1 as f64);
        self.context.line_to(end.0 as f64, end.1 as f64);
        self.context.stroke();
        Ok(())
    }

    fn draw_rect(&self, start: Coordinate, width: u32, height: u32, color: &str) -> Result<(), JsValue> {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.fill_rect(start.0 as f64, start.1 as f64, width as f64, height as f64);
        Ok(())
    }

    fn draw_circle(&self, center: Coordinate, radius: u32, color: &str) -> Result<(), JsValue> {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.begin_path();
        self.context.arc(
            center.0 as f64,
            center.1 as f64,
            radius as f64,
            0.0,
            2.0 * std::f64::consts::PI,
        )?;
        self.context.fill();
        Ok(())
    }
}
