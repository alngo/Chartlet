use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod controller;
pub mod model;
pub mod view;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
struct Bridge {
    root: Element,
}

impl Bridge {
    fn new(id: String) -> Bridge {
        let document = window().unwrap().document().unwrap();
        let root = document.get_element_by_id(&id).unwrap();
        Bridge { root }
    }

    fn render(&self, html: &str) {
        self.root.set_inner_html(html);
    }
}

#[wasm_bindgen]
struct Chartlet {
    bridge: Option<Bridge>,
}

#[wasm_bindgen]
impl Chartlet {
    pub fn new() -> Chartlet {
        Chartlet { bridge: None }
    }

    pub fn bridge(&mut self, bridge: Bridge) {
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
