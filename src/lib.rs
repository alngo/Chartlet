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
    let _controller = controller::Controller::new(&model);

    Ok(())
}
