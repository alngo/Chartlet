use crate::context::Context;

pub trait Component {
    fn draw(&self, context: &impl Context);
}

pub trait Composable {
    fn insert(&mut self, component: impl Component + 'static);
    fn remove(&mut self, component: impl Component + 'static);
}
