use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod builder;
pub mod chart;
pub mod graphic;

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
}

#[derive(Default, Clone, Serialize, Deserialize)]
struct Layer {
    pub name: Box<str>,
}

#[derive(Default, Serialize, Deserialize)]
struct Layers {
    pub layers: HashMap<String, Layer>,
}

impl Layers {
    pub fn default() -> Self {
        let mut layers = HashMap::new();
        layers.insert(
            "layer1".to_string(),
            Layer {
                name: "layer1".into(),
            },
        );
        layers.insert(
            "layer2".to_string(),
            Layer {
                name: "layer2".into(),
            },
        );
        let svg = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
            .unwrap();
        svg.set_attribute("width", "100%").unwrap();
        svg.set_attribute("height", "100%").unwrap();
        svg.set_attribute("viewBox", "0 0 100 100").unwrap();
        layers.insert(
            "layer3".to_string(),
            Layer {
                name: svg.outer_html().into(),
            },
        );
        Self { layers }
    }
}

#[wasm_bindgen]
pub fn load_layers() -> JsValue {
    let layers = Layers::default();
    serde_wasm_bindgen::to_value(&layers).unwrap()
}

fn set_panic_hook() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}
