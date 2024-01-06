use std::{cell::RefCell, rc::Rc};

use crate::model::frame::Frame;

pub enum FrameControllerMessage {
    SetAuto(bool),
    SetMinX(f64),
    SetMinY(f64),
    SetOffsetX(f64),
    SetOffsetY(f64),
}

pub(crate) struct FrameController {
    frame: Rc<RefCell<Frame>>,
}

impl FrameController {
    pub fn new(frame: Rc<RefCell<Frame>>) -> FrameController {
        FrameController { frame }
    }

    pub fn call(&mut self, message: FrameControllerMessage) {
        match message {
            FrameControllerMessage::SetAuto(auto) => self.set_auto(auto),
            FrameControllerMessage::SetMinX(min_x) => self.set_width(min_x),
            FrameControllerMessage::SetMinY(min_y) => self.set_height(min_y),
            FrameControllerMessage::SetOffsetX(x) => self.set_max_x(x),
            FrameControllerMessage::SetOffsetY(y) => self.set_max_y(y),
        }
    }

    fn set_auto(&mut self, auto: bool) {
        self.frame.borrow_mut().set_auto(auto);
    }

    fn set_width(&mut self, min_x: f64) {
        self.frame.borrow_mut().set_min_x(min_x);
    }

    fn set_height(&mut self, min_y: f64) {
        self.frame.borrow_mut().set_min_y(min_y);
    }

    fn set_max_x(&mut self, x: f64) {
        self.frame.borrow_mut().set_max_x(x);
    }

    fn set_max_y(&mut self, y: f64) {
        self.frame.borrow_mut().set_max_y(y);
    }
}

#[cfg(test)]
mod frame_controller_tests {
    use super::*;
    use crate::model::Model;

    #[test]
    fn test_frame_controller() {
        let model = Model::new();
        let mut controller = FrameController::new(model.frame.clone());
        controller.call(FrameControllerMessage::SetAuto(true));
        controller.call(FrameControllerMessage::SetMinX(1.0));
        controller.call(FrameControllerMessage::SetMinY(2.0));
        controller.call(FrameControllerMessage::SetOffsetX(3.0));
        controller.call(FrameControllerMessage::SetOffsetY(4.0));
        assert_eq!(model.frame.borrow().auto, true);
        assert_eq!(model.frame.borrow().min_x, 1.0);
        assert_eq!(model.frame.borrow().min_y, 2.0);
        assert_eq!(model.frame.borrow().max_x, 3.0);
        assert_eq!(model.frame.borrow().max_y, 4.0);
    }
}
