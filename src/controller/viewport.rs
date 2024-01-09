use std::{cell::RefCell, rc::Rc};

use crate::model::viewport::Viewport;

pub enum ViewportControllerMessage {
    SetWidth(usize),
    SetHeight(usize),
}

#[derive(Default)]
pub(crate) struct ViewportController {
    viewport: Rc<RefCell<Viewport>>,
}

impl ViewportController {
    pub fn new(viewport: Rc<RefCell<Viewport>>) -> ViewportController {
        ViewportController { viewport }
    }

    pub fn call(&mut self, message: ViewportControllerMessage) {
        match message {
            ViewportControllerMessage::SetWidth(width) => {
                self.set_width(width);
            }
            ViewportControllerMessage::SetHeight(height) => {
                self.set_height(height);
            }
        }
    }

    fn set_width(&self, width: usize) {
        self.viewport.borrow_mut().set_width(width);
    }

    fn set_height(&self, height: usize) {
        self.viewport.borrow_mut().set_height(height);
    }
}

#[cfg(test)]
mod viewport_controller_tests {
    use super::*;

    #[test]
    fn test_viewport_controller() {
        let viewport = Rc::new(RefCell::new(Viewport::default()));
        let mut viewport_controller = ViewportController::new(viewport);
        viewport_controller.call(ViewportControllerMessage::SetWidth(1));
        viewport_controller.call(ViewportControllerMessage::SetHeight(2));
        assert_eq!(viewport_controller.viewport.borrow().width, 1);
        assert_eq!(viewport_controller.viewport.borrow().height, 2);
    }
}
