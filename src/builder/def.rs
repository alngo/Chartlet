use serde::{Deserialize, Serialize};

struct Layer {
    name: String,
    data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefBuilder {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
}

#[wasm_bindgen]
impl DefBuilder {
    pub fn get_layers(&self) -> JsValue {
        JsValue::from_serde(&self.layers).unwrap()
    }
}
