use wasm_bindgen::prelude::*;
use web_sys::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

trait Drawable {
    fn draw(&self);
}

trait Interactable {
    fn interact(&self);
}

struct Glyph;

#[wasm_bindgen]
pub struct App {
    root: HtmlElement,
}

#[wasm_bindgen]
impl App {
    pub fn new(root: HtmlElement) -> App {
        App {
            root
        }
    }

    fn init(&self) {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();
    }

    pub fn run(&self) -> Result<(), JsValue> {
        self.init();

        console::log_1(&JsValue::from_str("App is running!"));

        Ok(())
    }
}
