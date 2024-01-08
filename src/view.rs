use std::collections::HashMap;

use crate::model;

mod builder;

use builder::Builder;

#[derive(Debug, Clone)]
pub struct ViewError;

impl std::fmt::Display for ViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ViewError")
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum Layer {
    Grid,
    Label,
    Data,
    Indicators,
    Objects,
    Orders,
}

#[derive(Default)]
pub struct View {
    layers: HashMap<Layer, String>,
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

    pub fn render(&mut self, model: &model::Model) {
        // get width and height from model
        let builder = Builder::new(100, 100);
        let grid = builder.build_grid();
        self.layers.insert(Layer::Grid, grid);
    }

    pub fn clear(&self) {}
}
