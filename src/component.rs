use crate::context::Context;

pub trait Draw {
    fn draw(&self);
}

pub struct Layer<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Layer<T> where T: Draw {}

// old
pub trait Component {
    fn draw(&self, context: &impl Context);
}

pub trait Composable {
    fn insert(&mut self, component: impl Component + 'static);
    fn remove(&mut self, component: impl Component + 'static);
}
