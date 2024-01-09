use std::collections::HashMap;

use crate::model;

mod composite;
mod context;

#[derive(Hash, PartialEq, Eq)]
pub enum Layer {
    Grid,
    Label,
    Data,
    Indicators,
    Objects,
    Orders,
}

pub struct Point(pub f64, pub f64);

#[derive(Default)]
pub struct View {
    layers: HashMap<Layer, Box<dyn composite::Draw>>,
}

impl View {
    pub fn new() -> View {
        View {
            layers: HashMap::new(),
        }
    }

    pub fn update(&mut self, model: &model::Model) {
        self.clear();
        self.render(model);
    }

    pub fn clear(&mut self) {
        self.layers.clear();
    }

    pub fn render(&mut self, model: &model::Model) {
        let width = model.viewport.borrow().width;
        let height = model.viewport.borrow().height;
    }
}
