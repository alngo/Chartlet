use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn run(width: u32, height: u32) -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.

    // Outer SVG
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let chartlet = document.get_element_by_id("chartlet").unwrap();

    let svg = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
        .unwrap()
        .dyn_into::<web_sys::SvgElement>()
        .map_err(|_| ())
        .unwrap();

    svg.style().set_property("border", "solid")?;

    svg.set_attribute("width", "500")?;
    svg.set_attribute("height", "500")?;
    svg.set_attribute("viewBox", "0 0 500 500")?;

    let circle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    circle.set_attribute("cx", "100")?;
    circle.set_attribute("cy", "100")?;
    circle.set_attribute("r", "60")?;
    circle.set_attribute("stroke", "black")?;
    circle.set_attribute("fill", "blue")?;
    svg.append_child(&circle)?;

    chartlet.append_child(&svg)?;

    Ok(())
}
