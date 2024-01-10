use super::context;

pub mod line;

pub trait Drawable {
    fn draw(&self, context: &dyn context::Context);
}

pub trait Composable {
    fn insert(&mut self, component: &impl Drawable);
    fn remove(&mut self, component: &impl Drawable);
}
