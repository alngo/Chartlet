use wasm_bindgen::prelude::*;
use web_sys::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Should have a better name
pub mod chart;
pub mod graphic;
pub mod history;

pub use graphic::Renderer;

fn set_panic_hook() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}
