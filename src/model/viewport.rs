#[derive(Default, Copy, Clone, Debug)]
pub struct Viewport {
    pub width: usize,
    pub height: usize,
}

impl Viewport {
    pub fn new(width: usize, height: usize) -> Viewport {
        Viewport { width, height }
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }
}

#[cfg(test)]
mod viewport_tests {
    use super::*;

    #[test]
    fn test_default_viewport() {
        let viewport = Viewport::default();
        assert_eq!(viewport.width, 0);
        assert_eq!(viewport.height, 0);
    }

    #[test]
    fn test_new_viewport() {
        let viewport = Viewport::new(4, 2);
        assert_eq!(viewport.width, 4);
        assert_eq!(viewport.height, 2);
    }

    #[test]
    fn test_viewport() {
        let mut viewport = Viewport::default();
        viewport.set_width(1);
        viewport.set_height(2);
        assert_eq!(viewport.width, 1);
        assert_eq!(viewport.height, 2);
    }
}
