use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod controller;
pub mod model;
pub mod view;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let model = model::Model::new();
    let view = view::View::new();

    let _controller = {
        let mut controller = controller::Controller::new(model, view);
        let messages = vec![
            controller::ControllerMessage::ViewportController(
                controller::ViewportControllerMessage::SetWidth(100),
            ),
            controller::ControllerMessage::ViewportController(
                controller::ViewportControllerMessage::SetHeight(100),
            ),
        ];

        for message in messages {
            controller.call(message);
        }
        controller
    };

    Ok(())
}
