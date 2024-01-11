use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod controller;
pub mod model;
pub mod view;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
struct Chartlet {
    root: HtmlElement,
}





#[wasm_bindgen]
impl Chartlet {
    pub fn new(root: String) -> Chartlet {
        let document = web_sys::window().unwrap().document().unwrap();
        let root = document
            .get_element_by_id(&root)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        Chartlet { root }
    }

    pub fn run(&self) -> Result<(), JsValue> {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        let model = model::Model::new();
        let view = view::View::new();

        let width = self.root.client_width();
        let height = self.root.client_height();

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
