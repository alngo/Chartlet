use crate::model;

mod json;

#[derive(Debug, Clone)]
pub struct ViewError;

impl std::fmt::Display for ViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ViewError")
    }
}

pub struct View;

impl View {
    pub fn new() -> View {
        View
    }

    pub fn render(&self, _model: &model::Model) {}
    pub fn update(&self, _model: &model::Model) {}
    pub fn clear(&self) {}
}
