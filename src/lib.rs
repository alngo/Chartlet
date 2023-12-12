use wasm_bindgen::prelude::*;
use web_sys::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod context;
use context::canvas_context::CanvasContext;
use context::svg_context::SvgContext;
use context::Context;

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
        // svg_context.draw_pixel((50, 50), "red")?;
        // svg_context.draw_line((0, 0), (100, 100), "red")?;
        // svg_context.draw_rect((0, 0), (100, 100), "red")?;
        svg_context.draw_circle((50, 50), 50, "red")?;
        self.root.append_child(&svg_context.svg)?;

        let canvas_context = CanvasContext::new(100, 100)?;
        // canvas_context.draw_pixel((50, 50), "blue")?;
        // canvas_context.draw_line((0, 0), (100, 100), "blue")?;
        // canvas_context.draw_rect((0, 0), (100, 100), "blue")?;
        canvas_context.draw_circle((50, 50), 50, "blue")?;
        self.root.append_child(&canvas_context.canvas)?;

        console::log_1(&JsValue::from_str("App is running!"));

        Ok(())
    }
}
