use crate::model;

mod json;

#[derive(Debug, Clone)]
pub struct ViewError;

impl std::fmt::Display for ViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ViewError")
    }
}

pub trait View<T> {
    fn render(&self, model: &model::Model) -> Result<T, ViewError>;
}
