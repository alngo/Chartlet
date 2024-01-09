use crate::model;

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
pub struct View {}

impl View {
    pub fn new() -> View {
        View {}
    }

    pub fn update(&mut self, model: &model::Model) {
        self.clear();
        self.render(model);
    }

    pub fn clear(&mut self) {}
    pub fn render(&mut self, _model: &model::Model) {}
}
