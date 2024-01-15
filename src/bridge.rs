use std::collections::HashMap;

use crate::view::Layer;
use wasm_bindgen::prelude::*;
use web_sys::*;

use super::view::Drawables;

pub trait Bridge {
    fn call(&mut self, message: String);
    fn update(&mut self, layers: &HashMap<Layer, Drawables>);
    fn clear(&mut self);
    fn render(&mut self, layers: &HashMap<Layer, Drawables>);
}

#[wasm_bindgen]
pub struct HtmlBridge {
    root: Element,
}

#[wasm_bindgen]
impl HtmlBridge {
    pub fn new(id: String) -> HtmlBridge {
        let document = window().unwrap().document().unwrap();
        let root = document.get_element_by_id(&id).unwrap();
        HtmlBridge { root }
    }
}

impl Bridge for HtmlBridge {
    // message should be mapped to ControllerMessage
    // Serialize, Deserialize?
    fn call(&mut self, _message: String) {
        todo!()
    }

    fn update(&mut self, _layers: &HashMap<Layer, Drawables>) {
        todo!()
    }

    fn clear(&mut self) {
        let first_child = self.root.first_child();
        if first_child.is_some() {
            let _ = self.root.remove_child(&first_child.unwrap());
        }
    }

    fn render(&mut self, _layers: &HashMap<Layer, Drawables>) {
        todo!()
    }
}
