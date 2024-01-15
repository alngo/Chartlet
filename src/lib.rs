use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod bridge;
pub mod controller;
pub mod model;
pub mod view;

pub use bridge::HtmlBridge;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
struct Chartlet {
    // Not very good, should have Option<Bridge>, the interface instead of the implementation
    // TODO: Option<Interface>
    bridge: Option<HtmlBridge>,
}

#[wasm_bindgen]
impl Chartlet {
    pub fn new() -> Chartlet {
        Chartlet { bridge: None }
    }

    pub fn connect_to(&mut self, bridge: HtmlBridge) {
        self.bridge = Some(bridge);
    }

    pub fn run(&self) -> Result<(), JsValue> {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        let model = model::Model::new();
        let view = view::View::new();

        let width = 100;
        let height = 100;

        let _controller = {
            let mut controller = controller::Controller::new(model, view);
            let messages = vec![
                controller::ControllerMessage::ViewportController(
                    controller::ViewportControllerMessage::SetWidth(width as usize),
                ),
                controller::ControllerMessage::ViewportController(
                    controller::ViewportControllerMessage::SetHeight(height as usize),
                ),
            ];
            for message in messages {
                controller.call(message);
            }
            controller
        };
        Ok(())
    }
}
