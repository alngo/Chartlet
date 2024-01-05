use std::{cell::RefCell, rc::Rc};

use crate::model::frame::Frame;

pub enum FrameControllerMessage {
    SetAuto(bool),
    SetWidth(f64),
    SetHeight(f64),
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
            FrameControllerMessage::SetWidth(width) => self.set_width(width),
            FrameControllerMessage::SetHeight(height) => self.set_height(height),
            FrameControllerMessage::SetOffsetX(x) => self.set_offset_x(x),
            FrameControllerMessage::SetOffsetY(y) => self.set_offset_y(y),
        }
    }

    fn set_auto(&mut self, auto: bool) {
        self.frame.borrow_mut().set_auto(auto);
    }

    fn set_width(&mut self, width: f64) {
        self.frame.borrow_mut().set_width(width);
    }

    fn set_height(&mut self, height: f64) {
        self.frame.borrow_mut().set_height(height);
    }

    fn set_offset_x(&mut self, x: f64) {
        self.frame.borrow_mut().set_offset_x(x);
    }

    fn set_offset_y(&mut self, y: f64) {
        self.frame.borrow_mut().set_offset_y(y);
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
        controller.call(FrameControllerMessage::SetWidth(1.0));
        controller.call(FrameControllerMessage::SetHeight(2.0));
        controller.call(FrameControllerMessage::SetOffsetX(3.0));
        controller.call(FrameControllerMessage::SetOffsetY(4.0));
        assert_eq!(model.frame.borrow().width, 1.0);
        assert_eq!(model.frame.borrow().height, 2.0);
        assert_eq!(model.frame.borrow().offset_x, 3.0);
        assert_eq!(model.frame.borrow().offset_y, 4.0);
    }
}
