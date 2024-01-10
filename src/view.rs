mod builder;
mod composite;
mod context;
mod converter;

use crate::model;
use converter::Converter;
use std::collections::HashMap;

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
        let min_y = model.frame.borrow().min_y;
        let max_y = model.frame.borrow().max_y;

        let converter = Converter::new(width, height, min_x, min_y, max_x, max_y);
        let builder = builder::Builder::new(converter);

        self.layers
            .insert(Layer::Grid, builder.build_grid(min_x, max_x, min_y, max_y));
    }
}
