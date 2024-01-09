use super::context;

mod line;

pub trait Draw {
    fn draw(&self, context: &dyn context::Context);
}

pub trait Composable {
    fn insert(&mut self, component: &impl Draw);
    fn remove(&mut self, component: &impl Draw);
}
