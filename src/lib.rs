use wasm_bindgen::prelude::*;
use web_sys::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Coordinate = (u32, u32);

struct SvgContext {
    svg: SvgElement,
}

impl SvgContext {
    fn new(width: u32, height: u32) -> Result<SvgContext, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let svg = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?
            .dyn_into::<SvgElement>()?;
        svg.set_attribute("width", &width.to_string())?;
        svg.set_attribute("height", &height.to_string())?;
        svg.set_attribute("viewBox", &format!("0 0 {} {}", width, height))?;
        Ok(SvgContext { svg })
    }

    fn draw_pixel(&self, coord: Coordinate, color: &str) -> Result<(), JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let rect = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
            .dyn_into::<SvgRectElement>()?;
        rect.set_attribute("x", &coord.0.to_string())?;
        rect.set_attribute("y", &coord.1.to_string())?;
        rect.set_attribute("width", "1")?;
        rect.set_attribute("height", "1")?;
        rect.set_attribute("fill", color)?;
        self.svg.append_child(&rect)?;
        Ok(())
    }
}

struct CanvasContext {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl CanvasContext {
    fn new(width: u32, height: u32) -> Result<CanvasContext, JsValue> {
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
        Ok(CanvasContext { canvas, context })
    }

    fn draw_pixel(&self, coord: Coordinate, color: &str) -> Result<(), JsValue> {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context
            .fill_rect(coord.0 as f64, coord.1 as f64, 1.0, 1.0);
        Ok(())
    }
}

#[wasm_bindgen]
pub struct App {
    root: HtmlElement,
}

#[wasm_bindgen]
impl App {
    pub fn new(root: HtmlElement) -> App {
        App { root }
    }

    fn init(&self) {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();
    }

    pub fn run(&self) -> Result<(), JsValue> {
        self.init();
        let svg_context = SvgContext::new(100, 100)?;
        svg_context.draw_pixel((50, 50), "red")?;
        self.root.append_child(&svg_context.svg)?;

        let canvas_context = CanvasContext::new(100, 100)?;
        canvas_context.draw_pixel((50, 50), "blue")?;
        self.root.append_child(&canvas_context.canvas)?;

        console::log_1(&JsValue::from_str("App is running!"));

        Ok(())
    }
}
