use std::collections::HashMap;

use crate::model;

mod builder;
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(pub f64, pub f64);

pub type Drawables = Vec<Box<dyn composite::Drawable>>;

#[derive(Default)]
pub struct View {
    layers: HashMap<Layer, Drawables>,
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

        let min_x = model.frame.borrow().min_x;
        let max_x = model.frame.borrow().max_x;

        let builder = builder::Builder::new(width, height);
        let grid = builder.build_grid(min_x, max_x);
        self.layers.insert(Layer::Grid, grid);
    }
}
